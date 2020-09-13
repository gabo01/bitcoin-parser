use std::iter::FromIterator;

pub mod block;
pub mod script;
pub mod transactions;

use crate::TransactionBlock;

pub struct BlockChain<T: TransactionBlock> {
    blocks: Vec<T>,
}

impl<T: TransactionBlock> BlockChain<T> {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }
}

impl<T: TransactionBlock> FromIterator<T> for BlockChain<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            blocks: iter.into_iter().collect(),
        }
    }
}
