extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_container_iterators_0(mut c: ()) {
    let cc: *const () = &c;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_container_iterators_1(mut c: ()) {
    let cc: *const () = &c;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_container_iterators_2(mut c: ()) {
    let cc: *const () = &c;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_container_iterators_3(mut c: ()) {
    let cc: *const () = &c;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_container_iterators_4(mut c: ()) {
    let cc: *const () = &c;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_valarray_iterators_5() {
    let mut v: () = unimplemented!();
    let mut p: *mut () = unimplemented!();
    let mut q: *mut () = unimplemented!();
    unimplemented!();
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
    (unsafe { test_container_iterators_0(unimplemented!()) });
    (unsafe { test_container_iterators_1(unimplemented!()) });
    (unsafe { test_container_iterators_2(unimplemented!()) });
    (unsafe { test_container_iterators_3(unimplemented!()) });
    (unsafe { test_container_iterators_4(unimplemented!()) });
    (unsafe { test_valarray_iterators_5() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
