extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f_0(mut t: ()) -> () {
    return ((t) + (5_()));
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut a1: [(); 10] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_(), 10_()];
        let mut a2: [(); 10] = [
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
        ];
        let N1: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut v1: () = unimplemented!();
        let mut v2: () = (unsafe { v1.apply_69(Some(f_0)) });
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (N1)) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let N1: () = 0_();
        let mut v1: () = unimplemented!();
        let mut v2: () = (unsafe { v1.apply_69(Some(f_0)) });
        unimplemented!();
    }
    {
        let mut a1: [(); 10] = [1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_(), 10_()];
        let mut a2: [(); 10] = [
            7_(),
            9_(),
            11_(),
            13_(),
            15_(),
            17_(),
            19_(),
            21_(),
            23_(),
            25_(),
        ];
        let N1: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut v1: () = unimplemented!();
        let mut v2: () = (unsafe { (unsafe { (unimplemented!()).apply_18(Some(f_0)) })() });
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (N1)) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
