pub struct BitcoinScript {
    contents: Vec<u8>,
}

impl BitcoinScript {
    pub fn new(contents: Vec<u8>) -> Self {
        Self { contents }
    }
}
