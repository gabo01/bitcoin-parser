use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use crate::blockchain::BlockChain;
use crate::Parser;

mod block;
mod cursor;
mod helpers;
mod transaction;

pub use block::SerialBlock;
use cursor::Cursor;
pub use transaction::SerialTransaction;

pub const BLOCK_FILE_SIZE: u64 = 128 * 1024 * 1024;
pub const MAGIC_BYTES: u32 = 0xf9beb4d9;

pub struct BitcoinParser<'a> {
    file: &'a Path,
}

impl<'a> BitcoinParser<'a> {
    pub fn new(file: &'a Path) -> Self {
        Self { file }
    }

    fn read_raw_blocks<'buf>(buffer: &'buf [u8]) -> impl Iterator<Item = Cursor<'buf>> {
        BlockIterator::new(Cursor::new(buffer))
    }

    fn read_file_contents(&self) -> Result<Vec<u8>, io::Error> {
        let mut buffer = Vec::with_capacity(BLOCK_FILE_SIZE as usize);
        let mut file = File::open(self.file)?;
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

impl<'a> Parser<SerialBlock> for BitcoinParser<'a> {
    fn parse(&self) -> BlockChain<SerialBlock> {
        let buffer = self.read_file_contents().expect("unable to read file");
        Self::read_raw_blocks(&buffer)
            .map(|block| SerialBlock::from_raw_data(block))
            .collect::<BlockChain<SerialBlock>>()
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
    type Item = Cursor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let read = self.buffer.read_u32::<BigEndian>();
        match read {
            Ok(magic_bytes) => {
                if magic_bytes == MAGIC_BYTES {
                    let size = self
                        .buffer
                        .read_u32::<LittleEndian>()
                        .expect("Size has to exist for a valid blk file");
                    Some(self.buffer.bytes_to_cursor(size as usize))
                } else {
                    None
                }
            }
            Err(err) => {
                assert_eq!(
                    err.kind(),
                    io::ErrorKind::UnexpectedEof,
                    "The only valid error is end of file in case of no more blocks"
                );
                None
            }
        }
    }
}
