use std::io::Cursor as IOCursor;
use std::io::Read;

pub struct Cursor<'a> {
    data: IOCursor<&'a [u8]>,
}

impl<'a> Cursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data: IOCursor::new(data),
        }
    }

    pub fn bytes_to_cursor(&mut self, bytes: usize) -> Cursor<'a> {
        Cursor::new(self.read_bytes(bytes))
    }

    pub fn read_bytes(&mut self, bytes: usize) -> &'a [u8] {
        let slice = &(self.data.get_ref()
            [self.data.position() as usize..self.data.position() as usize + bytes]);
        self.data.set_position(self.data.position() + bytes as u64);
        slice
    }

    pub fn size(&self) -> usize {
        self.data.get_ref().len()
    }

    pub fn get_ref(&self) -> &[u8] {
        self.data.get_ref()
    }
}

impl<'a> Read for Cursor<'a> {
    fn read(&mut self, buffer: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        self.data.read(buffer)
    }
}
