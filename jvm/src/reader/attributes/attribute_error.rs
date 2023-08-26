use crate::reader::buffer::BufferError;

pub enum AttributeError {
    DecodeAttributeError,
}

impl From<BufferError> for AttributeError {
    fn from(value: BufferError) -> Self {
        AttributeError::DecodeAttributeError
    }
}
