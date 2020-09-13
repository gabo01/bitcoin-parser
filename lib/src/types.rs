use arrayref::array_ref;
use byteorder::ByteOrder;
use core::ops::Deref;
use sha2::Digest;
use sha2::Sha256;

#[derive(Copy, Clone)]
pub struct VarInt(u64);

impl VarInt {
    pub fn new(number: u64) -> Self {
        Self(number)
    }

    pub fn from_2_bytes<O: ByteOrder>(data: &[u8; 2]) -> Self {
        Self(O::read_u16(data) as u64)
    }

    pub fn from_4_bytes<O: ByteOrder>(data: &[u8; 4]) -> Self {
        Self(O::read_u32(data) as u64)
    }

    pub fn from_8_bytes<O: ByteOrder>(data: &[u8; 8]) -> Self {
        Self(O::read_u64(data))
    }
}

impl Deref for VarInt {
    type Target = u64;

    fn deref(&self) -> &u64 {
        &self.0
    }
}

pub struct BlockTarget([u8; 4]);

impl BlockTarget {
    pub fn new(data: [u8; 4]) -> Self {
        Self(data)
    }
}

impl<'a> From<&'a [u8]> for BlockTarget {
    fn from(slice: &'a [u8]) -> Self {
        Self::new(array_ref!(slice, 0, 4).to_owned())
    }
}

pub struct BitcoinHash([u8; 32]);

impl BitcoinHash {
    pub fn new(data: [u8; 32]) -> Self {
        Self(data)
    }

    pub fn hash(digest: &[u8]) -> Self {
        let mut hash = Sha256::digest(digest);
        hash.reverse();
        Self::new(array_ref!(&hash[..], 0, 32).to_owned())
    }
}
