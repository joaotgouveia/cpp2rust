extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_void__void_ {}
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
    let mut a2: [i32; 5] = [-1_i32, -2_i32, -3_i32, -4_i32, -5_i32];
    let mut v1: () = unimplemented!();
    let mut v2: () = unimplemented!();
    let mut s1: () = unimplemented!();
    unimplemented!();
    assert!(((unimplemented!()) == (16_usize)));
    assert!(((*unimplemented!()) == (0)));
    assert!(((*unimplemented!()) == (-1_i32)));
    assert!(((*unimplemented!()) == (2)));
    assert!(((*unimplemented!()) == (3)));
    assert!(((*unimplemented!()) == (-2_i32)));
    assert!(((*unimplemented!()) == (5)));
    assert!(((*unimplemented!()) == (6)));
    assert!(((*unimplemented!()) == (-3_i32)));
    assert!(((*unimplemented!()) == (8)));
    assert!(((*unimplemented!()) == (9)));
    assert!(((*unimplemented!()) == (-4_i32)));
    assert!(((*unimplemented!()) == (11)));
    assert!(((*unimplemented!()) == (12)));
    assert!(((*unimplemented!()) == (-5_i32)));
    assert!(((*unimplemented!()) == (14)));
    assert!(((*unimplemented!()) == (15)));
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
    let mut s2: () = unimplemented!();
    unimplemented!();
    assert!(((*unimplemented!()) == (1_f64)));
    assert!(((*unimplemented!()) == (2_f64)));
    assert!(((*unimplemented!()) == (3_f64)));
    const _: () = assert!(true, "");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
