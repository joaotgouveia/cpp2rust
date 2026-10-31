extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> bool {
    let mut s1: [u16; 3] = [1_u16, 2_u16, 3_u16];
    let mut s2: [u16; 3] = [0_u16, 0_u16, 0_u16];
    assert!(((unimplemented!()) == (s2.as_mut_ptr())));
    assert!(((s2[(0) as usize] as i32) == (1_u16 as i32)));
    assert!(((s2[(1) as usize] as i32) == (2_u16 as i32)));
    assert!(((s2[(2) as usize] as i32) == (3_u16 as i32)));
    assert!((unimplemented!()).is_null());
    assert!(((unimplemented!()) == (s1.as_mut_ptr())));
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
