extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn make_string_0(in_: *const [libc::c_char; 7]) -> Vec<libc::c_char> {
    return unimplemented!();
}
pub unsafe fn make_string_1(in_: *const [libc::c_char; 5]) -> Vec<libc::c_char> {
    return unimplemented!();
}
pub unsafe fn make_string_2(in_: *const [libc::c_char; 1]) -> Vec<libc::c_char> {
    return unimplemented!();
}
pub unsafe fn make_string_3(in_: *const [libc::c_char; 7]) -> Vec<i32> {
    return unimplemented!();
}
pub unsafe fn make_string_4(in_: *const [libc::c_char; 5]) -> Vec<i32> {
    return unimplemented!();
}
pub unsafe fn make_string_5(in_: *const [libc::c_char; 1]) -> Vec<i32> {
    return unimplemented!();
}
pub unsafe fn make_string_stream_6(in_: *const [libc::c_char; 7]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_7(in_: *const [libc::c_char; 5]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_8(in_: *const [libc::c_char; 1]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_9(in_: *const [libc::c_char; 7]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_10(in_: *const [libc::c_char; 5]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_11(in_: *const [libc::c_char; 1]) -> () {
    return unimplemented!();
}
pub unsafe fn test_12() {
    {
        let mut iss: () = (unsafe { make_string_stream_6(&std::mem::transmute(*b"a123 4\0")) });
        let mut isv: () = unimplemented!();
        let mut it: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut iss: () = (unsafe { make_string_stream_7(&std::mem::transmute(*b"123 \0")) });
        let mut isv: () = unimplemented!();
        let mut it: () = unimplemented!();
        assert!(it != unimplemented!());
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut iss: () = (unsafe { make_string_stream_8(&[0 as libc::c_char; 1]) });
        let mut isv: () = unimplemented!();
        let mut it: () = unimplemented!();
        assert!(unimplemented!());
    }
}
pub unsafe fn test_13() {
    {
        let mut iss: () = (unsafe { make_string_stream_9(&std::mem::transmute(*b"a123 4\0")) });
        let mut isv: () = unimplemented!();
        let mut it: () = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut iss: () = (unsafe { make_string_stream_10(&std::mem::transmute(*b"123 \0")) });
        let mut isv: () = unimplemented!();
        let mut it: () = unimplemented!();
        assert!(it != unimplemented!());
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut iss: () = (unsafe { make_string_stream_11(&[0 as libc::c_char; 1]) });
        let mut isv: () = unimplemented!();
        let mut it: () = unimplemented!();
        assert!(unimplemented!());
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
    (unsafe { test_12() });
    (unsafe { test_13() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
