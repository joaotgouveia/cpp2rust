extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn base_name_0(mut path: *const ()) -> *const () {
    let mut slash: *const () = (unsafe { strrchr_1(path, ('/' as ())) });
    return if !(slash).is_null() {
        slash.offset((1_()) as isize)
    } else {
        path
    };
}
pub unsafe fn log_2(mut file: *const (), mut line: (), mut func: *const ()) {
    (unsafe {
        libc::printf(
            c"%s %d %s\n".as_ptr() as *const libc::c_char,
            (file),
            (line),
            (func),
        )
    });
}
pub unsafe fn line_3() -> () {
    return line!() as ();
}
pub unsafe fn function_4() -> *const () {
    return c"function".as_ptr();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe {
        libc::printf(
            c"%s %d %s\n".as_ptr() as *const libc::c_char,
            (c"macros.cpp".as_ptr()),
            (19_()),
            (c"main".as_ptr()),
        )
    });
    (unsafe { log_2(c"macros.cpp".as_ptr(), 20_(), c"main".as_ptr()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe {
        libc::printf(
            c"%s\n".as_ptr() as *const libc::c_char,
            (unsafe { function_4() }),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
