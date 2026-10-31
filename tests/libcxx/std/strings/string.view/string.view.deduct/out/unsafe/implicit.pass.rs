extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_view_char__std_char_traits_char____std_basic_string_view_char__std_char_traits_char___
{}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_basic_string_view_wchar_t__std_char_traits_wchar_t____std_basic_string_view_wchar_t__std_char_traits_wchar_t___
{}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut str: *const libc::c_char = c"hello world".as_ptr();
        let mut sv: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut str: *const i32 = [
            104 as i32, 101 as i32, 108 as i32, 108 as i32, 111 as i32, 32 as i32, 119 as i32,
            111 as i32, 114 as i32, 108 as i32, 100 as i32, 0 as i32,
        ]
        .as_ptr();
        let mut sv: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
