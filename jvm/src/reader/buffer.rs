pub struct Buffer<'a> {
    buffer: &'a [u8],
    buffer_start: usize,
    buffer_end: usize,
    current: usize,
    source: &'a str,
}

impl<'a> Buffer<'a> {
    pub fn new(buffer: &'a [u8], length: usize, source: &'a str) -> Self {
        Buffer {
            buffer,
            buffer_start: 0,
            buffer_end: length,
            current: 0,
            source,
        }
    }

    pub fn length(&self) -> usize {
        self.buffer_end - self.buffer_start
    }

    pub fn source(&self) -> &str {
        self.source
    }

    pub fn get_u8_fast(&mut self) -> u8 {
        self.current += 1;
        self.buffer[self.current - 1]
    }

    pub fn get_u8_array_fast(&mut self, length: usize) -> &[u8] {
        self.current += length;
        &self.buffer[self.current - length..self.current]
    }

    pub fn get_u8(&mut self) -> u8 {
        assert!(1 <= self.buffer_end - self.current, "buffer overflow");
        self.get_u8_fast()
    }

    pub fn get_u16_fast(&mut self) -> u16 {
        let high_byte = self.get_u8_fast();
        let low_byte = self.get_u8_fast();
        (high_byte as u16) << 8 | low_byte as u16
    }

    pub fn get_u16(&mut self) -> u16 {
        assert!(2 <= self.buffer_end - self.current, "buffer overflow");
        self.get_u16_fast()
    }

    pub fn get_u32_fast(&mut self) -> u32 {
        let high_u16 = self.get_u16_fast();
        let low_u16 = self.get_u16_fast();
        (high_u16 as u32) << 16 | low_u16 as u32
    }

    pub fn get_u32(&mut self) -> u32 {
        assert!(4 <= self.buffer_end - self.current, "buffer overflow");
        self.get_u32_fast()
    }

    pub fn get_u64_fast(&mut self) -> u64 {
        let high_u32 = self.get_u32_fast();
        let low_u32 = self.get_u32_fast();
        (high_u32 as u64) << 32 | low_u32 as u64
    }

    pub fn get_u64(&mut self) -> u64 {
        assert!(8 <= self.buffer_end - self.current, "buffer overflow");
        self.get_u64_fast()
    }

    pub fn skip_u8_fast(&mut self, length: usize) {
        self.current += 1 * length
    }

    pub fn skip_u16_fast(&mut self, length: usize) {
        self.current += 2 * length
    }

    pub fn skip_u32_fast(&mut self, length: usize) {
        self.current += 4
    }

    pub fn guarantee_more(&self, size: usize) {
        assert!(
            size <= self.buffer_end - self.buffer_start,
            "buffer overflow"
        )
    }

    pub fn at_eos(&self) -> bool {
        self.current == self.buffer_end
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::buffer::Buffer;

    #[test]
    fn we_can_get_u8_from_buffer() {
        let data = vec![0x32, 0x00, 0x00, 0x42];
        let mut buffer = Buffer::new(&data, data.len(), "src");

        assert_eq!(0x32u8, buffer.get_u8());
    }

    #[test]
    fn we_can_get_u16_from_buffer() {
        let data = vec![0x32, 0x21, 0x00, 0x42];
        let mut buffer = Buffer::new(&data, data.len(), "src");

        assert_eq!(0x3221u16, buffer.get_u16());
    }

    #[test]
    fn we_can_get_u32_from_buffer() {
        let data = vec![0x32, 0x21, 0x00, 0x42];
        let mut buffer = Buffer::new(&data, data.len(), "src");

        assert_eq!(0x32210042u32, buffer.get_u32());
    }
}
