use crate::utilities::definition::{jdouble, jfloat, jint, jlong, u2};

pub enum ConstantPoolEntry {
    Utf8(String),
    Integer(jint),
    Float(jfloat),
    Double(jdouble),
    Long(jlong),
    Class(u2),
    String(u2),
    FieldRef(u2, u2),
    MethodRef(u2, u2),
    NameAndType(u2, u2),
    Invalid,
}

pub struct ConstantPool {
    entries: Vec<ConstantPoolEntry>,
}
