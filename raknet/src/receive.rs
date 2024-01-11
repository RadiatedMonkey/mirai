use std::io::Read;
use std::sync::atomic::Ordering;
use std::time::{Instant, Duration};

use async_recursion::async_recursion;
use proto::bedrock::{Animate, CacheStatus, ChunkRadiusRequest, ClientToServerHandshake, CommandRequest, CompressionAlgorithm, CONNECTED_PACKET_ID, ConnectedPacket, ContainerClose, FormResponse, Header, Interact, Login, MovePlayer, PlayerAction, RequestAbility, RequestNetworkSettings, ResourcePackClientResponse, SetLocalPlayerAsInitialized, SettingsCommand, TextMessage, TickSync, UpdateSkin, ViolationWarning};
use proto::raknet::{Ack, ConnectedPing, ConnectionRequest, DisconnectNotification, Nak, NewIncomingConnection};

use tokio::sync::mpsc::error::SendTimeoutError;
use util::{BinaryRead, MutableBuffer};

use crate::{RaknetUser, FrameBatch, Frame, BroadcastPacket};

const RAKNET_OUTPUT_TIMEOUT: Duration = Duration::from_millis(10);

impl RaknetUser {
    /// Processes the raw packet coming directly from the network.
    ///
    /// If a packet is an ACK or NACK type, it will be responded to accordingly (using [`Session::process_ack`] and [`Session::process_nak`]).
    /// Frame batches are processed by [`Session::process_frame_batch`].
    pub async fn handle_raw_packet(&self, packet: MutableBuffer) -> anyhow::Result<bool> {
        *self.last_update.write() = Instant::now();

        if packet.is_empty() {
            anyhow::bail!("Packet is empty");
        }

        let pk_id = *packet.first().unwrap();
        match pk_id {
            Ack::ID => self.handle_ack(packet.snapshot())?,
            Nak::ID => self.handle_nack(packet.snapshot()).await?,
            _ => self.handle_frame_batch(packet).await?,
        }

        Ok(true)
    }

    // /// Processes a broadcasted packet sent by another client connected to the server.
    // pub fn handle_broadcast(&self, packet: BroadcastPacket) -> anyhow::Result<()> {
    //     if let Some(sender) = packet.sender {
    //         if sender == self.address {
    //             // Source is self, do not send.
    //             return Ok(());
    //         }
    //     }

    //     self.send_serialized(packet.content, DEFAULT_SEND_CONFIG)
    // }

    /// Processes a batch of frames.
    ///
    /// This performs the actions required by the Raknet reliability layer, such as
    /// * Inserting raknet into the order channels
    /// * Inserting raknet into the compound collector
    /// * Discarding old sequenced frames
    /// * Acknowledging reliable raknet
    async fn handle_frame_batch(&self, packet: MutableBuffer) -> anyhow::Result<()> {
        let batch = FrameBatch::deserialize(packet.snapshot())?;
        self
            .batch_number
            .fetch_max(batch.sequence_number, Ordering::SeqCst);

        for frame in batch.frames {
            self.handle_frame(frame, batch.sequence_number).await?;
        }

        Ok(())
    }

    #[async_recursion]
    async fn handle_frame(
        &self,
        frame: Frame,
        batch_number: u32,
    ) -> anyhow::Result<()> {
        if frame.reliability.is_sequenced()
            && frame.sequence_index
            < self.batch_number.load(Ordering::SeqCst)
        {
            // Discard packet
            return Ok(());
        }

        if frame.reliability.is_reliable() {
            // Confirm packet
            let mut lock = self.acknowledged.lock();
            lock.push(batch_number);
        }

        if frame.is_compound {
            let possible_frag = self.compounds.insert(frame)?;

            return if let Some(packet) = possible_frag {
                self.handle_frame(packet, batch_number).await
            } else {
                // Compound incomplete
                Ok(())
            };
        }

        // Sequenced implies ordered
        if frame.reliability.is_ordered() || frame.reliability.is_sequenced() {
            // Add packet to order queue
            if let Some(ready) = self.order[frame.order_channel as usize]
                .insert(frame)
            {
                for packet in ready {
                    self.handle_frame_body(packet.body).await?;
                }
            }
            return Ok(());
        }

        self.handle_frame_body(frame.body).await
    }

    /// Processes an unencapsulated game packet.
    async fn handle_frame_body(&self, packet: MutableBuffer) -> anyhow::Result<()> {
        let packet_id = *packet.first().expect("Game packet buffer was empty");
        match packet_id {
            // CONNECTED_PACKET_ID => self.handle_encrypted_frame(packet).await?,
            CONNECTED_PACKET_ID => {
                if let Err(err) = self.output.send_timeout(packet, RAKNET_OUTPUT_TIMEOUT).await {
                    if matches!(err, SendTimeoutError::Closed(_)) {
                        // Output channel has been closed.
                        let _ = self.disconnect().await;
                        tracing::error!("RakNet layer output channel closed, disconnecting client");
                    }   
                }
            },
            DisconnectNotification::ID => self.handle_disconnect(),
            ConnectionRequest::ID => self.handle_connection_request(packet)?,
            NewIncomingConnection::ID => {
                self.handle_new_incoming_connection(packet)?
            }
            ConnectedPing::ID => self.handle_connected_ping(packet)?,
            id => anyhow::bail!("Invalid Raknet packet ID: {}", id),
        }

        Ok(())
    }
}