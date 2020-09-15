use arrayref::array_ref;
use byteorder::{LittleEndian, ReadBytesExt};
#[cfg(feature = "writer")]
use serde::{Deserialize, Serialize};

use super::cursor::Cursor;
use super::helpers::read_var_int;
use super::transaction::SerialTransaction;
use crate::blockchain::block::Block;
use crate::blockchain::block::BlockHeader;
use crate::blockchain::block::MiningInfo;
use crate::types::BitcoinHash as BHash;
use crate::types::BlockTarget;
use crate::TransactionBlock;

#[cfg_attr(feature = "writer", derive(Serialize, Deserialize))]
pub struct SerialBlock {
    size: u32,
    hash: BHash,
    #[cfg_attr(feature = "writer", serde(flatten))]
    contents: Block<SerialTransaction>,
}

impl SerialBlock {
    pub fn from_raw_data<'a>(mut cursor: Cursor<'a>) -> Self {
        let size = cursor.size() as u32;
        let raw_header = cursor.bytes_to_cursor(80);
        let hash = BHash::hash_header(raw_header.get_ref());
        let header = SerialHeader::build_header(raw_header);
        let transactions = Self::read_transactions(cursor);

        Self {
            size,
            hash,
            contents: Block::new(header, transactions),
        }
    }

    fn read_transactions(mut cursor: Cursor<'_>) -> Vec<SerialTransaction> {
        let txcount = read_var_int(&mut cursor);
        let mut transactions = vec![];
        for _ in 0..*txcount {
            transactions.push(SerialTransaction::from_raw_data(&mut cursor));
        }
        transactions
    }
}

impl TransactionBlock for SerialBlock {
    type Transaction = SerialTransaction;
}

struct SerialHeader;

impl SerialHeader {
    fn build_header(mut cursor: Cursor<'_>) -> BlockHeader {
        let version = cursor
            .read_u32::<LittleEndian>()
            .expect("Version has to exist for a valid cursor");
        let prev_hash = BHash::new(array_ref!(cursor.read_bytes(32), 0, 32).to_owned());
        let merkle_root =
            BHash::from_little_endian(array_ref!(cursor.read_bytes(32), 0, 32).to_owned());
        let time = cursor
            .read_u32::<LittleEndian>()
            .expect("Time has to exist for a valid cursor");
        let bits = cursor.read_bytes(4);
        let nonce = cursor
            .read_u32::<LittleEndian>()
            .expect("Nonce has to exist for a valid cursor");

        BlockHeader::new(
            version,
            prev_hash,
            merkle_root,
            MiningInfo::new(time, BlockTarget::from(bits), nonce),
        )
    }

    fn build_hash_array(slice: &[u8]) -> [u8; 32] {
        assert!(slice.len() >= 32);
        let mut hash_array = [0; 32];
        hash_array
            .iter_mut()
            .zip(slice)
            .for_each(|(arraybyte, slicebyte)| *arraybyte = *slicebyte);
        hash_array
    }
}
