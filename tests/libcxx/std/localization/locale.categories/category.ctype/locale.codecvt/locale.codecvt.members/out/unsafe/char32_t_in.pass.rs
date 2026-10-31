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
    let from: [libc::c_char; 10] = std::mem::transmute(*b"some text\0");
    let mut to: [(); 9] = [unimplemented!(); 9];
    let f: *const () = unimplemented!();
    let mut mbs: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
    let mut from_next: *const libc::c_char = std::ptr::null();
    let mut to_next: *mut () = std::ptr::null_mut();
    assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
    assert!(
        ((((from_next as usize - from.as_ptr() as usize) / ::std::mem::size_of::<libc::c_char>())
            as i64)
            == (9_i64))
    );
    assert!(
        ((((to_next as usize - to.as_mut_ptr() as usize) / ::std::mem::size_of::<()>()) as i64)
            == (9_i64))
    );
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (9_u32)) {
        assert!(((to[(i) as usize] as u32) == (from[(i) as usize] as u32)));
        i.prefix_inc();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
