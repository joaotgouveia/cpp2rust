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
        let mut buf: [libc::c_char; 10] = [(0 as libc::c_char); 10];
        let mut f: () = unimplemented!();
        unimplemented!();
        assert!(!((unimplemented!()).is_null()));
        assert!(unimplemented!());
        unimplemented!();
        let mut p: () = unimplemented!();
        assert!(((unimplemented!()) == (11_())));
        assert!(((unimplemented!()) == (('l' as libc::c_char) as i32)));
        unimplemented!();
        assert!(((unimplemented!()) == (('a' as libc::c_char) as i32)));
        unimplemented!();
        assert!(((unimplemented!()) == (('z' as libc::c_char) as i32)));
        assert!(unimplemented!());
        assert!(((unimplemented!()) == (('l' as libc::c_char) as i32)));
    }
    unimplemented!();
    {
        let mut buf: [i32; 10] = [0_i32; 10];
        let mut f: () = unimplemented!();
        unimplemented!();
        assert!(!((unimplemented!()).is_null()));
        assert!(unimplemented!());
        unimplemented!();
        const _: () = assert!(true, "");;
        let mut p: () = unimplemented!();
        assert!(((unimplemented!()) == (11_())));
        assert!(((unimplemented!()) == ((108 as i32) as ())));
        unimplemented!();
        assert!(((unimplemented!()) == ((97 as i32) as ())));
        unimplemented!();
        assert!(((unimplemented!()) == ((122 as i32) as ())));
        assert!(unimplemented!());
        assert!(((unimplemented!()) == ((108 as i32) as ())));
    }
    unimplemented!();
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
