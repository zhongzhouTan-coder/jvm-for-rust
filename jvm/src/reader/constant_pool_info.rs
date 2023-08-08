#[derive(Debug)]
pub struct ConstantPoolInfo {
    tag: u8,
    info: ConstantInfo,
}

#[derive(Debug)]
pub enum ConstantInfo {
    Utf8(u16, String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(u16),
    String(u16),
    FieldRef(u16, u16),
    MethodRef(u16, u16),
    InterfaceMethodRef(u16, u16),
    NameAndType(u16, u16),
    MethodHandle(u8, u16),
    MethodType(u16),
    Dynamic(u16, u16),
    InvokeDynamic(u16, u16),
    Module(u16),
    Package(u16),
}

impl ConstantPoolInfo {
    pub fn new(tag: u8, info: ConstantInfo) -> Self {
        ConstantPoolInfo { tag, info }
    }
}