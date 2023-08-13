pub trait ToUsizeSafe {
    fn into_usize_safe(self) -> usize;
}

impl ToUsizeSafe for u8 {
    fn into_usize_safe(self) -> usize {
        usize::try_from(self).expect("fail to convert u8 to usize.")
    }
}

impl ToUsizeSafe for u16 {
    fn into_usize_safe(self) -> usize {
        usize::try_from(self).expect("fail to convert u16 to usize.")
    }
}

impl ToUsizeSafe for u32 {
    fn into_usize_safe(self) -> usize {
        usize::try_from(self).expect("fail to convert u32 to usize.")
    }
}
