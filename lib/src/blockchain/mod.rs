#[cfg(feature = "writer")]
use serde::{Deserialize, Serialize};

use std::iter::FromIterator;

pub mod block;
pub mod script;
pub mod transactions;

use crate::TransactionBlock;

#[cfg_attr(feature = "writer", derive(Serialize, Deserialize))]
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

impl<T: TransactionBlock> IntoIterator for BlockChain<T> {
    type Item = <Vec<T> as IntoIterator>::Item;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.blocks.into_iter()
    }
}

impl<'a, T: TransactionBlock> IntoIterator for &'a BlockChain<T> {
    type Item = <&'a Vec<T> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.blocks.iter()
    }
}

impl<'a, T: TransactionBlock> IntoIterator for &'a mut BlockChain<T> {
    type Item = <&'a mut Vec<T> as IntoIterator>::Item;
    type IntoIter = <&'a mut Vec<T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> <Self as std::iter::IntoIterator>::IntoIter {
        self.blocks.iter_mut()
    }
}
