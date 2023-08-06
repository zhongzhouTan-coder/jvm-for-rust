pub struct Buffer<'a> {
    buffer: &'a [u8],
    position: usize,
}

/// Errors related to reading from a [Buffer]
#[derive(Debug, PartialEq)]
pub enum BufferError {
    OutBoundaryOfData,
}

type Result<T> = std::result::Result<T, BufferError>;

impl<'a> Buffer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Buffer {
            buffer: data,
            position: 0,
        }
    }

    fn read_bytes(&mut self, size: usize) -> Result<&'a [u8]> {
        if self.position + size > self.buffer.len() {
            Err(BufferError::OutBoundaryOfData)
        } else {
            let slice = &self.buffer[self.position..self.position + size];
            self.position += size;
            Ok(slice)
        }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        self.read_bytes(std::mem::size_of::<u8>())
            .map(|bytes| u8::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        self.read_bytes(std::mem::size_of::<u16>())
            .map(|bytes| u16::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        self.read_bytes(std::mem::size_of::<u32>())
            .map(|bytes| u32::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_utf8(&mut self, length: usize) -> Result<String> {
        self.read_bytes(length)
            .map(|bytes| String::from_utf8(bytes.try_into().unwrap()).unwrap())
    }

    pub fn read_integer(&mut self) -> Result<i32> {
        self.read_bytes(std::mem::size_of::<i32>())
            .map(|bytes| i32::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_float(&mut self) -> Result<f32> {
        self.read_bytes(std::mem::size_of::<f32>())
            .map(|bytes| f32::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_double(&mut self) -> Result<f64> {
        self.read_bytes(std::mem::size_of::<f64>())
            .map(|bytes| f64::from_be_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_long(&mut self) -> Result<i64> {
        self.read_bytes(std::mem::size_of::<i64>())
            .map(|bytes| i64::from_be_bytes(bytes.try_into().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::buffer::Buffer;

    #[test]
    fn buffer_works() {
        let data = vec![0x00, 0x00, 0x00, 0x42];
        let mut buffer = Buffer::new(&data);

        assert_eq!(0x42u32, buffer.read_u32().unwrap());
    }
}
