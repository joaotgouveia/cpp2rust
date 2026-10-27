extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn search_0(mut pat: *const (), mut f: (), mut target: *const (), mut expected: ()) {
    let mut re: () = unimplemented!();
    let mut m: () = unimplemented!();
    unimplemented!();
    if expected {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    } else {
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
    {
        let mut pat: *const () = c"^foo".as_ptr();
        let mut target: *const () = c"foo".as_ptr();
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
    }
    {
        let mut pat: *const () = c"^foo".as_ptr();
        let mut target: *const () = c"\nfoo".as_ptr();
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
    }
    {
        let mut pat: *const () = c"^foo".as_ptr();
        let mut target: *const () = c"bar\nfoo".as_ptr();
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
    }
    {
        let mut pat: *const () = c"foo$".as_ptr();
        let mut target: *const () = c"foo".as_ptr();
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
    }
    {
        let mut pat: *const () = c"foo$".as_ptr();
        let mut target: *const () = c"foo\n".as_ptr();
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
    }
    {
        let mut pat: *const () = c"foo$".as_ptr();
        let mut target: *const () = c"foo\nbar".as_ptr();
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
    }
    {
        let mut pat: *const () = c"^foo".as_ptr();
        let mut target: *const () = c"foo".as_ptr();
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
    }
    {
        let mut pat: *const () = c"^foo".as_ptr();
        let mut target: *const () = c"\rfoo".as_ptr();
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
    }
    {
        let mut pat: *const () = c"^foo".as_ptr();
        let mut target: *const () = c"bar\rfoo".as_ptr();
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
    }
    {
        let mut pat: *const () = c"foo$".as_ptr();
        let mut target: *const () = c"foo".as_ptr();
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
    }
    {
        let mut pat: *const () = c"foo$".as_ptr();
        let mut target: *const () = c"foo\r".as_ptr();
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
    }
    {
        let mut pat: *const () = c"foo$".as_ptr();
        let mut target: *const () = c"foo\rbar".as_ptr();
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, true) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
        (unsafe { search_0(pat, unimplemented!(), target, false) });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
