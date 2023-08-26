use super::parameter_annotation::ParameterAnnotation;

pub struct RuntimeVisibleParameterAnnotationsAttribute {
    num_parameters: u8,
    parameter_annotations: Vec<ParameterAnnotation>,
}
