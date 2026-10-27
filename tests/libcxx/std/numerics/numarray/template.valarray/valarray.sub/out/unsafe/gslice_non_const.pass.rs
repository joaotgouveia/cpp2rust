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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut a: [(); 41] = [
            0_(),
            1_(),
            2_(),
            3_(),
            4_(),
            5_(),
            6_(),
            7_(),
            8_(),
            9_(),
            10_(),
            11_(),
            12_(),
            13_(),
            14_(),
            15_(),
            16_(),
            17_(),
            18_(),
            19_(),
            20_(),
            21_(),
            22_(),
            23_(),
            24_(),
            25_(),
            26_(),
            27_(),
            28_(),
            29_(),
            30_(),
            31_(),
            32_(),
            33_(),
            34_(),
            35_(),
            36_(),
            37_(),
            38_(),
            39_(),
            40_(),
        ];
        let mut v1: () = unimplemented!();
        let mut sz: [(); 3] = [2_(), 4_(), 3_()];
        let mut st: [(); 3] = [19_(), 4_(), 1_()];
        let mut v: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 41] = [
            0_(),
            1_(),
            2_(),
            3_(),
            4_(),
            5_(),
            6_(),
            7_(),
            8_(),
            9_(),
            10_(),
            11_(),
            12_(),
            13_(),
            14_(),
            15_(),
            16_(),
            17_(),
            18_(),
            19_(),
            20_(),
            21_(),
            22_(),
            23_(),
            24_(),
            25_(),
            26_(),
            27_(),
            28_(),
            29_(),
            30_(),
            31_(),
            32_(),
            33_(),
            34_(),
            35_(),
            36_(),
            37_(),
            38_(),
            39_(),
            40_(),
        ];
        let mut v1: () = unimplemented!();
        let mut sz: [(); 3] = [2_(), 4_(), 3_()];
        let mut st: [(); 3] = [19_(), 4_(), 1_()];
        let mut v: () = (unsafe { unimplemented!()() });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
