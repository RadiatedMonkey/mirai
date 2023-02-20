use bytes::Bytes;
use bytes::{BufMut, BytesMut};
use common::size_of_var;

use crate::network::packets::ConnectedPacket;
use common::Serialize;
use common::VResult;
use common::WriteExtensions;

/// Sent by the server to initiate encryption.
/// The client responds with a [`ClientToServerHandshake`](super::ClientToServerHandshake) to
/// indicate encryption has successfully been initiated.
#[derive(Debug, Clone)]
pub struct ServerToClientHandshake<'a> {
    /// Token containing the salt and public key.
    pub jwt: &'a str,
}

impl ConnectedPacket for ServerToClientHandshake<'_> {
    const ID: u32 = 0x03;

    fn serialized_size(&self) -> usize {
        size_of_var(self.jwt.len() as u32) + self.jwt.len()
    }
}

impl Serialize for ServerToClientHandshake<'_> {
    fn serialize(&self, buffer: &mut BytesMut) {
        buffer.put_string(self.jwt);
    }
}
