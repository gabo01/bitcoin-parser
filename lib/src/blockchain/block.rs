use super::transactions::Transaction;
use crate::types::{BitcoinHash as BHash, BlockTarget};
use crate::TransactionBlock;

pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
}

impl Block {
    pub(crate) fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Self {
            header,
            transactions,
        }
    }
}

impl TransactionBlock for Block {
    type Transaction = Transaction;
}

pub(crate) struct BlockHeader {
    version: u32,
    previous: BHash,
    txroot: BHash,
    consensus: MiningInfo,
}

impl BlockHeader {
    pub(crate) fn new(version: u32, previous: BHash, txroot: BHash, consensus: MiningInfo) -> Self {
        Self {
            version,
            previous,
            txroot,
            consensus,
        }
    }
}

pub(crate) struct MiningInfo {
    time: u32,
    bits: BlockTarget,
    nonce: u32,
}

impl MiningInfo {
    pub(crate) fn new(time: u32, bits: BlockTarget, nonce: u32) -> Self {
        Self { time, bits, nonce }
    }
}
