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
        let mut i: i32 = 0;
        const _: () = assert!(
            true,
            "std::is_same<decltype(std::pointer_traits<int*>::pointer_to(i)), int*>::value"
        );;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (&mut i as *mut i32)));
    }
    {
        let i: i32 = 0;
        const _: () = assert!(
            true,
            "std::is_same<decltype(std::pointer_traits<const int*>::pointer_to(i)), const int*>::value"
        );;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (&i as *const i32)));
    }
    {
        let mut i: i32 = 0;
        const _: () = assert!(
            true,
            "std::is_same<decltype(std::pointer_traits<const int*>::pointer_to(i)), const int*>::value"
        );;
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == ((&mut i as *mut i32).cast_const())));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    {
        const _: () = assert!(
            true,
            "std::is_same<std::pointer_traits<void*>::element_type, void>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_same<std::pointer_traits<const void*>::element_type, const void>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_same<std::pointer_traits<volatile void*>::element_type, volatile void>::value"
        );;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
