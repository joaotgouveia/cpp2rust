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
pub struct test_macros_detail_is_same_std_contiguous_iterator_tag__std_contiguous_iterator_tag_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
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
    const _: () = assert!(
        true,
        "(std::is_same<It::difference_type, std::ptrdiff_t>::value)"
    );;
    const _: () = assert!(true, "(std::is_same<It::value_type, A>::value)");;
    const _: () = assert!(true, "(std::is_same<It::pointer, A*>::value)");;
    const _: () = assert!(true, "(std::is_same<It::reference, A&>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<It::iterator_category, std::random_access_iterator_tag>::value)"
    );;
    const _: () = assert!(true, "");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
