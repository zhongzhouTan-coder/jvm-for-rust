#[cfg(not(windows))]
use std::ffi::{c_char, c_int, c_void, CString};

#[cfg(windows)]
use std::os::windows::prelude::*;


fn find_entry(name: &str) -> Option<*mut c_void> {
    #[cfg(windows)]
    {
        #[link(name = "kernel32")]
        extern "system" {
            fn GetModuleHandleW(lpModuleName: *const u16) -> *mut c_void;

            fn LoadLibraryW(lpFileName: *const u16) -> *mut c_void;

            fn GetProcAddress(hModule: *mut c_void, procName: *const u8) -> *mut c_void;
        }

        let module_name = "C:\\Windows\\SysWOW64\\zipfldr.dll";
        let module_name_wide = OsString::from(module_name)
            .encode_wide()
            .collect::<Vec<_>>();
        let mut module_handle = unsafe { GetModuleHandleW(module_name_wide.as_ptr()) };
        if module_handle.is_null() {
            module_handle = unsafe { LoadLibraryW(module_name_wide.as_ptr()) };
        }

        if module_handle.is_null() {
            println!("fail to load function");
            return None;
        }

        unsafe { Some(GetProcAddress(module_handle, name.as_ptr())) }
    }

    #[cfg(not(windows))]
    {
        extern "C" {
            fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;

            fn dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
        }

        let name = CString::new(name).unwrap();

        let lib = if cfg!(linux) {
            CString::new("libzip.so").unwrap()
        } else {
            CString::new("libzip.dylib").unwrap()
        };
        let addr = unsafe { dlopen(lib.as_ptr(), libc::RTLD_LAZY | libc::RTLD_GLOBAL) };
        let result = unsafe { dlsym(addr, name.as_ptr()) };
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::find_entry;

    #[test]
    fn load_function_from_zip_dll() {
        let func_ptr = find_entry("ZIP_InflateFully");
        assert!(func_ptr.is_some())
    }
}
