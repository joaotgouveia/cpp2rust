extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn tests_0() -> () {
    {
        let mut arr: () = unimplemented!();
        const _: () = assert!(true, "std::is_same_v<decltype(arr), std::array<int, 3>>");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let l1: () = 42_();
        let mut arr: () = unimplemented!();
        const _: () = assert!(true, "std::is_same_v<decltype(arr)::value_type, long>");;
        const _: () = assert!(true, "arr.size() == 4");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut source: () = unimplemented!();
        let mut arr: () = source.clone();
        const _: () = assert!(true, "std::is_same_v<decltype(arr), decltype(source)>");;
        const _: () = assert!(true, "std::is_same_v<decltype(arr), std::array<double, 2>>");;
        unimplemented!();
        unimplemented!();
    }
    return true;
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
    (unsafe { tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
