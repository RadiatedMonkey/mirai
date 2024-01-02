use util::SharedBuffer;
use util::pyassert;
use util::{BinaryRead, Deserialize};

/// Confirms that the connection was successfully initiated.
#[derive(Debug)]
pub struct NewIncomingConnection;

impl NewIncomingConnection {
    /// Unique ID of this packet.
    pub const ID: u8 = 0x13;
}

impl Deserialize<'_> for NewIncomingConnection {
    fn deserialize(mut buffer: SharedBuffer) -> anyhow::Result<Self> {
        pyassert!(buffer.read_u8()? == Self::ID);

        // No data in this packet is used, there is no point in decoding it
        Ok(Self)
    }
}
