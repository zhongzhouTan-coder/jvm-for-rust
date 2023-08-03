pub struct Buffer<'a> {
    buffer: &'a [u8],
    position: usize,
}

impl<'a> Buffer<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Buffer {
            buffer: data,
            position: 0,
        }
    }
}
