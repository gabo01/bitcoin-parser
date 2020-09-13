#[cfg(feature = "writer")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct BitcoinScript {
    contents: Vec<u8>,
}

impl BitcoinScript {
    pub fn new(contents: Vec<u8>) -> Self {
        Self { contents }
    }
}

#[cfg(feature = "writer")]
impl Serialize for BitcoinScript {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        hex::encode(&self.contents).serialize(serializer)
    }
}

#[cfg(feature = "writer")]
impl<'de> Deserialize<'de> for BitcoinScript {
    fn deserialize<D: Deserializer<'de>>(_deserializer: D) -> Result<Self, D::Error> {
        todo!()
    }
}
