
use util::{Serialize, Result};
use util::bytes::{BinaryWriter, MutableBuffer};
use crate::network::cache_blob::CacheBlob;
use crate::network::packets::ConnectedPacket;

#[derive(Debug, Clone)]
pub struct CacheMissResponse<'a> {
    pub blobs: &'a [CacheBlob<'a>]
}

impl ConnectedPacket for CacheMissResponse<'_> {
    const ID: u32 = 0x88;

    fn serialized_size(&self) -> usize {
        1 + self.blobs.iter().fold(0, |acc, blob| acc + blob.len())
    }
}

impl Serialize for CacheMissResponse<'_> {
    fn serialize(&self, buffer: &mut MutableBuffer) {
        buffer.write_var_u32(self.blobs.len() as u32);
        for blob in self.blobs {
            blob.serialize(buffer);
        }
    }
}