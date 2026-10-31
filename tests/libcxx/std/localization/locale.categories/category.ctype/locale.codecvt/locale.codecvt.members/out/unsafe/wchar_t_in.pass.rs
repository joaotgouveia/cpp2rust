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
    let mut l: () = unimplemented!();
    let from: Vec<libc::c_char> = std::slice::from_raw_parts(
        c"some text".as_ptr(),
        libcc2rs::c_str_len(c"some text".as_ptr()) + 1,
    )
    .to_vec();
    let expected: Vec<i32> = unimplemented!();
    let mut to: Vec<i32> = vec![(unimplemented!() as i32); (from.len() - 1) as usize]
        .iter()
        .cloned()
        .chain(std::iter::once(0_i32))
        .collect();
    let f: *const () = unimplemented!();
    let mut mbs: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
    let mut from_next: *const () = std::ptr::null();
    let mut to_next: *mut () = std::ptr::null_mut();
    let mut r: () = unimplemented!();
    assert!(((r as i32) == (unimplemented!() as i32)));
    assert!(
        (((((from_next as usize - from.as_ptr() as usize) / ::std::mem::size_of::<()>()) as i64)
            as usize)
            == (from.len() - 1))
    );
    assert!(
        (((((to_next as usize - to.as_mut_ptr() as usize) / ::std::mem::size_of::<()>()) as i64)
            as usize)
            == (expected.len() - 1))
    );
    assert!(
        (((((to_next as usize - to.as_mut_ptr() as usize) / ::std::mem::size_of::<()>()) as i64)
            as usize)
            == (expected.len() - 1))
    );
    assert!(unimplemented!());
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
