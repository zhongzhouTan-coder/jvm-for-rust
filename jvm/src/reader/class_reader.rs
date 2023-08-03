use super::{buffer::Buffer, class_file::ClassFile};

struct ClassReader<'a> {
    buffer: Buffer<'a>,
    class_file: ClassFile,
}

impl<'a> ClassReader<'a> {
    pub fn new(data: &[u8]) -> ClassReader {
        ClassReader {
            buffer: Buffer::new(data),
            class_file: Default::default(),
        }
    }
}
