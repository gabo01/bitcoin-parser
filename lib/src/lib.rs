#![allow(dead_code)]

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
