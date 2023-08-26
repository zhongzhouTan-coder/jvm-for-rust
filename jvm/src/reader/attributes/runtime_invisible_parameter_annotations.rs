use super::parameter_annotation::ParameterAnnotation;

pub struct RuntimeInvisibleParameterAnnotationsAttribute {
    num_parameters: u8,
    parameter_annotations: Vec<ParameterAnnotation>,
}
