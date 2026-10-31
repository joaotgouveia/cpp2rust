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
    let mut ia: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut ia1: [i32; 10] = [2, 7, 1, 4, 3, 6, 5, 10, 9, 8];
    let mut ia2: [i32; 10] = [1, 8, 3, 4, 6, 9, 5, 7, 2, 10];
    let sa: u32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut g: () = unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "");;
    assert!(unimplemented!());
    unimplemented!();
    const _: () = assert!(true, "");;
    assert!(unimplemented!());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
