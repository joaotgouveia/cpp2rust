extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn make_string_0(in_: *const [(); 17]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_1(in_: *const [(); 2]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_2(in_: *const [(); 11]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_3(in_: *const [(); 17]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_4(in_: *const [(); 2]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_5(in_: *const [(); 11]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_6(in_: *const [(); 17]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_7(in_: *const [(); 17]) -> () {
    return unimplemented!();
}
pub unsafe fn test_8() {
    let mut ints: () =
        (unsafe { make_string_stream_6(&std::mem::transmute(*b"0 1  2   3     4\0")) });
    let mut oss: () = unimplemented!();
    let mut delimiter: () = (unsafe { make_string_1(&std::mem::transmute(*b"-\0")) });
    (unsafe {
        let mut ___r: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_9))(&mut ___r, unimplemented!())
    });
    let mut expected: () = (unsafe { make_string_2(&std::mem::transmute(*b"0-1-2-3-4-\0")) });
    unimplemented!();
}
pub unsafe fn test_10() {
    let mut ints: () =
        (unsafe { make_string_stream_7(&std::mem::transmute(*b"0 1  2   3     4\0")) });
    let mut oss: () = unimplemented!();
    let mut delimiter: () = (unsafe { make_string_4(&std::mem::transmute(*b"-\0")) });
    (unsafe {
        let mut ___r: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_9))(&mut ___r, unimplemented!())
    });
    let mut expected: () = (unsafe { make_string_5(&std::mem::transmute(*b"0-1-2-3-4-\0")) });
    unimplemented!();
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
    (unsafe { test_8() });
    (unsafe { test_10() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
