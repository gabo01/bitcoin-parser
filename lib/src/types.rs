use arrayref::array_ref;
use byteorder::ByteOrder;
use core::ops::Deref;
#[cfg(feature = "writer")]
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use sha2::Digest;
use sha2::Sha256;
use std::convert::TryFrom;

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

#[cfg(feature = "writer")]
impl Serialize for VarInt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "writer")]
impl<'de> Deserialize<'de> for VarInt {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = <u64 as Deserialize<'de>>::deserialize(deserializer)?;
        Ok(Self::new(data))
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

#[cfg(feature = "writer")]
impl Serialize for BlockTarget {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        hex::encode(self.0).serialize(serializer)
    }
}

#[cfg(feature = "writer")]
impl<'de> Deserialize<'de> for BlockTarget {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let hex_string = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let bytes = hex::decode(hex_string).map_err(de::Error::custom)?;
        let data = <&[u8; 4]>::try_from(&bytes[..]).map_err(de::Error::custom)?;
        Ok(Self::new(data.to_owned()))
    }
}

pub struct BitcoinHash([u8; 32]);

impl BitcoinHash {
    pub fn new(data: [u8; 32]) -> Self {
        Self(data)
    }

    pub fn from_little_endian(mut data: [u8; 32]) -> Self {
        data.reverse();
        Self(data)
    }

    pub fn hash_header(digest: &[u8]) -> Self {
        let mut hash = Sha256::digest(&Sha256::digest(digest)[..]);
        hash.reverse();
        Self::new(array_ref!(&hash[..], 0, 32).to_owned())
    }
}

#[cfg(feature = "writer")]
impl Serialize for BitcoinHash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        hex::encode(self.0).serialize(serializer)
    }
}

#[cfg(feature = "writer")]
impl<'de> Deserialize<'de> for BitcoinHash {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let hex_string = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let bytes = hex::decode(hex_string).map_err(de::Error::custom)?;
        let data = <&[u8; 32]>::try_from(&bytes[..]).map_err(de::Error::custom)?;
        Ok(Self::new(data.to_owned()))
    }
}

pub struct BitcoinHashBuilder {
    contents: Vec<u8>,
}

impl BitcoinHashBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_digest(&mut self, data: &[u8]) {
        self.contents.extend(data)
    }

    pub fn into_hash(self) -> BitcoinHash {
        BitcoinHash::hash_header(&self.contents)
    }
}

impl Default for BitcoinHashBuilder {
    fn default() -> Self {
        Self {
            contents: Vec::with_capacity(768),
        }
    }
}
