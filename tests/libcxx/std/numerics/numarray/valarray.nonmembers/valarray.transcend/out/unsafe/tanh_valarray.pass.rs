extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_about_0(mut x: (), mut y: (), mut significant_digits: ()) -> () {
    let mut epsilon: () = (unsafe { pow_1(((1_f64) / (10_f64)), significant_digits) });
    return ((unsafe { abs_2(((x) - (y))) }) < (epsilon));
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
        let mut a1: [(); 5] = [-0.9_f64, -0.5_f64, 0_f64, 0.5_f64, 0.75_f64];
        let mut a3: [(); 5] = [
            -0.7162978701990245_f64,
            -0.46211715726000974_f64,
            0_f64,
            0.46211715726000974_f64,
            0.6351489523872873_f64,
        ];
        let N: () = (::std::mem::size_of::<[(); 5]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut v1: () = unimplemented!();
        let mut v3: () = (unsafe { unimplemented!()() });
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
