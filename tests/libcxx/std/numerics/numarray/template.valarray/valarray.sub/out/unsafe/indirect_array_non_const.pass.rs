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
    {
        let mut a: [i32; 41] = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
        ];
        let N1: usize = (::std::mem::size_of::<[i32; 41]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize));
        let mut s: [usize; 24] = [
            3_usize, 4_usize, 5_usize, 7_usize, 8_usize, 9_usize, 11_usize, 12_usize, 13_usize,
            15_usize, 16_usize, 17_usize, 22_usize, 23_usize, 24_usize, 26_usize, 27_usize,
            28_usize, 30_usize, 31_usize, 32_usize, 34_usize, 35_usize, 36_usize,
        ];
        let S: usize = (::std::mem::size_of::<[usize; 24]>() as usize)
            .wrapping_div((::std::mem::size_of::<usize>() as usize));
        let mut v1: () = unimplemented!();
        let mut ia: () = unimplemented!();
        let mut v: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (24_usize)));
        assert!(((*unimplemented!()) == (3)));
        assert!(((*unimplemented!()) == (4)));
        assert!(((*unimplemented!()) == (5)));
        assert!(((*unimplemented!()) == (7)));
        assert!(((*unimplemented!()) == (8)));
        assert!(((*unimplemented!()) == (9)));
        assert!(((*unimplemented!()) == (11)));
        assert!(((*unimplemented!()) == (12)));
        assert!(((*unimplemented!()) == (13)));
        assert!(((*unimplemented!()) == (15)));
        assert!(((*unimplemented!()) == (16)));
        assert!(((*unimplemented!()) == (17)));
        assert!(((*unimplemented!()) == (22)));
        assert!(((*unimplemented!()) == (23)));
        assert!(((*unimplemented!()) == (24)));
        assert!(((*unimplemented!()) == (26)));
        assert!(((*unimplemented!()) == (27)));
        assert!(((*unimplemented!()) == (28)));
        assert!(((*unimplemented!()) == (30)));
        assert!(((*unimplemented!()) == (31)));
        assert!(((*unimplemented!()) == (32)));
        assert!(((*unimplemented!()) == (34)));
        assert!(((*unimplemented!()) == (35)));
        assert!(((*unimplemented!()) == (36)));
    }
    {
        let mut raw_data: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut idx_data: [usize; 5] = [0_usize, 2_usize, 4_usize, 6_usize, 8_usize];
        let mut data: () = unimplemented!();
        let mut idx: () = unimplemented!();
        let mut result: () = unimplemented!();
        assert!(((unimplemented!()) == (5_usize)));
        assert!(((*unimplemented!()) == (*unimplemented!())));
        assert!(((*unimplemented!()) == (*unimplemented!())));
        assert!(((*unimplemented!()) == (*unimplemented!())));
        assert!(((*unimplemented!()) == (*unimplemented!())));
        assert!(((*unimplemented!()) == (*unimplemented!())));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
