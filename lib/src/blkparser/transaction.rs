use arrayref::array_ref;
use byteorder::{LittleEndian, ReadBytesExt};
#[cfg(feature = "writer")]
use serde::{Deserialize, Serialize};

use super::cursor::Cursor;
use super::helpers::read_var_int;
use crate::blockchain::script::BitcoinScript as BScript;
use crate::blockchain::transactions::Input;
use crate::blockchain::transactions::Output;
use crate::blockchain::transactions::Transaction;
use crate::blockchain::transactions::Utxo;
use crate::blockchain::transactions::Witness;
use crate::parser::{TransactionError as TxError, TransactionErrorKind as TxErrorKind};
use crate::types::BitcoinHash as BHash;
use crate::types::VarInt;
use crate::Transaction as TransactionTrait;

#[cfg_attr(feature = "writer", derive(Serialize, Deserialize))]
pub struct SerialTransaction {
    txin: VarInt,
    txout: VarInt,
    // hash: BHash,
    #[cfg_attr(feature = "writer", serde(flatten))]
    contents: Transaction,
}

impl SerialTransaction {
    pub fn from_raw_data<'a>(cursor: &mut Cursor<'a>) -> Result<Self, TxError> {
        let version = cursor
            .read_u32::<LittleEndian>()
            .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
        let marker = read_var_int(cursor)
            .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
        if *marker == 0x00 {
            let flag = read_var_int(cursor)
                .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
            return match *flag {
                0x01 => Ok(Self::from_segwit_raw_data(version, cursor)?),
                _ => Err(TxError::new(TxErrorKind::FlagError(*flag), None)),
            };
        }
        let txin = marker;
        let inputs = Self::read_inputs(*txin, cursor)?;
        let txout = read_var_int(cursor)
            .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
        let outputs = Self::read_outputs(*txout, cursor)?;
        let locktime = cursor
            .read_u32::<LittleEndian>()
            .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
        Ok(Self {
            txin,
            txout,
            contents: Transaction::new(version, inputs, outputs, locktime),
        })
    }

    fn from_segwit_raw_data(version: u32, cursor: &mut Cursor<'_>) -> Result<Self, TxError> {
        let txin = read_var_int(cursor)
            .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
        let mut inputs = Self::read_inputs(*txin, cursor)?;
        let txout = read_var_int(cursor)
            .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
        let outputs = Self::read_outputs(*txout, cursor)?;
        for input in &mut inputs {
            let witness = Self::read_witness(cursor)?;
            if let Some(witness) = witness {
                input.assign_witness(witness);
            }
        }
        let locktime = cursor
            .read_u32::<LittleEndian>()
            .expect("Transaction locktime has to exist for valid transaction");

        Ok(Self {
            txin,
            txout,
            contents: Transaction::new(version, inputs, outputs, locktime),
        })
    }

    fn read_inputs(amount: u64, cursor: &mut Cursor) -> Result<Vec<Input>, TxError> {
        let mut inputs = vec![];
        for _ in 0..amount {
            let txid = BHash::new(
                array_ref!(
                    cursor
                        .read_bytes(32)
                        .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?,
                    0,
                    32
                )
                .to_owned(),
            );
            let vout = cursor
                .read_u32::<LittleEndian>()
                .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
            let scriptsize = read_var_int(cursor)
                .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
            let signature = cursor
                .read_bytes(*scriptsize as usize)
                .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?
                .to_owned();
            let sequence = cursor
                .read_u32::<LittleEndian>()
                .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
            let input = Input::new(
                Utxo::new(txid, vout),
                BScript::new(signature),
                sequence,
                None,
            );
            inputs.push(input);
        }
        Ok(inputs)
    }

    fn read_outputs(amount: u64, cursor: &mut Cursor) -> Result<Vec<Output>, TxError> {
        let mut outputs = vec![];
        for _ in 0..amount {
            let value = cursor
                .read_u64::<LittleEndian>()
                .expect("Valid transaction must have a value");
            let scriptsize = read_var_int(cursor)
                .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
            let pubkey = cursor
                .read_bytes(*scriptsize as usize)
                .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?
                .to_owned();
            let output = Output::new(value, BScript::new(pubkey));
            outputs.push(output);
        }
        Ok(outputs)
    }

    fn read_witness(cursor: &mut Cursor) -> Result<Option<Witness>, TxError> {
        let witness_stack_size = read_var_int(cursor)
            .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
        if *witness_stack_size == 0x00 {
            return Ok(None);
        }
        let mut items = vec![];
        for __ in 0..*witness_stack_size {
            let item_length = read_var_int(cursor)
                .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?;
            let item = cursor
                .read_bytes(*item_length as usize)
                .map_err(|err| TxError::new(TxErrorKind::ReadError, Some(Box::new(err))))?
                .to_owned();
            items.push(item)
        }
        Ok(Some(Witness::new(items)))
    }
}

impl TransactionTrait for SerialTransaction {}
