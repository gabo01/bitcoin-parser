use arrayref::array_ref;
use byteorder::LittleEndian;

use super::cursor::Cursor;
use crate::types::VarInt;

pub fn read_var_int(cursor: &mut Cursor<'_>) -> VarInt {
    let byte = cursor.read_bytes(1);
    read_var_int_marker(byte[0], cursor)
}

pub fn read_var_int_marker(marker: u8, cursor: &mut Cursor<'_>) -> VarInt {
    match marker {
        0x00..=0xfc => VarInt::new(marker as u64),
        0xfd => VarInt::from_2_bytes::<LittleEndian>(array_ref!(cursor.read_bytes(2), 0, 2)),
        0xfe => VarInt::from_4_bytes::<LittleEndian>(array_ref!(cursor.read_bytes(4), 0, 4)),
        0xff => VarInt::from_8_bytes::<LittleEndian>(array_ref!(cursor.read_bytes(8), 0, 8)),
    }
}
