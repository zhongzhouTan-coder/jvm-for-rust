#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstantTag {
    JVM_CONSTANT_Invalid = 0,
    JVM_CONSTANT_Utf8 = 1,
    JVM_CONSTANT_Integer = 3,
    JVM_CONSTANT_Float = 4,
    JVM_CONSTANT_Long = 5,
    JVM_CONSTANT_Double = 6,
    JVM_CONSTANT_Class = 7,
    JVM_CONSTANT_String = 8,
    JVM_CONSTANT_Fieldref = 9,
    JVM_CONSTANT_Methodref = 10,
    JVM_CONSTANT_InterfaceMethodref = 11,
    JVM_CONSTANT_NameAndType = 12,
    JVM_CONSTANT_MethodHandle = 15,
    JVM_CONSTANT_MethodType = 16,
    JVM_CONSTANT_Dynamic = 17,
    JVM_CONSTANT_InvokeDynamic = 18,
    JVM_CONSTANT_Module = 19,
    JVM_CONSTANT_Package = 20,
    JVM_CONSTANT_ClassIndex,
    JVM_CONSTANT_StringIndex,
    JVM_CONSTANT_UnresolvedClass,
}

impl ConstantTag {
    pub fn is_klass(&self) -> bool {
        *self == Self::JVM_CONSTANT_Class
    }

    pub fn is_field(&self) -> bool {
        *self == Self::JVM_CONSTANT_Fieldref
    }

    pub fn is_method(&self) -> bool {
        *self == Self::JVM_CONSTANT_Methodref
    }

    pub fn is_interface_method(&self) -> bool {
        *self == Self::JVM_CONSTANT_InterfaceMethodref
    }

    pub fn is_string(&self) -> bool {
        *self == Self::JVM_CONSTANT_String
    }

    pub fn is_int(&self) -> bool {
        *self == Self::JVM_CONSTANT_Integer
    }

    pub fn is_float(&self) -> bool {
        *self == Self::JVM_CONSTANT_Float
    }

    pub fn is_long(&self) -> bool {
        *self == Self::JVM_CONSTANT_Long
    }

    pub fn is_double(&self) -> bool {
        *self == Self::JVM_CONSTANT_Double
    }

    pub fn is_name_and_type(&self) -> bool {
        *self == Self::JVM_CONSTANT_NameAndType
    }

    pub fn is_utf8(&self) -> bool {
        *self == Self::JVM_CONSTANT_Utf8
    }

    pub fn is_invalid(&self) -> bool {
        *self == Self::JVM_CONSTANT_Invalid
    }

    pub fn is_klass_or_klass_reference(&self) -> bool {
        *self == Self::JVM_CONSTANT_Class || *self == Self::JVM_CONSTANT_ClassIndex
    }
}
