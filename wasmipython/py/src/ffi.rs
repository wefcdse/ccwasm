mod ffi_inner {
    #[link(wasm_import_module = "host")]
    extern "C" {
        pub fn export_string(addr: i32, len: i32);
        pub fn import_string_length() -> i32;
        pub fn import_string_data(addr: i32);
    }
}
pub fn export_str(s: &str) {
    unsafe {
        ffi_inner::export_string(addrof(s), s.len() as i32);
    }
}
pub fn addrof<T: ?Sized>(s: *const T) -> i32 {
    s as *const () as usize as i32
}
pub fn import_string() -> String {
    let mut a = vec![0u8; unsafe { ffi_inner::import_string_length() } as usize];
    unsafe {
        let addr = a.as_mut_ptr();
        ffi_inner::import_string_data(addr as *const u8 as usize as i32);
    }
    String::from_utf8(a).unwrap()
}
