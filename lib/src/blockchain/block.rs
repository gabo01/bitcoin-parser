#[cfg(feature = "writer")]
use serde::{Deserialize, Serialize};

use crate::types::{BitcoinHash as BHash, BlockTarget};
use crate::Transaction as TransactionTrait;
use crate::TransactionBlock;

#[cfg_attr(feature = "writer", derive(Serialize, Deserialize))]
pub struct Block<T: TransactionTrait> {
    #[cfg_attr(feature = "writer", serde(flatten))]
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

#[cfg_attr(feature = "writer", derive(Serialize, Deserialize))]
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

#[cfg_attr(feature = "writer", derive(Serialize, Deserialize))]
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
