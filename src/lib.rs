#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn write_to_open_file(path: &str, data: &[u8]) {
    let path = std::ffi::CString::new(path).expect("Failed to convert path to CString");
    unsafe {
        bindings::WriteToOpenFileC(path.as_ptr(), path.as_bytes().len() as u64, data.as_ptr() as *const i8, data.len() as u64);
    }
}
