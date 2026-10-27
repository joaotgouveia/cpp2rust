extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_required_span_size_0(mut e: (), mut strides: (), mut expected_size: ()) {
    let m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_required_span_size_1(mut e: (), mut strides: (), mut expected_size: ()) {
    let m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_required_span_size_2(mut e: (), mut strides: (), mut expected_size: ()) {
    let m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_required_span_size_3(mut e: (), mut strides: (), mut expected_size: ()) {
    let m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_required_span_size_4(mut e: (), mut strides: (), mut expected_size: ()) {
    let m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_required_span_size_5(mut e: (), mut strides: (), mut expected_size: ()) {
    let m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_required_span_size_6(mut e: (), mut strides: (), mut expected_size: ()) {
    let m: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn test_7() -> () {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_8));
    (unsafe { test_required_span_size_0(unimplemented!(), unimplemented!(), 1_()) });
    (unsafe { test_required_span_size_1(unimplemented!(), unimplemented!(), 0_()) });
    (unsafe { test_required_span_size_1(unimplemented!(), unimplemented!(), 1_()) });
    (unsafe { test_required_span_size_1(unimplemented!(), unimplemented!(), 31_()) });
    (unsafe { test_required_span_size_2(unimplemented!(), unimplemented!(), 31_()) });
    (unsafe { test_required_span_size_3(unimplemented!(), unimplemented!(), 135_()) });
    (unsafe { test_required_span_size_4(unimplemented!(), unimplemented!(), 5040_()) });
    (unsafe { test_required_span_size_5(unimplemented!(), unimplemented!(), 5034_()) });
    (unsafe { test_required_span_size_6(unimplemented!(), unimplemented!(), 0_()) });
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_7() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
