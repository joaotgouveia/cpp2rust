extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> bool {
    {
        let mut e: () = unimplemented!();
        let x: *mut i32 = unimplemented!();
        const _: () = assert!(true, "std::same_as<decltype(x), int&>");;
        assert!(((x) == (&mut (*unimplemented!()) as *mut i32)));
        assert!(((*x) == (5)));
    }
    {
        let e: () = unimplemented!();
        let x: *const i32 = unimplemented!();
        const _: () = assert!(true, "std::same_as<decltype(x), const int&>");;
        assert!(((x) == (&(*unimplemented!()) as *const i32)));
        assert!(((*x) == (5)));
    }
    {
        let mut e: () = unimplemented!();
        let x: *mut i32 = unimplemented!();
        const _: () = assert!(true, "std::same_as<decltype(x), int&&>");;
        assert!(((x) == (&mut (*unimplemented!()) as *mut i32)));
        assert!(((*x) == (5)));
    }
    {
        let e: () = unimplemented!();
        let x: *const i32 = unimplemented!();
        const _: () = assert!(true, "std::same_as<decltype(x), const int&&>");;
        assert!(((x) == (&(*unimplemented!()) as *const i32)));
        assert!(((*x) == (5)));
    }
    return true;
}
pub unsafe fn testException_1() {}
pub unsafe fn testAsConst_2() {}
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    (unsafe { testException_1() });
    (unsafe { testAsConst_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
