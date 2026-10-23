extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_unget_0(mut fb: *mut (), mut expected: ()) {
    let mut c: () = unimplemented!();
    if ((c) != (-1_())) {
        unimplemented!();
        unimplemented!();
    }
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
    let mut stream_buffer: () = unimplemented!();
    let mut fs: () = unimplemented!();
    let mut fb: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut test_buffer: () = unimplemented!();
    (*unimplemented!()) = ('\0' as ());
    {
        unimplemented!();
        unimplemented!();
        (unsafe { check_unget_0(fb, (' ' as ())) });
    }
    {
        unimplemented!();
        unimplemented!();
        (unsafe { check_unget_0(fb, ('o' as ())) });
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { check_unget_0(fb, ('a' as ())) });
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { check_unget_0(fb, ('a' as ())) });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { check_unget_0(fb, (' ' as ())) });
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        (unsafe { check_unget_0(fb, ('r' as ())) });
    }
    {
        let mut buf: [(); 10] = [unimplemented!(); 10];
        let mut ifs: () = unimplemented!();
        let mut filebuf: *mut () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
