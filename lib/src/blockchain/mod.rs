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
