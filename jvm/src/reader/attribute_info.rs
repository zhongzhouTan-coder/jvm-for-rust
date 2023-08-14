use std::vec;

use super::{attribute::Attribute, constant_info::ConstantInfo, exception_table::ExceptionTable};

pub trait AttributeInfo {}

pub struct ConstantValueAttribute {
    constant_value_index: u16,
}

pub struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    code_length: u32,
    code: Vec<u8>,
    exception_table_length: u16,
    exception_table: ExceptionTable,
    attributes_count: u16,
    attributes: Vec<Attribute>,
}

pub struct StackMapTableAttribute {
    number_of_entries: u16,
    entries: Vec<StackMapFrame>,
}

#[repr(u8)]
pub enum StackMapFrame {
    SameFrame(u8),
    SameLocals1StackItemFrame(u8, VerificationTypeInfo),
    SameLocals1StackItemFrameExtended(u8, u16, VerificationTypeInfo),
    ChopFrame(u8, u16),
    SameFrameExtend(u8, u16),
    AppendFrame(u8, u16, Vec<VerificationTypeInfo>),
    FullFrame(
        u8,
        u16,
        u16,
        Vec<VerificationTypeInfo>,
        u16,
        Vec<VerificationTypeInfo>,
    ),
}

pub enum VerificationTypeInfo {
    TopVariableInfo(u8),
    IntegerVariableInfo(u8),
    FloatVariableInfo(u8),
    NullVariableInfo(u8),
    UninitializedThisVariableInfo(u8),
    ObjectVariableInfo(u8, u16),
    UninitializedVariableInfo(u8, u16),
    LongVariableInfo(u8),
    DoubleVariableInfo(u8),
}

pub struct ExceptionsAttribute {
    number_of_exceptions: u16,
    exception_index_table: Vec<u16>,
}

pub struct InnerClassesAttribute {
    number_of_classes: u16,
    classes: Vec<Classes>,
}

pub struct Classes {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}

pub struct EnclosingMethodAttribute {
    class_index: u16,
    method_index: u16,
}

pub struct SyntheticAttribute {}

pub struct SignatureAttribute {
    signature_index: u16,
}

pub struct SourceFileAttribute {
    sourcefile_index: u16,
}

pub struct SourceDebugExtensionAttribute {
    debug_extension: Vec<u8>,
}

pub struct LineNumberTableAttribute {
    line_number_table_length: u16,
    line_number_table: Vec<LineNumberTable>,
}

pub struct LineNumberTable {
    start_pc: u16,
    line_number: u16,
}

pub struct LocalVariableTableAttribute {
    local_variable_table_length: u16,
    local_variable_table: Vec<LocalVariableTable>,
}

pub struct LocalVariableTable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

pub struct LocalVariableTypeTableAttribute {
    local_variable_type_table_length: u16,
    local_variable_type_table: Vec<LocalVariableTypeTable>,
}

pub struct LocalVariableTypeTable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

pub struct DeprecatedAttribute {}

pub struct RuntimeVisibleAnnotationsAttribute {
    num_annotations: u16,
}

pub struct Annotation {
    type_index: u16,
    num_element_value_pairs: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

pub struct ElementValuePair {
    element_name_index: u16,
    value: u16,
}

pub struct ElementValue {
    tag: u8,
    value: Value,
}

pub struct Value {
    const_value_index: u16,
    enum_const_value: EnumConstValue,
    class_info_index: u16,
    annotation_value: Annotation,
    array_value: ArrayValue,
}

pub struct EnumConstValue {
    type_name_index: u16,
    const_name_index: u16,
}

pub struct ArrayValue {
    num_values: u16,
    values: Vec<ElementValue>,
}

pub struct RuntimeInvisibleAnnotationsAttribute {
    num_annotations: u16,
    annotations: Vec<Annotation>,
}

pub struct RuntimeVisibleParameterAnnotationsAttribute {
    num_parameters: u8,
    parameter_annotations: Vec<ParameterAnnotation>,
}

pub struct ParameterAnnotation {
    num_annotations: u16,
    annotations: Vec<Annotation>,
}

pub struct RuntimeInvisibleParameterAnnotationsAttribute {
    num_parameters: u8,
    parameter_annotations: Vec<ParameterAnnotation>,
}

pub struct RuntimeVisibleTypeAnnotationsAttribute {
    num_annotations: u16,
    annotations: Vec<TypeAnnotation>,
}

pub struct TypeAnnotation {
    target_type: u8,
    target_info: TargetInfo,
    target_path: TypePath,
    type_index: u16,
    num_element_value_pairs: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

pub enum TargetInfo {
    TypeParameterTarget(u8),
    SuperTypeTarget(u16),
    TypeParameterBoundTarget(u8, u8),
    EmptyTarget,
    FormalParameterTarget(u8),
    ThrowsTarget(u16),
    LocalVarTarget(u16, Table),
    CatchTarget(u16),
    OffsetTarget(u16),
    TypeArgumentTarget(u16, u8),
}

pub struct Table {
    start_pc: u16,
    length: u16,
    index: u16,
}

pub struct TypePath {
    path_length: u8,
    paths: Vec<Path>,
}

pub struct Path {
    type_path_kind: u8,
    type_argument_index: u8,
}

pub struct RuntimeInvisibleTypeAnnotationsAttribute {
    num_annotations: u16,
    annotations: Vec<TypeAnnotation>,
}

pub struct AnnotationDefaultAttribute {
    default_value: ElementValue,
}

pub struct BootstrapMethodsAttribute {
    num_bootstrap_methods: u16,
    bootstrap_methods: Vec<BootstrapMethod>,
}

pub struct BootstrapMethod {
    bootstrap_method_ref: u16,
    num_bootstrap_arguments: u16,
    bootstrap_arguments: Vec<u16>,
}

pub struct MethodParametersAttribute {
    parameters_count: u8,
    parameters: Vec<Parameter>,
}

pub struct Parameter {
    name_index: u16,
    access_flags: u16,
}

pub struct ModuleAttribute {
    module_name_index: u16,
    module_flags: u16,
    module_version_index: u16,
    requires_count: u16,
    requires: Vec<Require>,
    exports_count: u16,
    exports: Vec<Export>,
    opens_count: u16,
    opens: Vec<Open>,
    uses_count: u16,
    uses_index: Vec<u16>,
    provides_count: u16,
    provides: Vec<Provide>,
}

pub struct Require {
    requires_index: u16,
    requires_flags: u16,
    requires_version_index: u16,
}

pub struct Export {
    exports_index: u16,
    exports_flags: u16,
    exports_to_count: u16,
    exports_to_index: Vec<u16>,
}

pub struct Open {
    opens_index: u16,
    opens_flags: u16,
    opens_to_count: u16,
    opens_to_index: Vec<u16>,
}

pub struct Provide {
    provides_index: u16,
    provides_with_count: u16,
    provides_with_index: Vec<u16>,
}

pub struct ModulePackagesAttribute {
    package_count: u16,
    package_index: u16,
}

pub struct ModuleMainClassAttribute {
    main_class_index: u16,
}

pub struct NestHostAttribute {
    host_class_index: u16,
}

pub struct NestMembersAttribute {
    number_of_classes: u16,
    classes: Vec<u16>,
}

pub struct RecordAttribute {
    components_count: u16,
    components: Vec<RecordComponentInfo>,
}

pub struct RecordComponentInfo {
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<Attribute>,
}

pub struct PermittedSubclassesAttribute {
    number_of_classes: u16,
    classes: Vec<u16>,
}
