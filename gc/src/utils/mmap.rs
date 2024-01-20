use std::ptr;

use crate::{
    align_up,
    model::{address::Address, block::BLOCK_SIZE},
};

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "macos")]
extern crate libc;

pub struct MemoryMap {
    base: *mut u8,
    ptr: *mut u8,
    end: *mut u8,
}

unsafe impl Send for MemoryMap {}

#[cfg(target_os = "windows")]
impl MemoryMap {
    pub fn new(size: usize) -> MemoryMap {
        unsafe {
            let mem = winapi::um::memoryapi::VirtualAlloc(
                ptr::null_mut(),
                size,
                winapi::um::winnt::MEM_RESERVE,
                winapi::um::winnt::PAGE_READWRITE,
            );
            let mem = mem as *mut u8;
            if mem.is_null() {
                panic!("VirtualAlloc failed");
            }
            let ptr: *mut u8 = align_up!(mem as usize, BLOCK_SIZE) as *mut u8;
            MemoryMap {
                base: mem,
                ptr,
                end: mem.add(size),
            }
        }
    }

    pub fn allocate_memory(&mut self, size: usize) -> Option<Address> {
        unsafe {
            if self.ptr.add(size) <= self.end {
                return None;
            }
            let mem = winapi::um::memoryapi::VirtualAlloc(
                self.ptr.cast(),
                size,
                winapi::um::winnt::MEM_COMMIT,
                winapi::um::winnt::PAGE_READWRITE,
            );
            let mem = mem as *mut u8;
            if mem.is_null() {
                None
            } else {
                self.ptr = mem.add(size);
                Some(Address::new(mem as usize))
            }
        }
    }
}

#[cfg(target_os = "windows")]
impl Drop for MemoryMap {
    fn drop(&mut self) {
        unsafe {
            winapi::um::memoryapi::VirtualFree(self.base.cast(), 0, winapi::um::winnt::MEM_RELEASE);
        }
    }
}

#[cfg(target_os = "macos")]
impl MemoryMap {
    pub fn new(size: usize) -> MemoryMap {
        unsafe {
            let mem = libc::mmap(
                ptr::null_mut(),
                size,
                libc::PROT_NONE,
                libc::MAP_ANON | libc::MAP_PRIVATE,
                -1,
                0,
            );
            let mem = mem as *mut u8;
            if mem.is_null() {
                panic!("mmap failed");
            }
            MemoryMap { ptr: mem, size }
        }
    }
}
