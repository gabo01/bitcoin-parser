use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use crate::blockchain::BlockChain;
use crate::parser::Parser;
use crate::parser::{ParseError, ParseErrorKind};

mod block;
mod cursor;
mod helpers;
mod transaction;

pub use block::SerialBlock;
use cursor::Cursor;
pub use transaction::SerialTransaction;

pub const BLOCK_FILE_SIZE: u64 = 128 * 1024 * 1024;
pub const MAGIC_BYTES: u32 = 0xf9beb4d9;

#[derive(Default)]
pub struct BitcoinParser {
    blkbuffer: Vec<u8>,
}

impl BitcoinParser {
    pub fn new() -> Self {
        Self::default()
    }

    fn read_raw_blocks(buffer: &[u8]) -> impl Iterator<Item = io::Result<Cursor<'_>>> {
        BlockIterator::new(Cursor::new(buffer))
    }

    fn read_file_contents<P: AsRef<Path>>(&mut self, file: P) -> Result<&[u8], io::Error> {
        if self.blkbuffer.capacity() == 0 {
            self.blkbuffer = Vec::with_capacity(BLOCK_FILE_SIZE as usize);
        }
        if !self.blkbuffer.is_empty() {
            self.blkbuffer.clear();
        }
        let mut file = File::open(file)?;
        file.read_to_end(&mut self.blkbuffer)?;
        Ok(&self.blkbuffer)
    }
}

impl Parser<SerialBlock> for BitcoinParser {
    fn parse<P: AsRef<Path>>(&mut self, file: P) -> Result<BlockChain<SerialBlock>, ParseError> {
        let buffer = self.read_file_contents(file)?;
        Ok(Self::read_raw_blocks(&buffer)
            .map(|block| {
                block
                    .map_err(|err| ParseError::new(ParseErrorKind::ReadError, Some(Box::new(err))))
                    .and_then(|data| SerialBlock::from_raw_data(data).map_err(From::from))
            })
            .collect::<Result<BlockChain<SerialBlock>, _>>()?)
    }
}

struct BlockIterator<'a> {
    buffer: Cursor<'a>,
}

impl<'a> BlockIterator<'a> {
    fn new(buffer: Cursor<'a>) -> Self {
        Self { buffer }
    }
}

impl<'a> Iterator for BlockIterator<'a> {
    type Item = io::Result<Cursor<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.buffer.read_u32::<BigEndian>() {
            Ok(delimiter) if delimiter == MAGIC_BYTES => Some(
                self.buffer
                    .read_u32::<LittleEndian>()
                    .and_then(|size| self.buffer.bytes_to_cursor(size as usize)),
            ),
            Ok(delimiter) => Some(Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "Expected to find the magic bytes: {} but found instead {}",
                    MAGIC_BYTES, delimiter
                ),
            ))),
            Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(err) => Some(Err(err)),
        }
    }
}
