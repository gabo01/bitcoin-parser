use serde::Serialize;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

#[cfg(feature = "json")]
pub const JSON_BLK_BUFFER: usize = 400 * 1024 * 1024;

pub trait Writer {
    fn save<S: Serialize, P: AsRef<Path>>(&mut self, data: S, file: P) -> io::Result<()>;
}

#[cfg(feature = "json")]
pub struct JsonWriter<'a> {
    path: &'a Path,
    buf: Vec<u8>,
}

#[cfg(feature = "json")]
impl<'a> JsonWriter<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path, buf: vec![] }
    }

    fn alloc_buffer(&mut self) {
        if self.buf.capacity() == 0 {
            self.buf = Vec::with_capacity(JSON_BLK_BUFFER);
        }
        if !self.buf.is_empty() {
            self.buf.clear()
        }
    }
}

#[cfg(feature = "json")]
impl<'a> Writer for JsonWriter<'a> {
    fn save<S: Serialize, P: AsRef<Path>>(&mut self, blockchain: S, file: P) -> io::Result<()> {
        self.alloc_buffer();
        json::to_writer(&mut self.buf, &blockchain).expect("Write to in memory buffer cannot fail");
        File::create(self.path.join(file))?.write_all(&self.buf)?;
        Ok(())
    }
}
