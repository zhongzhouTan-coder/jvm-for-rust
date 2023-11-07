pub struct Buffer {
    data: Vec<u8>,
    pos: usize,
    len: usize,
}

impl Buffer {
    pub fn new(data: Vec<u8>) -> Self {
        let len = data.len();
        Buffer { data, pos: 0, len }
    }
    pub fn read_u8(&mut self) -> u8 {
        assert!(self.pos < self.len, "read out of boundary data!");
        let mut bytes = [0u8; 1];
        bytes.copy_from_slice(&self.data[self.pos..self.pos + 1]);
        self.pos += 1;
        u8::from_ne_bytes(bytes)
    }

    pub fn read_u16(&mut self) -> u16 {
        assert!(self.pos + 1 < self.len, "read out of boundary data!");
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(&self.data[self.pos..self.pos + 2]);
        self.pos += 2;
        u16::from_ne_bytes(bytes)
    }

    pub fn read_u32(&mut self) -> u32 {
        assert!(self.pos + 3 < self.len, "read out of boundary data!");
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.data[self.pos..self.pos + 4]);
        self.pos += 4;
        u32::from_ne_bytes(bytes)
    }
}
