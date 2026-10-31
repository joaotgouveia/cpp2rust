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
        let mut a1: [f64; 5] = [1.5_f64, 2.5_f64, (-3_i32 as f64), 4_f64, 5.5_f64];
        let N1: u32 = (((::std::mem::size_of::<[f64; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<f64>() as usize))) as u32);
        let mut v1: () = unimplemented!();
        assert!(((unimplemented!()) == (-3_f64)));
    }
    {
        let mut a1: [f64; 5] = [1.5_f64, 2.5_f64, (-3_i32 as f64), 4_f64, 5.5_f64];
        let N1: u32 = (((::std::mem::size_of::<[f64; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<f64>() as usize))) as u32);
        let mut v1: () = unimplemented!();
        assert!(((unimplemented!()) == (-6_f64)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
