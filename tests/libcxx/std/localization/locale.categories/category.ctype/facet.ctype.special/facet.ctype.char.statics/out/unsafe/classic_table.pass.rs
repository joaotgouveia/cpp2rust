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
    unimplemented!();
    unimplemented!();
    let mut p: *const () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (128_())) {
        let mut expect_cntrl: () = (((i) < (32_())) || ((126_()) < (i)));
        let mut expect_print: () = (((32_()) <= (i)) && ((i) <= (126_())));
        let mut expect_space: () = (((9_()) <= (i)) && ((i) <= (13_()))) || ((i) == (' ' as ()));
        let mut expect_blank: () = (((i) == ('\t' as ())) || ((i) == (' ' as ())));
        let mut expect_upper: () = ((('A' as ()) <= (i)) && ((i) <= ('Z' as ())));
        let mut expect_lower: () = ((('a' as ()) <= (i)) && ((i) <= ('z' as ())));
        let mut expect_alpha: () = (expect_upper) || (expect_lower);
        let mut expect_digit: () = ((('0' as ()) <= (i)) && ((i) <= ('9' as ())));
        let mut expect_xdigit: () = (((('A' as ()) <= (i)) && ((i) <= ('F' as ())))
            || ((('a' as ()) <= (i)) && ((i) <= ('f' as ()))))
            || (expect_digit);
        let mut expect_punct: () = (((((33_()) <= (i)) && ((i) <= (47_())))
            || (((58_()) <= (i)) && ((i) <= (64_()))))
            || (((91_()) <= (i)) && ((i) <= (96_()))))
            || (((123_()) <= (i)) && ((i) <= (126_())));
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
        i.prefix_inc();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
