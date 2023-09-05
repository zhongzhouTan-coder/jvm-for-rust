#[derive(Debug)]
pub struct Symbol {
    pub data: Vec<u8>,
}

impl Symbol {
    pub fn new(data: Vec<u8>) -> Self {
        Symbol { data }
    }
}
