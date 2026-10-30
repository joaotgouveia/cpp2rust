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
unsafe fn main_0() -> i32 {
    let mut cond: bool = true;
    let mut os1: *mut ::std::fs::File = if cond {
        (&raw mut (*libcc2rs::cout_unsafe()) as *mut ::std::fs::File)
    } else {
        (&raw mut (*libcc2rs::cerr_unsafe()) as *mut ::std::fs::File)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*os1);
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(c"hello\n".as_ptr()).to_bytes());
        __s
    };
    let os2: *mut ::std::fs::File = if cond {
        (&raw mut (*libcc2rs::cout_unsafe()) as *mut ::std::fs::File)
    } else {
        (&raw mut (*libcc2rs::cerr_unsafe()) as *mut ::std::fs::File)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*os2);
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(c"hello\n".as_ptr()).to_bytes());
        __s
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
