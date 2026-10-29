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
        let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let N: () = (::std::mem::size_of::<[(); 5]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut a: [(); 5] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let N: () = (::std::mem::size_of::<[(); 5]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (N)) {
            unimplemented!();
            let mut j: () = 0_();
            'loop_: while ((j) < (unimplemented!())) {
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let N: () = (::std::mem::size_of::<[(); 5]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut v: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut array: [(); 3] = [1_(), 2_(), 3_()];
        let mut a: () = unimplemented!();
        let mut b: () = unimplemented!();
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
