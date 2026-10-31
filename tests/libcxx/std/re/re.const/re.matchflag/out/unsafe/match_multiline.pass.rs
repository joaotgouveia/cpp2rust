extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn search_0(
    mut pat: *const libc::c_char,
    mut f: (),
    mut target: *const libc::c_char,
    mut expected: bool,
) {
    let mut re: () = unimplemented!();
    let mut m: () = unimplemented!();
    assert!(((unimplemented!() as i32) == (expected as i32)));
    if expected {
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (3_())));
        assert!(
            unimplemented!()
                == std::slice::from_raw_parts(
                    c"foo".as_ptr(),
                    libcc2rs::c_str_len(c"foo".as_ptr()) + 1
                )
                .to_vec()
        );
    } else {
        assert!(((unimplemented!()) == (0_())));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut pat: *const libc::c_char = c"^foo".as_ptr();
        let mut target: *const libc::c_char = c"foo".as_ptr();
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
        let mut pat: *const libc::c_char = c"^foo".as_ptr();
        let mut target: *const libc::c_char = c"\nfoo".as_ptr();
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
        let mut pat: *const libc::c_char = c"^foo".as_ptr();
        let mut target: *const libc::c_char = c"bar\nfoo".as_ptr();
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
        let mut pat: *const libc::c_char = c"foo$".as_ptr();
        let mut target: *const libc::c_char = c"foo".as_ptr();
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
        let mut pat: *const libc::c_char = c"foo$".as_ptr();
        let mut target: *const libc::c_char = c"foo\n".as_ptr();
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
        let mut pat: *const libc::c_char = c"foo$".as_ptr();
        let mut target: *const libc::c_char = c"foo\nbar".as_ptr();
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
        let mut pat: *const libc::c_char = c"^foo".as_ptr();
        let mut target: *const libc::c_char = c"foo".as_ptr();
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
        let mut pat: *const libc::c_char = c"^foo".as_ptr();
        let mut target: *const libc::c_char = c"\rfoo".as_ptr();
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
        let mut pat: *const libc::c_char = c"^foo".as_ptr();
        let mut target: *const libc::c_char = c"bar\rfoo".as_ptr();
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
        let mut pat: *const libc::c_char = c"foo$".as_ptr();
        let mut target: *const libc::c_char = c"foo".as_ptr();
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
        let mut pat: *const libc::c_char = c"foo$".as_ptr();
        let mut target: *const libc::c_char = c"foo\r".as_ptr();
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
        let mut pat: *const libc::c_char = c"foo$".as_ptr();
        let mut target: *const libc::c_char = c"foo\rbar".as_ptr();
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
