extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_layout_mapping_stride_0(mut ext: (), mut strides: (), mut exhaustive: ()) {
    let mut m: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_layout_mapping_stride_1(mut ext: (), mut strides: (), mut exhaustive: ()) {
    let mut m: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_layout_mapping_stride_2(mut ext: (), mut strides: (), mut exhaustive: ()) {
    let mut m: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_layout_mapping_stride_3(mut ext: (), mut strides: (), mut exhaustive: ()) {
    let mut m: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_layout_mapping_stride_4(mut ext: (), mut strides: (), mut exhaustive: ()) {
    let mut m: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_layout_mapping_stride_5(mut ext: (), mut strides: (), mut exhaustive: ()) {
    let mut m: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_6() -> () {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_7));
    (unsafe { test_layout_mapping_stride_0(unimplemented!(), unimplemented!(), true) });
    (unsafe { test_layout_mapping_stride_1(unimplemented!(), unimplemented!(), true) });
    (unsafe { test_layout_mapping_stride_2(unimplemented!(), unimplemented!(), true) });
    (unsafe { test_layout_mapping_stride_3(unimplemented!(), unimplemented!(), true) });
    (unsafe { test_layout_mapping_stride_3(unimplemented!(), unimplemented!(), true) });
    (unsafe { test_layout_mapping_stride_4(unimplemented!(), unimplemented!(), true) });
    (unsafe { test_layout_mapping_stride_5(unimplemented!(), unimplemented!(), true) });
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
    (unsafe { test_6() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
