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
    let mut a1: [i32; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let N1: usize = (::std::mem::size_of::<[i32; 16]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize));
    let mut a2: [i32; 5] = [1, 2, 3, 4, 5];
    let N2: usize = (::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize));
    let mut b: [bool; 16] = [
        true, false, false, true, true, false, false, true, false, false, false, true, false,
        false, false, false,
    ];
    let mut v1: () = unimplemented!();
    let mut v2: () = unimplemented!();
    assert!(((N2) == (unimplemented!() as usize)));
    let mut vb: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (16_usize)));
    assert!(((*unimplemented!()) == (-1_i32)));
    assert!(((*unimplemented!()) == (1)));
    assert!(((*unimplemented!()) == (2)));
    assert!(((*unimplemented!()) == (1)));
    assert!(((*unimplemented!()) == (1)));
    assert!(((*unimplemented!()) == (5)));
    assert!(((*unimplemented!()) == (6)));
    assert!(((*unimplemented!()) == (3)));
    assert!(((*unimplemented!()) == (8)));
    assert!(((*unimplemented!()) == (9)));
    assert!(((*unimplemented!()) == (10)));
    assert!(((*unimplemented!()) == (6)));
    assert!(((*unimplemented!()) == (12)));
    assert!(((*unimplemented!()) == (13)));
    assert!(((*unimplemented!()) == (14)));
    assert!(((*unimplemented!()) == (15)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
