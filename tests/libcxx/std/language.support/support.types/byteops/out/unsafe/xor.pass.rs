extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
const _: () = assert!(true, "noexcept(std::byte{{}} ^ std::byte{{}})");
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::byte{{}} ^ std::byte{{}}), std::byte>"
);
pub unsafe fn test_0() -> bool {
    let mut b1: u8 = 1_u8;
    let mut b8: u8 = 8_u8;
    let mut b9: u8 = 9_u8;
    assert!(((<i32>::cpp_to_integer(b1 ^ b8)) == (9)));
    assert!(((<i32>::cpp_to_integer(b1 ^ b9)) == (8)));
    assert!(((<i32>::cpp_to_integer(b8 ^ b9)) == (1)));
    assert!(((<i32>::cpp_to_integer(b8 ^ b1)) == (9)));
    assert!(((<i32>::cpp_to_integer(b9 ^ b1)) == (8)));
    assert!(((<i32>::cpp_to_integer(b9 ^ b8)) == (1)));
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
