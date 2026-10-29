extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn base_name_0(mut path: *const ()) -> *const () {
    let mut slash: *const () = unimplemented!();
    return if !(slash).is_null() {
        slash.offset((1_()) as isize)
    } else {
        path
    };
}
pub unsafe fn log_1(mut file: *const (), mut line: (), mut func: *const ()) {
    unimplemented!();
}
pub unsafe fn line_2() -> () {
    return line!() as ();
}
pub unsafe fn function_3() -> *const () {
    return c"function".as_ptr();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    (unsafe { log_1(c"macros.cpp".as_ptr(), 20_(), c"main".as_ptr()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
