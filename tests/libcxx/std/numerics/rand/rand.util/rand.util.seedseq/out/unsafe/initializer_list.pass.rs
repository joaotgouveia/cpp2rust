extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
    let mut s: () = unimplemented!();
    assert!(((unimplemented!()) == (5_usize)));
    let mut b: [u32; 5] = [0_u32, 0_u32, 0_u32, 0_u32, 0_u32];
    unimplemented!();
    assert!(((b[(0) as usize]) == (5_u32)));
    assert!(((b[(1) as usize]) == (4_u32)));
    assert!(((b[(2) as usize]) == (3_u32)));
    assert!(((b[(3) as usize]) == (2_u32)));
    assert!(((b[(4) as usize]) == (1_u32)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
