use crate::types::{BitcoinHash as BHash, BlockTarget};
use crate::Transaction as TransactionTrait;
use crate::TransactionBlock;

pub struct Block<T: TransactionTrait> {
    header: BlockHeader,
    transactions: Vec<T>,
}

impl<T: TransactionTrait> Block<T> {
    pub(crate) fn new(header: BlockHeader, transactions: Vec<T>) -> Self {
        Self {
            header,
            transactions,
        }
    }
}

impl<T: TransactionTrait> TransactionBlock for Block<T> {
    type Transaction = T;
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
