use std::ffi::{c_char, c_int, c_longlong, c_ulonglong, c_void, CStr};

use super::jimage_error::JImageError;

#[link(name = "zip")]
extern "C" {
    fn zip_inflate(
        out_buf: *mut c_void,
        out_len: c_longlong,
        in_buf: *mut c_void,
        in_len: c_longlong,
        pmsg: *mut *const c_char,
    ) -> c_int;

    fn zip_deflate(
        out_buf: *mut c_void,
        out_len: c_longlong,
        in_buf: *mut c_void,
        in_len: c_longlong,
        pmsg: *mut *const c_char,
    ) -> c_ulonglong;
}

pub fn inflate(
    out_buf: &mut Vec<u8>,
    out_len: u64,
    in_buf: &mut Vec<u8>,
    in_len: u64,
) -> Result<(), JImageError> {
    let mut pmsg: *const c_char = std::ptr::null();
    unsafe {
        if zip_inflate(
            out_buf.as_mut_ptr() as *mut c_void,
            out_len as c_longlong,
            in_buf.as_mut_ptr() as *mut c_void,
            in_len as c_longlong,
            &mut pmsg,
        ) == 0
        {
            if !pmsg.is_null() {
                return Err(JImageError::DecompressError(
                    CStr::from_ptr(pmsg).to_str().unwrap().to_string(),
                ));
            }
            return Err(JImageError::DecompressError(
                "fail to decomrpess data.".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{c_char, c_longlong, CStr};

    #[test]
    fn we_can_compression_data() {
        let mut in_buf: Vec<u8> = "hello hello hello"
            .to_string()
            .chars()
            .into_iter()
            .map(|c| c as u8)
            .collect();
        let mut out_buf: Vec<u8> = vec![0u8; 16];
        let in_len = in_buf.len() as c_longlong;
        let out_len = in_buf.len() as c_longlong;
        let mut pmsg: *const c_char = std::ptr::null();
        let result = unsafe {
            zip_deflate(
                out_buf.as_mut_ptr() as *mut c_void,
                out_len,
                in_buf.as_mut_ptr() as *mut c_void,
                in_len,
                &mut pmsg,
            )
        };
        assert!(result == 16);
        assert!(
            out_buf == vec![120, 156, 203, 72, 205, 201, 201, 87, 200, 64, 144, 0, 58, 46, 6, 125,]
        )
    }

    #[test]
    fn we_can_decompress_data() {
        let mut in_buf: Vec<u8> = vec![
            120, 156, 203, 72, 205, 201, 201, 87, 200, 64, 144, 0, 58, 46, 6, 125,
        ];
        let mut out_buf: Vec<u8> = vec![0u8; 17];
        let in_len = in_buf.len() as c_longlong;
        let out_len = out_buf.len() as c_longlong;
        let mut pmsg: *const c_char = std::ptr::null();
        let result = unsafe {
            zip_inflate(
                out_buf.as_mut_ptr() as *mut c_void,
                out_len,
                in_buf.as_mut_ptr() as *mut c_void,
                in_len,
                &mut pmsg,
            )
        };
        assert!(result != 0);
        assert!(String::from_utf8(out_buf) == Ok("hello hello hello".to_string()))
    }
}
