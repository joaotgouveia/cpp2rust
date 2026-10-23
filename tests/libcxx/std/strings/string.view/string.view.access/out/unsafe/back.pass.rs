extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char_ref__const_char_ref_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_wchar_t_ref__const_wchar_t_ref_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char16_t_ref__const_char16_t_ref_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char32_t_ref__const_char32_t_ref_ {}
pub unsafe fn test_4(mut s: *const (), mut len: ()) -> () {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    return ((unimplemented!()) == (s.offset((len) as isize).offset(-((1_()) as isize))));
}
pub unsafe fn test_5(mut s: *const (), mut len: ()) -> () {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    return ((unimplemented!()) == (s.offset((len) as isize).offset(-((1_()) as isize))));
}
pub unsafe fn test_6(mut s: *const (), mut len: ()) -> () {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    return ((unimplemented!()) == (s.offset((len) as isize).offset(-((1_()) as isize))));
}
pub unsafe fn test_7(mut s: *const (), mut len: ()) -> () {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    return ((unimplemented!()) == (s.offset((len) as isize).offset(-((1_()) as isize))));
}
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let sv: () = unimplemented!();
        const _: () = assert!(true, "sv.length() == 2");;
        const _: () = assert!(true, "sv.back() == \'B\'");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
