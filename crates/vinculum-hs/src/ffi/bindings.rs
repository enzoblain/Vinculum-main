use std::ffi::c_char;
use std::os::raw::c_int;

unsafe extern "C" {
    pub(crate) fn haskell_init(argc: c_int, argv: *mut *mut c_char);
    pub(crate) fn haskell_exit();

    pub(crate) fn call_haskell_function(
        name_ptr: *const u8,
        name_len: usize,
        input_ptr: *const u8,
        input_len: usize,
        out_ptr: *mut *mut u8,
        out_len: *mut usize,
    );

    pub(crate) fn free_haskell_buffer(ptr: *mut u8);
}
