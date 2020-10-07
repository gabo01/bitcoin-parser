#![allow(dead_code)]

#[cfg(feature = "parser")]
pub mod blkparser;
pub mod blockchain;
pub mod cursor;
#[cfg(feature = "writer")]
pub mod disk;
#[cfg(feature = "parser")]
pub mod parser;
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
