use crate::reader::buffer::Buffer;

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};

pub struct StackMapTableAttribute {
    number_of_entries: u16,
    entries: Vec<StackMapFrame>,
}

impl AttributeTrait for StackMapTableAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let number_of_entries = buffer.read_u16()?;
        let mut entries = Vec::new();
        for _ in 0..number_of_entries {
            let frame_type = FrameType::from_u8(buffer.read_u8()?);
            let entry: StackMapFrame = match frame_type {
                FrameType::Same(_) => StackMapFrame::SameFrame(frame_type),
                FrameType::SameLocals1StackItem(_) => {
                    let stack = VerificationTypeInfo::decode_attribute(buffer)?;
                    StackMapFrame::SameLocals1StackItemFrame(frame_type, stack)
                }
                FrameType::SameLocals1StackItemExtended(_) => {
                    let offset_delta = buffer.read_u16()?;
                    let stack = VerificationTypeInfo::decode_attribute(buffer)?;
                    StackMapFrame::SameLocals1StackItemFrameExtended(
                        frame_type,
                        offset_delta,
                        stack,
                    )
                }
                FrameType::Chop(_) => {
                    let offset_delta = buffer.read_u16()?;
                    StackMapFrame::ChopFrame(frame_type, offset_delta)
                }
                FrameType::SameFrameExtended(_) => {
                    let offset_delta = buffer.read_u16()?;
                    StackMapFrame::SameFrameExtend(frame_type, offset_delta)
                }
                FrameType::Append(value) => {
                    let offset_delta = buffer.read_u16()?;
                    let locals = (0..value - 251)
                        .map(|_| VerificationTypeInfo::decode_attribute(buffer))
                        .collect::<Result<Vec<VerificationTypeInfo>, AttributeError>>()?;
                    StackMapFrame::AppendFrame(frame_type, offset_delta, locals)
                }
                FrameType::FullFrame(_) => {
                    let offset_delta = buffer.read_u16()?;
                    let number_of_locals = buffer.read_u16()?;
                    let locals = (0..number_of_locals)
                        .map(|_| VerificationTypeInfo::decode_attribute(buffer))
                        .collect::<Result<Vec<VerificationTypeInfo>, AttributeError>>()?;
                    let number_of_stack_items = buffer.read_u16()?;
                    let stack = (0..number_of_stack_items)
                        .map(|_| VerificationTypeInfo::decode_attribute(buffer))
                        .collect::<Result<Vec<VerificationTypeInfo>, AttributeError>>()?;
                    StackMapFrame::FullFrame(
                        frame_type,
                        offset_delta,
                        number_of_locals,
                        locals,
                        number_of_stack_items,
                        stack,
                    )
                }
            };
            entries.push(entry);
        }
        Ok(StackMapTableAttribute {
            number_of_entries,
            entries,
        })
    }
}

pub enum StackMapFrame {
    SameFrame(FrameType),
    SameLocals1StackItemFrame(FrameType, VerificationTypeInfo),
    SameLocals1StackItemFrameExtended(FrameType, u16, VerificationTypeInfo),
    ChopFrame(FrameType, u16),
    SameFrameExtend(FrameType, u16),
    AppendFrame(FrameType, u16, Vec<VerificationTypeInfo>),
    FullFrame(
        FrameType,
        u16,
        u16,
        Vec<VerificationTypeInfo>,
        u16,
        Vec<VerificationTypeInfo>,
    ),
}

pub enum FrameType {
    Same(u8),
    SameLocals1StackItem(u8),
    SameLocals1StackItemExtended(u8),
    Chop(u8),
    SameFrameExtended(u8),
    Append(u8),
    FullFrame(u8),
}

impl FrameType {
    pub fn from_u8(value: u8) -> Self {
        if value <= 63 {
            Self::Same(value)
        } else if value <= 127 {
            Self::SameLocals1StackItem(value)
        } else if value == 247 {
            Self::SameLocals1StackItemExtended(value)
        } else if value >= 248 && value <= 250 {
            Self::Chop(value)
        } else if value == 251 {
            Self::SameFrameExtended(value)
        } else if value >= 252 && value <= 254 {
            Self::Append(value)
        } else if value == 255 {
            Self::FullFrame(value)
        } else {
            panic!("unsupported frame type -- [{}].", value)
        }
    }
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

impl AttributeTrait for VerificationTypeInfo {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let tag = buffer.read_u8()?;
        match tag {
            0 => Ok(Self::TopVariableInfo(0)),
            1 => Ok(Self::IntegerVariableInfo(1)),
            2 => Ok(Self::FloatVariableInfo(2)),
            5 => Ok(Self::NullVariableInfo(5)),
            6 => Ok(Self::UninitializedThisVariableInfo(6)),
            7 => {
                let cpool_index = buffer.read_u16()?;
                Ok(Self::ObjectVariableInfo(7, cpool_index))
            }
            8 => {
                let offset = buffer.read_u16()?;
                Ok(Self::UninitializedVariableInfo(8, offset))
            }
            4 => Ok(Self::LongVariableInfo(4)),
            3 => Ok(Self::DoubleVariableInfo(3)),
            _ => panic!("unsupported verification type info tag"),
        }
    }
}