use super::script::BitcoinScript as BScript;
use crate::types::BitcoinHash;
use crate::Transaction as TransactionTrait;

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

pub struct Input {
    utxo: Utxo,
    signature: BScript,
    sequence: u32,
    witness: Option<BScript>,
}

impl Input {
    pub fn new(utxo: Utxo, signature: BScript, sequence: u32, witness: Option<BScript>) -> Self {
        Self {
            utxo,
            signature,
            sequence,
            witness,
        }
    }
}

pub struct Utxo {
    txid: BitcoinHash,
    vout: u32,
}

impl Utxo {
    pub fn new(txid: BitcoinHash, vout: u32) -> Self {
        Self { txid, vout }
    }
}

pub struct Output {
    value: u64,
    pubkey: BScript,
}

impl Output {
    pub fn new(value: u64, pubkey: BScript) -> Self {
        Self { value, pubkey }
    }
}
