use std::ptr;

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "macos")]
extern crate libc;

pub struct MemoryMap {
    ptr: *mut u8,
    size: usize,
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
            MemoryMap { ptr: mem, size }
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
