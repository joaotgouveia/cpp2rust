extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_about_0(mut x: f64, mut y: f64, mut significant_digits: i32) -> bool {
    let mut epsilon: f64 = unimplemented!();
    return ((unimplemented!()) < (epsilon));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut a1: [f64; 5] = [-0.9_f64, -0.5_f64, 0_f64, 0.5_f64, 0.75_f64];
        let mut a3: [f64; 5] = [
            0.4065696597405991_f64,
            0.6065306597126334_f64,
            1_f64,
            1.6487212707001282_f64,
            2.117000016612675_f64,
        ];
        let N: u32 = (((::std::mem::size_of::<[f64; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<f64>() as usize))) as u32);
        let mut v1: () = unimplemented!();
        let mut v3: () = unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(
                (unsafe {
                    let _x: f64 = (*unimplemented!());
                    let _y: f64 = a3[(i)];
                    is_about_0(_x, _y, 10)
                })
            );
            i.prefix_inc();
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
