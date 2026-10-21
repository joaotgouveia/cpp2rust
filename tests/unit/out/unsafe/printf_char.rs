extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut c: () = ('a' as ());
    let mut n: () = 3_();
    (unsafe {
        libc::printf(
            (c"%c\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (c),
        )
    });
    (unsafe {
        libc::printf(
            (c"%d %c\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (n),
            (c),
        )
    });
    (unsafe {
        libc::printf(
            (c"100%% %c\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (c),
        )
    });
    (unsafe {
        libc::printf(
            (c"%c%c%d\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (c),
            ((c) + (1_())),
            (n),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
