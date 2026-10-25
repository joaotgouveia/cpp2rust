extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn make_string_0(in_: *const [(); 7]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_1(in_: *const [(); 7]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_2(in_: *const [(); 7]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_3(in_: *const [(); 7]) -> () {
    return unimplemented!();
}
const _: () = assert!(!(false), "!std::copy_constructible<Iter<char>>");
const _: () = assert!(true, "!std::is_copy_assignable_v<Iter<char>>");
const _: () = assert!(true, "std::move_constructible<Iter<char>>");
const _: () = assert!(true, "std::is_move_assignable_v<Iter<char>>");
const _: () = assert!(!(false), "!std::copy_constructible<Iter<wchar_t>>");
const _: () = assert!(true, "!std::is_copy_assignable_v<Iter<wchar_t>>");
const _: () = assert!(true, "std::move_constructible<Iter<wchar_t>>");
const _: () = assert!(true, "std::is_move_assignable_v<Iter<wchar_t>>");
pub unsafe fn test_4() {
    {
        let mut iss: () = (unsafe { make_string_stream_2(&std::mem::transmute(*b"12   3\0")) });
        let mut isv: () = unimplemented!();
        let mut it: () = unimplemented!();
        let mut it2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut iss1: () = (unsafe { make_string_stream_2(&std::mem::transmute(*b"12   3\0")) });
        let mut isv1: () = unimplemented!();
        let mut iss2: () = (unsafe { make_string_stream_2(&std::mem::transmute(*b"45   6\0")) });
        let mut isv2: () = unimplemented!();
        let mut it1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_5() {
    {
        let mut iss: () = (unsafe { make_string_stream_3(&std::mem::transmute(*b"12   3\0")) });
        let mut isv: () = unimplemented!();
        let mut it: () = unimplemented!();
        let mut it2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut iss1: () = (unsafe { make_string_stream_3(&std::mem::transmute(*b"12   3\0")) });
        let mut isv1: () = unimplemented!();
        let mut iss2: () = (unsafe { make_string_stream_3(&std::mem::transmute(*b"45   6\0")) });
        let mut isv2: () = unimplemented!();
        let mut it1: () = unimplemented!();
        unimplemented!();
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
    (unsafe { test_4() });
    (unsafe { test_5() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
