use arrayref::array_ref;
use byteorder::{LittleEndian, ReadBytesExt};
#[cfg(feature = "writer")]
use serde::{Deserialize, Serialize};

use super::cursor::Cursor;
use super::helpers::{read_var_int, read_var_int_marker};
use crate::blockchain::script::BitcoinScript as BScript;
use crate::blockchain::transactions::Input;
use crate::blockchain::transactions::Output;
use crate::blockchain::transactions::Transaction;
use crate::blockchain::transactions::Utxo;
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
    pub fn from_raw_data<'a>(cursor: &mut Cursor<'a>) -> Self {
        let version = cursor
            .read_u32::<LittleEndian>()
            .expect("Transaction version has to exist for valid transaction");
        let marker = cursor.read_bytes(1)[0];
        if marker == 0x00 {
            return Self::from_segwit_raw_data(version, cursor);
        }
        let txin = read_var_int_marker(marker, cursor);
        let inputs = Self::read_inputs(*txin, cursor);
        let txout = read_var_int(cursor);
        let outputs = Self::read_outputs(*txout, cursor);
        let locktime = cursor
            .read_u32::<LittleEndian>()
            .expect("Transaction locktime has to exist for valid transaction");
        Self {
            txin,
            txout,
            contents: Transaction::new(version, inputs, outputs, locktime),
        }
    }

    fn from_segwit_raw_data(version: u32, cursor: &mut Cursor<'_>) -> Self {
        let txin = read_var_int(cursor);
        let mut inputs = Self::read_inputs(*txin, cursor);
        let txout = read_var_int(cursor);
        let outputs = Self::read_outputs(*txout, cursor);
        for input in &mut inputs {
            let witness_size = read_var_int(cursor);
            if *witness_size > 0 {
                let witness = BScript::new(cursor.read_bytes(*witness_size as usize).to_owned());
                input.assign_witness(witness);
            }
        }
        let locktime = cursor
            .read_u32::<LittleEndian>()
            .expect("Transaction locktime has to exist for valid transaction");

        Self {
            txin,
            txout,
            contents: Transaction::new(version, inputs, outputs, locktime),
        }
    }

    fn read_inputs(amount: u64, cursor: &mut Cursor) -> Vec<Input> {
        let mut inputs = vec![];
        for _ in 0..amount {
            let txid = BHash::new(array_ref!(cursor.read_bytes(32), 0, 32).to_owned());
            let vout = cursor
                .read_u32::<LittleEndian>()
                .expect("Valid transaction must have a vout");
            let scriptsize = read_var_int(cursor);
            let signature = cursor.read_bytes(*scriptsize as usize).to_owned();
            let sequence = cursor
                .read_u32::<LittleEndian>()
                .expect("Valid transaction must have a sequence");
            let input = Input::new(
                Utxo::new(txid, vout),
                BScript::new(signature),
                sequence,
                None,
            );
            inputs.push(input);
        }
        inputs
    }

    fn read_outputs(amount: u64, cursor: &mut Cursor) -> Vec<Output> {
        let mut outputs = vec![];
        for _ in 0..amount {
            let value = cursor
                .read_u64::<LittleEndian>()
                .expect("Valid transaction must have a value");
            let scriptsize = read_var_int(cursor);
            let pubkey = cursor.read_bytes(*scriptsize as usize).to_owned();
            let output = Output::new(value, BScript::new(pubkey));
            outputs.push(output);
        }
        outputs
    }
}

impl TransactionTrait for SerialTransaction {}
