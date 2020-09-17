use arrayref::array_ref;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
#[cfg(feature = "writer")]
use serde::{Deserialize, Serialize};

use super::cursor::Cursor;
use super::helpers::{read_var_int, read_var_int_marker};
use crate::blockchain::script::BitcoinScript as BScript;
use crate::blockchain::transactions::Input;
use crate::blockchain::transactions::Output;
use crate::blockchain::transactions::Transaction;
use crate::blockchain::transactions::Utxo;
use crate::blockchain::transactions::Witness;
use crate::parser::{TransactionError as TxError, TransactionErrorKind as TxErrorKind};
use crate::types::BitcoinHash as BHash;
use crate::types::BitcoinHashBuilder as BHashBuilder;
use crate::types::VarInt;
use crate::Transaction as TransactionTrait;

#[cfg_attr(feature = "writer", derive(Serialize, Deserialize))]
pub struct SerialTransaction {
    txin: VarInt,
    txout: VarInt,
    hash: BHash,
    #[cfg_attr(feature = "writer", serde(flatten))]
    contents: Transaction,
}

impl SerialTransaction {
    pub fn from_raw_data<'a>(cursor: &mut Cursor<'a>) -> Result<Self, TxError> {
        let mut txparser = TxParser::new(cursor);
        let segwit;
        let version = txparser.parse_version()?;
        let marker = txparser.parse_bytes_hash(1, |bytes| bytes[0] != 0x00)?[0];
        segwit = marker == 0x00;
        let (txin, mut inputs, txout, outputs);
        if segwit {
            let flag = txparser.parse_bytes(1)?[0];
            if flag != 0x01 {
                return Err(TxError::new(TxErrorKind::FlagError(flag as u64), None));
            }
            txin = txparser.parse_var_int_hash()?;
        } else {
            txin = txparser.parse_var_int_marker(marker)?;
        }
        inputs = Self::read_inputs(*txin, &mut txparser)?;
        txout = txparser.parse_var_int_hash()?;
        outputs = Self::read_outputs(*txout, &mut txparser)?;
        if segwit {
            Self::read_witnesses(&mut inputs, &mut txparser)?;
        }
        let locktime = txparser.parse_locktime()?;
        Ok(Self {
            txin,
            txout,
            hash: txparser.generate_txhash(),
            contents: Transaction::new(version, inputs, outputs, locktime),
        })
    }

    fn read_inputs(amount: u64, parser: &mut TxParser<'_, '_>) -> Result<Vec<Input>, TxError> {
        let mut inputs = vec![];
        for _ in 0..amount {
            let txid = parser.parse_txid()?;
            let vout = parser.parse_vout()?;
            let (_scriptsize, signature) = parser.parse_script()?;
            let sequence = parser.parse_sequence()?;

            let utxo = Utxo::new(txid, vout);
            let script = BScript::new(signature);
            let input = Input::new(utxo, script, sequence, None);
            inputs.push(input);
        }
        Ok(inputs)
    }

    fn read_outputs(amount: u64, parser: &mut TxParser<'_, '_>) -> Result<Vec<Output>, TxError> {
        let mut outputs = vec![];
        for _ in 0..amount {
            let value = parser.parse_out_value()?;
            let (_scriptsize, pubkey) = parser.parse_script()?;
            let output = Output::new(value, BScript::new(pubkey));
            outputs.push(output);
        }
        Ok(outputs)
    }

    fn read_witnesses(inputs: &mut [Input], parser: &mut TxParser<'_, '_>) -> Result<(), TxError> {
        for input in inputs {
            let witness = Self::read_witness(parser)?;
            if let Some(witness) = witness {
                input.assign_witness(witness);
            }
        }
        Ok(())
    }

    fn read_witness(parser: &mut TxParser<'_, '_>) -> Result<Option<Witness>, TxError> {
        let witness_stack_size = parser.parse_var_int()?;
        if *witness_stack_size == 0x00 {
            return Ok(None);
        }
        let mut items = vec![];
        for __ in 0..*witness_stack_size {
            let item_length = parser.parse_var_int()?;
            let item = parser.parse_bytes(*item_length as usize)?.to_owned();
            items.push(item)
        }
        Ok(Some(Witness::new(items)))
    }
}

impl TransactionTrait for SerialTransaction {}

struct TxParser<'a, 'b: 'a> {
    hasher: BHashBuilder,
    cursor: &'a mut Cursor<'b>,
}

impl<'a, 'b: 'a> TxParser<'a, 'b> {
    fn new(cursor: &'a mut Cursor<'b>) -> Self {
        Self {
            hasher: BHashBuilder::default(),
            cursor,
        }
    }

    fn generate_txhash(self) -> BHash {
        self.hasher.into_hash()
    }

    fn parse_bytes(&mut self, bytes: usize) -> Result<&'b [u8], TxError> {
        let read = self.cursor.read_bytes(bytes);
        let bytes =
            read.map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
        Ok(bytes)
    }

    fn parse_bytes_hash<F>(&mut self, bytes: usize, hash: F) -> Result<&'b [u8], TxError>
    where
        F: Fn(&[u8]) -> bool,
    {
        let bytes = self.parse_bytes(bytes)?;
        if hash(bytes) {
            self.hasher.add_digest(bytes)
        }
        Ok(bytes)
    }

    fn parse_var_int(&mut self) -> Result<VarInt, TxError> {
        Ok(read_var_int(self.cursor)
            .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?)
    }

    fn parse_var_int_hash(&mut self) -> Result<VarInt, TxError> {
        let position = self.cursor.position();
        let bytes = self.parse_var_int()?;
        self.hasher
            .add_digest(&self.cursor.get_ref()[position..self.cursor.position()]);
        Ok(bytes)
    }

    fn parse_var_int_marker(&mut self, marker: u8) -> Result<VarInt, TxError> {
        Ok(read_var_int_marker(marker, self.cursor)
            .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?)
    }

    fn parse_version(&mut self) -> Result<u32, TxError> {
        let bytes = self.parse_bytes(4)?;
        self.hasher.add_digest(bytes);
        Ok(LittleEndian::read_u32(bytes))
    }

    fn parse_txid(&mut self) -> Result<BHash, TxError> {
        Ok(BHash::new(
            array_ref!(self.parse_bytes_hash(32, |_| true)?, 0, 32).to_owned(),
        ))
    }

    fn parse_vout(&mut self) -> Result<u32, TxError> {
        let bytes = self.parse_bytes(4)?;
        self.hasher.add_digest(bytes);
        Ok(LittleEndian::read_u32(bytes))
    }

    fn parse_script(&mut self) -> Result<(VarInt, Vec<u8>), TxError> {
        let size = self.parse_var_int_hash()?;
        let script = self.parse_bytes_hash(*size as usize, |_| true)?.to_owned();
        Ok((size, script))
    }

    fn parse_sequence(&mut self) -> Result<u32, TxError> {
        let bytes = self.parse_bytes(4)?;
        self.hasher.add_digest(bytes);
        Ok(LittleEndian::read_u32(bytes))
    }

    fn parse_out_value(&mut self) -> Result<u64, TxError> {
        let bytes = self.parse_bytes(8)?;
        self.hasher.add_digest(bytes);
        Ok(LittleEndian::read_u64(bytes))
    }

    fn parse_locktime(&mut self) -> Result<u32, TxError> {
        let bytes = self.parse_bytes(4)?;
        self.hasher.add_digest(bytes);
        Ok(LittleEndian::read_u32(bytes))
    }
}
