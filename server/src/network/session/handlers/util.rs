use bytes::BytesMut;
use common::{VResult, Vector3i};

use crate::network::{session::Session, packets::{TextMessage, SetTime, MessageType, PlaySound, ShowProfile}, Decodable};

impl Session {
    pub fn handle_text_message(&self, packet: BytesMut) -> VResult<()> {
        let request = TextMessage::decode(packet)?;
        tracing::info!("{request:?}");
        
        let reply = ShowProfile {
            xuid: request.xuid.clone()
        };
        self.send_packet(reply)?;

        // let reply = TextMessage {
        //     message_type: MessageType::System,
        //     needs_translation: false,
        //     source_name: "Server".to_owned(),
        //     message: "Popup".to_owned(),
        //     parameters: Vec::new(),
        //     xuid: request.xuid,
        //     platform_chat_id: request.platform_chat_id,
        // };
        // self.send_packet(reply)?;

        Ok(())
    }
}