extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn make_string_0(in_: *const [libc::c_char; 9]) -> Vec<libc::c_char> {
    return unimplemented!();
}
pub unsafe fn make_string_1(in_: *const [libc::c_char; 9]) -> Vec<i32> {
    return unimplemented!();
}
pub unsafe fn make_string_stream_2(in_: *const [libc::c_char; 9]) -> () {
    return unimplemented!();
}
pub unsafe fn make_string_stream_3(in_: *const [libc::c_char; 9]) -> () {
    return unimplemented!();
}
pub unsafe fn test_4() {
    {
        let mut iss: () = (unsafe { make_string_stream_2(&std::mem::transmute(*b"1 2 345 \0")) });
        let mut isv: () = unimplemented!();
        let mut it: () = unimplemented!();
        assert!(((*unimplemented!()) == (1)));
        let it2: *mut () = &mut (*unimplemented!());
        assert!(((it2) == (&mut it as *mut ())));
        assert!(((*unimplemented!()) == (2)));
        unimplemented!();
        assert!(((*unimplemented!()) == (345)));
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut iss: () = (unsafe { make_string_stream_2(&std::mem::transmute(*b"1 2 345 \0")) });
        let mut isv: () = unimplemented!();
        let mut it: () = unimplemented!();
        assert!(((*unimplemented!()) == (1)));
        const _: () = assert!(true, "std::same_as<decltype(it++), void>");;
        unimplemented!();
        assert!(((*unimplemented!()) == (2)));
    }
}
pub unsafe fn test_5() {
    {
        let mut iss: () = (unsafe { make_string_stream_3(&std::mem::transmute(*b"1 2 345 \0")) });
        let mut isv: () = unimplemented!();
        let mut it: () = unimplemented!();
        assert!(((*unimplemented!()) == (1)));
        let it2: *mut () = &mut (*unimplemented!());
        assert!(((it2) == (&mut it as *mut ())));
        assert!(((*unimplemented!()) == (2)));
        unimplemented!();
        assert!(((*unimplemented!()) == (345)));
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut iss: () = (unsafe { make_string_stream_3(&std::mem::transmute(*b"1 2 345 \0")) });
        let mut isv: () = unimplemented!();
        let mut it: () = unimplemented!();
        assert!(((*unimplemented!()) == (1)));
        const _: () = assert!(true, "std::same_as<decltype(it++), void>");;
        unimplemented!();
        assert!(((*unimplemented!()) == (2)));
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
    (unsafe { test_4() });
    (unsafe { test_5() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
