use std::mem::size_of;

use super::address::Address;

pub struct LineMap {
    base: Address,
    len: usize,
}

impl LineMap {
    pub fn new(line_size: usize) -> LineMap {
        let base = unsafe { libc::malloc(line_size * size_of::<u8>()) };
        LineMap {
            base: Address::new(base as usize),
            len: line_size,
        }
    }
}

impl Drop for LineMap {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.base.to_usize() as *mut libc::c_void);
        }
    }
}
