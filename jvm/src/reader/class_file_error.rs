#[derive(Debug, PartialEq, Eq)]
pub enum ClassFileError {
    ClassFormatError(String),
    InCompatibleClassChangeError,
    UnsupportedClassVersionError,
}
