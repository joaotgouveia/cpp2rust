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
        let mut b: [(); 4] = [10_(), 14_(), 16_(), 17_()];
        let mut p: [(); 3] = [25_(), 62.5_f64, 12.5_f64];
        let mut p1: () = std_piecewise_constant_distribution___param_type::new_1(
            { b.as_mut_ptr() },
            { b.as_mut_ptr().offset((4_()) as isize) },
            { p.as_mut_ptr() },
        );
        let mut p2: () = std_piecewise_constant_distribution___param_type::new_1(
            { b.as_mut_ptr() },
            { b.as_mut_ptr().offset((4_()) as isize) },
            { p.as_mut_ptr() },
        );
        unimplemented!();
    }
    {
        let mut b: [(); 4] = [10_(), 14_(), 16_(), 17_()];
        let mut p: [(); 3] = [25_(), 62.5_f64, 12.5_f64];
        let mut p1: () = std_piecewise_constant_distribution___param_type::new_1(
            { b.as_mut_ptr() },
            { b.as_mut_ptr().offset((3_()) as isize) },
            { p.as_mut_ptr() },
        );
        let mut p2: () = std_piecewise_constant_distribution___param_type::new_1(
            { b.as_mut_ptr() },
            { b.as_mut_ptr().offset((4_()) as isize) },
            { p.as_mut_ptr() },
        );
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
