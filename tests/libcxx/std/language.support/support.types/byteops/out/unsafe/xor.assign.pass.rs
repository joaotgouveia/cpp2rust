extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_op_0(mut b1: u8, mut b2: u8) -> u8 {
    const _: () = assert!(true, "noexcept(b1 ^= b2)");;
    const _: () = assert!(true, "std::is_same_v<decltype(b1 ^= b2), std::byte&>");;
    let ret: *mut u8 = &mut (*{
        *&mut b1 ^= b2;
        &mut b1
    });
    assert!(((ret) == (&mut b1 as *mut u8)));
    return (*ret);
}
pub unsafe fn test_1() -> bool {
    let mut b1: u8 = 1_u8;
    let mut b8: u8 = 8_u8;
    let mut b9: u8 = 9_u8;
    assert!(((<i32>::cpp_to_integer((unsafe { test_op_0(b1, b8,) }))) == (9)));
    assert!(((<i32>::cpp_to_integer((unsafe { test_op_0(b1, b9,) }))) == (8)));
    assert!(((<i32>::cpp_to_integer((unsafe { test_op_0(b8, b9,) }))) == (1)));
    assert!(((<i32>::cpp_to_integer((unsafe { test_op_0(b8, b1,) }))) == (9)));
    assert!(((<i32>::cpp_to_integer((unsafe { test_op_0(b9, b1,) }))) == (8)));
    assert!(((<i32>::cpp_to_integer((unsafe { test_op_0(b9, b8,) }))) == (1)));
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
