use std::error::Error as StdError;
use std::io;
use std::path::Path;
use thiserror::Error;

use crate::blockchain::BlockChain;
use crate::TransactionBlock;

macro_rules! err_bound {
    () => {
        dyn StdError + Send + Sync + 'static
    };
}

#[cfg(feature = "parallel")]
pub trait ParallelParser<T: TransactionBlock> {
    fn parse<P: AsRef<Path>>(&mut self, file: P) -> Result<BlockChain<T>, ParseError>;
}

pub trait Parser<T: TransactionBlock> {
    fn parse<P: AsRef<Path>>(&mut self, file: P) -> Result<BlockChain<T>, ParseError>;
}

#[derive(Debug, Error)]
#[error("unable to perform the blockchain deserialization")]
pub struct ParseError {
    kind: ParseErrorKind,
    source: Option<Box<err_bound!()>>,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, source: Option<Box<err_bound!()>>) -> Self {
        Self { kind, source }
    }
}

impl From<io::Error> for ParseError {
    fn from(source: io::Error) -> Self {
        Self {
            kind: ParseErrorKind::ReadError,
            source: Some(Box::new(source)),
        }
    }
}

impl From<BlockError> for ParseError {
    fn from(source: BlockError) -> Self {
        Self {
            kind: ParseErrorKind::BlockError,
            source: Some(Box::new(source)),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseErrorKind {
    #[error("unable to read block data")]
    ReadError,
    #[error("unable to deserialize block from data")]
    BlockError,
}

#[derive(Debug, Error)]
#[error("unable to deserialize block")]
pub struct BlockError {
    kind: BlockErrorKind,
    source: Option<Box<err_bound!()>>,
}

impl BlockError {
    pub fn new(kind: BlockErrorKind, source: Option<Box<err_bound!()>>) -> Self {
        Self { kind, source }
    }
}

impl From<HeaderError> for BlockError {
    fn from(source: HeaderError) -> Self {
        Self {
            kind: BlockErrorKind::HeaderError,
            source: Some(Box::new(source)),
        }
    }
}

impl From<TransactionError> for BlockError {
    fn from(source: TransactionError) -> Self {
        Self {
            kind: BlockErrorKind::TransactionError,
            source: Some(Box::new(source)),
        }
    }
}

#[derive(Debug, Error)]
pub enum BlockErrorKind {
    #[error("invalid format for the block header")]
    HeaderError,
    #[error("invalid format for the block transactions")]
    TransactionError,
    #[error("unable to read block data")]
    ReadError,
}

#[derive(Debug, Error)]
#[error("unable to deserialize the block header")]
pub struct HeaderError {
    kind: HeaderErrorKind,
    source: io::Error,
}

impl HeaderError {
    pub fn new(kind: HeaderErrorKind, source: io::Error) -> Self {
        Self { kind, source }
    }
}

#[derive(Debug, Error)]
pub enum HeaderErrorKind {
    #[error("unable to get the block version")]
    VersionError,
    #[error("unable to get the previous block hash")]
    HashError,
    #[error("unable to get the merkle root")]
    RootError,
    #[error("unable to get the block time")]
    TimeError,
    #[error("unable to get the block target")]
    BitsError,
    #[error("unable to get the block nonce")]
    NonceError,
}

#[derive(Debug, Error)]
#[error("unable to deserialize transaction")]
pub struct TransactionError {
    kind: TransactionErrorKind,
    source: Option<Box<err_bound!()>>,
}

impl TransactionError {
    pub fn new(kind: TransactionErrorKind, source: Option<Box<err_bound!()>>) -> Self {
        Self { kind, source }
    }
}

#[derive(Debug, Error)]
pub enum TransactionErrorKind {
    #[error("unable to read block data")]
    ReadError,
    #[error("unexpected flag value")]
    FlagError(u64),
}
