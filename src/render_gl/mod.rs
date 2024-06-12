use std::ffi::CString;

pub mod data;
pub mod shader;
pub mod program;

fn create_whitespace_cstring_with_length(length: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(length + 1);
    buffer.extend(
        [b' ']
        .iter()
        .cycle()
        .take(length as usize)
    );
    unsafe {
        CString::from_vec_unchecked(buffer)
    }
}
