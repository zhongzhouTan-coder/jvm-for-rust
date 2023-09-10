#[derive(Debug, Default)]
pub struct Symbol {
    pub data: String,
}

impl Symbol {
    pub fn new(data: Vec<u8>) -> Self {
        unsafe {
            Symbol {
                data: String::from_utf8_unchecked(data),
            }
        }
    }
}
