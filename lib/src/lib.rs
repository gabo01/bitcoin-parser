#![allow(dead_code)]

pub mod blockchain;
pub mod types;

pub trait TransactionBlock {
    type Transaction: Transaction;
}

pub trait Transaction {}
