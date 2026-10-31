extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut fs: ::std::fs::File = unimplemented!();
        assert!(!(unimplemented!()));
        let mut c: libc::c_char = ('a' as libc::c_char);
        unimplemented!();
        assert!(unimplemented!());
        assert!(((c as i32) == (('a' as libc::c_char) as i32)));
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
        assert!(((c as i32) == (('r' as libc::c_char) as i32)));
    }
    {
        let mut fs: () = unimplemented!();
        assert!(!(unimplemented!()));
        let mut c: i32 = (97 as i32);
        unimplemented!();
        assert!(unimplemented!());
        assert!(((c) == (97 as i32)));
        unimplemented!();
        assert!(unimplemented!());
        unimplemented!();
        assert!(((c) == (114 as i32)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
