extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test08_0() {
    let mut buff: [libc::c_char; 2048] = [(0 as libc::c_char); 2048];
    {
        let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(c"Enter name: ".as_ptr()).to_bytes());
        __s
    };
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test08_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
