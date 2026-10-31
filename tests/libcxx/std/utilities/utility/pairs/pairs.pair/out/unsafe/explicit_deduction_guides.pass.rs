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
pub struct test_macros_detail_is_same_std_pair_char_ptr__void__ptr______std_pair_char_ptr__void__ptr_____
{}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_pair_std_reference_wrapper_int___void__ptr______std_pair_std_reference_wrapper_int___void__ptr_____
{}
pub unsafe fn dummy_2() {}
pub unsafe fn test_decay_3() {
    let mut arr: [libc::c_char; 1] = [(0 as libc::c_char)];
    let mut pr: (*mut libc::c_char, Option<unsafe fn()>) = unimplemented!();
    const _: () = assert!(true, "");;
    assert!(unimplemented!());
}
pub unsafe fn test_unwrap_4() {
    let mut n: i32 = 0;
    let mut pr: (*mut i32, Option<unsafe fn()>) = (unimplemented!(), Some(dummy_2));
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "!std::is_same_v<decltype(pr), decltype(std::make_pair(std::ref(n), dummy))>"
    );;
    assert!(((&mut (*pr.0) as *mut i32) == (&mut n as *mut i32)));
    assert!(((pr.1) == (Some(dummy_2))));
}
pub unsafe fn test_5() -> bool {
    (unsafe { test_decay_3() });
    {
        (unsafe { test_unwrap_4() });
    }
    return true;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_5() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
