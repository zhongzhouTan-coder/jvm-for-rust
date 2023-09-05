#[derive(Debug)]
pub struct Attribute {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}

impl Attribute {
    pub fn new(attribute_name_index: u16, attribute_length: u32, info: Vec<u8>) -> Attribute {
        Attribute {
            attribute_name_index,
            attribute_length,
            info,
        }
    }
}

#[derive(Debug)]
pub enum AttributeName {
    ConstantValue,
    Code,
    Exceptions,
    SourceFile,
    LineNumberTable,
    LocalVariableTable,
    InnerClasses,
    Synthetic,
    Deprecated,
    EnclosingMethod,
    Signature,
    SourceDebugExtension,
    LocalVariableTypeTable,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    StackMapTable,
    BootstrapMethods,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    MethodParameters,
    Module,
    ModulePackages,
    ModuleMainClass,
    NestHost,
    NestMembers,
    Record,
    PermittedSubclasses,
}

impl AttributeName {
    pub fn from_string(name: &String) -> Self {
        let attribute_name = name.as_str();
        match attribute_name {
            "ConstantValue" => Self::ConstantValue,
            "Code" => Self::Code,
            "Exceptions" => Self::Exceptions,
            "SourceFile" => Self::SourceFile,
            "LineNumberTable" => Self::LineNumberTable,
            "LocalVariableTable" => Self::LocalVariableTable,
            "InnerClasses" => Self::InnerClasses,
            "Synthetic" => Self::Synthetic,
            "Deprecated" => Self::Deprecated,
            "EnclosingMethod" => Self::EnclosingMethod,
            "Signature" => Self::Signature,
            "SourceDebugExtension" => Self::SourceDebugExtension,
            "LocalVariableTypeTable" => Self::LocalVariableTypeTable,
            "RuntimeVisibleAnnotations" => Self::RuntimeVisibleAnnotations,
            "RuntimeInvisibleAnnotations" => Self::RuntimeInvisibleAnnotations,
            "RuntimeVisibleParameterAnnotations" => Self::RuntimeVisibleParameterAnnotations,
            "RuntimeInvisibleParameterAnnotations" => Self::RuntimeInvisibleParameterAnnotations,
            "AnnotationDefault" => Self::AnnotationDefault,
            "StackMapTable" => Self::StackMapTable,
            "BootstrapMethods" => Self::BootstrapMethods,
            "RuntimeVisibleTypeAnnotations" => Self::RuntimeVisibleTypeAnnotations,
            "RuntimeInvisibleTypeAnnotations" => Self::RuntimeInvisibleTypeAnnotations,
            "MethodParameters" => Self::MethodParameters,
            "Module" => Self::Module,
            "ModulePackages" => Self::ModulePackages,
            "ModuleMainClass" => Self::ModuleMainClass,
            "NestHost" => Self::NestHost,
            "NestMembers" => Self::NestMembers,
            "Record" => Self::Record,
            "PermittedSubclasses" => Self::PermittedSubclasses,
            _ => panic!("Invalid attribute name - {}.", attribute_name),
        }
    }
}
