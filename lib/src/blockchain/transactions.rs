#[cfg(feature = "writer")]
use serde::{de, ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};

use super::script::BitcoinScript as BScript;
use crate::types::BitcoinHash;
use crate::Transaction as TransactionTrait;

#[cfg_attr(feature = "writer", derive(Serialize, Deserialize))]
pub struct Transaction {
    version: u32,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    segwit: bool,
    locktime: u32,
}

impl Transaction {
    pub fn new(version: u32, inputs: Vec<Input>, outputs: Vec<Output>, locktime: u32) -> Self {
        let segwit = Transaction::look_for_witness(&inputs);
        Self {
            version,
            inputs,
            outputs,
            locktime,
            segwit,
        }
    }

    fn look_for_witness(inputs: &[Input]) -> bool {
        inputs.iter().any(|item| item.witness.is_some())
    }
}

impl TransactionTrait for Transaction {}

#[cfg_attr(feature = "writer", derive(Serialize, Deserialize))]
pub struct Input {
    #[cfg_attr(feature = "writer", serde(flatten))]
    utxo: Utxo,
    signature: BScript,
    sequence: u32,
    witness: Option<Witness>,
}

impl Input {
    pub fn new(utxo: Utxo, signature: BScript, sequence: u32, witness: Option<Witness>) -> Self {
        Self {
            utxo,
            signature,
            sequence,
            witness,
        }
    }

    pub fn assign_witness(&mut self, witness: Witness) {
        self.witness = Some(witness);
    }
}

#[cfg_attr(feature = "writer", derive(Serialize, Deserialize))]
pub struct Utxo {
    txid: BitcoinHash,
    vout: u32,
}

impl Utxo {
    pub fn new(txid: BitcoinHash, vout: u32) -> Self {
        Self { txid, vout }
    }
}

#[cfg_attr(feature = "writer", derive(Serialize, Deserialize))]
pub struct Output {
    value: u64,
    pubkey: BScript,
}

impl Output {
    pub fn new(value: u64, pubkey: BScript) -> Self {
        Self { value, pubkey }
    }
}

pub struct Witness {
    items: Vec<Vec<u8>>,
}

impl Witness {
    pub fn new(items: Vec<Vec<u8>>) -> Self {
        Self { items }
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}

impl IntoIterator for Witness {
    type Item = <Vec<Vec<u8>> as IntoIterator>::Item;
    type IntoIter = <Vec<Vec<u8>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.items.into_iter()
    }
}

impl<'a> IntoIterator for &'a Witness {
    type Item = <&'a Vec<Vec<u8>> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<Vec<u8>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.items.iter()
    }
}

impl<'a> IntoIterator for &'a mut Witness {
    type Item = <&'a mut Vec<Vec<u8>> as IntoIterator>::Item;
    type IntoIter = <&'a mut Vec<Vec<u8>> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.items.iter_mut()
    }
}

#[cfg(feature = "writer")]
impl Serialize for Witness {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.count()))?;
        for item in self {
            seq.serialize_element(&hex::encode(item))?;
        }
        seq.end()
    }
}

#[cfg(feature = "writer")]
impl<'de> Deserialize<'de> for Witness {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Witness, D::Error> {
        let witness_items = <Vec<&str> as Deserialize<'de>>::deserialize(deserializer)?
            .into_iter()
            .map(|hex_string| hex::decode(hex_string).map_err(de::Error::custom))
            .collect::<Result<Vec<Vec<u8>>, _>>()?;
        Ok(Witness::new(witness_items))
    }
}
