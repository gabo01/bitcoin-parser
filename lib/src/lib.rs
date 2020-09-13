#![allow(dead_code)]

#[cfg(feature = "parser")]
pub mod blkparser;
pub mod blockchain;
pub mod types;

pub mod byteorder {
    pub use byteorder::BigEndian;
    pub use byteorder::ByteOrder;
    pub use byteorder::LittleEndian;
}

pub trait TransactionBlock {
    type Transaction: Transaction;
}

pub trait Transaction {}

#[cfg(feature = "parser")]
pub trait Parser<T: TransactionBlock> {
    fn parse(&self) -> blockchain::BlockChain<T>;
}
