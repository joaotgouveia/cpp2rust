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
    let mut a1: [i32; 41] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    ];
    let mut a2: [i32; 38] = [
        -0_i32, -1_i32, -2_i32, -3_i32, -4_i32, -5_i32, -6_i32, -7_i32, -8_i32, -9_i32, -10_i32,
        -11_i32, -12_i32, -13_i32, -14_i32, -15_i32, -16_i32, -17_i32, -18_i32, -19_i32, -20_i32,
        -21_i32, -22_i32, -23_i32, -24_i32, -25_i32, -26_i32, -27_i32, -28_i32, -29_i32, -30_i32,
        -31_i32, -32_i32, -33_i32, -34_i32, -35_i32, -36_i32, -37_i32,
    ];
    let mut v1: () = unimplemented!();
    let v2: () = unimplemented!();
    let mut sz: [usize; 3] = [2_usize, 4_usize, 3_usize];
    let mut st: [usize; 3] = [19_usize, 4_usize, 1_usize];
    unimplemented!();
    assert!(((unimplemented!()) == (41_usize)));
    assert!(((*unimplemented!()) == (0)));
    assert!(((*unimplemented!()) == (1)));
    assert!(((*unimplemented!()) == (2)));
    assert!(((*unimplemented!()) == (-3_i32)));
    assert!(((*unimplemented!()) == (-4_i32)));
    assert!(((*unimplemented!()) == (-5_i32)));
    assert!(((*unimplemented!()) == (6)));
    assert!(((*unimplemented!()) == (-7_i32)));
    assert!(((*unimplemented!()) == (-8_i32)));
    assert!(((*unimplemented!()) == (-9_i32)));
    assert!(((*unimplemented!()) == (10)));
    assert!(((*unimplemented!()) == (-11_i32)));
    assert!(((*unimplemented!()) == (-12_i32)));
    assert!(((*unimplemented!()) == (-13_i32)));
    assert!(((*unimplemented!()) == (14)));
    assert!(((*unimplemented!()) == (-15_i32)));
    assert!(((*unimplemented!()) == (-16_i32)));
    assert!(((*unimplemented!()) == (-17_i32)));
    assert!(((*unimplemented!()) == (18)));
    assert!(((*unimplemented!()) == (19)));
    assert!(((*unimplemented!()) == (20)));
    assert!(((*unimplemented!()) == (21)));
    assert!(((*unimplemented!()) == (-22_i32)));
    assert!(((*unimplemented!()) == (-23_i32)));
    assert!(((*unimplemented!()) == (-24_i32)));
    assert!(((*unimplemented!()) == (25)));
    assert!(((*unimplemented!()) == (-26_i32)));
    assert!(((*unimplemented!()) == (-27_i32)));
    assert!(((*unimplemented!()) == (-28_i32)));
    assert!(((*unimplemented!()) == (29)));
    assert!(((*unimplemented!()) == (-30_i32)));
    assert!(((*unimplemented!()) == (-31_i32)));
    assert!(((*unimplemented!()) == (-32_i32)));
    assert!(((*unimplemented!()) == (33)));
    assert!(((*unimplemented!()) == (-34_i32)));
    assert!(((*unimplemented!()) == (-35_i32)));
    assert!(((*unimplemented!()) == (-36_i32)));
    assert!(((*unimplemented!()) == (37)));
    assert!(((*unimplemented!()) == (38)));
    assert!(((*unimplemented!()) == (39)));
    assert!(((*unimplemented!()) == (40)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
