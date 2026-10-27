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
        let mut e: () = unimplemented!();
        let mut d: () = unimplemented!();
        unimplemented!();
        let mut all_in_64bit_range: () = true;
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            let mut n: () = unimplemented!();
            all_in_64bit_range = (all_in_64bit_range) && ((n) <= (18446744073709551615_()));
            i.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut a: () = (((-9223372036854775807_()) - (1_())) * (10_()));
        let mut b: () = ((9223372036854775807_()) * (10_()));
        let mut e: () = unimplemented!();
        let mut d: () = unimplemented!();
        unimplemented!();
        let mut all_in_64bit_range: () = true;
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            let mut n: () = unimplemented!();
            unimplemented!();
            all_in_64bit_range = ((all_in_64bit_range)
                && (((-9223372036854775807_()) - (1_())) <= (n)))
                && ((n) <= (9223372036854775807_()));
            i.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut a: () = ((18446744073709551615_()) as ()).wrapping_div(3_());
        let mut b: () = ((18446744073709551615_()) as ()).wrapping_mul(10_());
        let mut e: () = unimplemented!();
        let mut d: () = unimplemented!();
        unimplemented!();
        let mut all_in_64bit_range: () = true;
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            let mut n: () = unimplemented!();
            unimplemented!();
            all_in_64bit_range = (all_in_64bit_range) && ((n) <= (18446744073709551615_()));
            i.prefix_inc();
        }
        unimplemented!();
    }
    {
        let mut e: () = unimplemented!();
        let mut d: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            let mut n: () = unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
