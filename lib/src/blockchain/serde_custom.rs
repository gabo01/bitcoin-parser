pub mod version {
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
    use std::convert::TryFrom;

    pub fn serialize<S: Serializer>(version: &u32, serializer: S) -> Result<S::Ok, S::Error> {
        hex::encode(version.to_be_bytes()).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
        let hex_string = <&str as Deserialize<'de>>::deserialize(deserializer)?;
        let bytes = hex::decode(hex_string).map_err(de::Error::custom)?;
        let data = <&[u8; 4]>::try_from(&bytes[..]).map_err(de::Error::custom)?;
        Ok(u32::from_be_bytes(*data))
    }
}
