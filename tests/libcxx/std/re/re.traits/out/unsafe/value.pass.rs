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
        let mut t: () = unimplemented!();
        let mut c: () = 0_();
        'loop_: while ((c) < ('0' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = ('0' as ());
        'loop_: while ((c) < ('8' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = ('8' as ());
        'loop_: while ((c) < (':' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = (':' as ());
        'loop_: while ((c) < ('A' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = ('A' as ());
        'loop_: while ((c) < ('G' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = ('G' as ());
        'loop_: while ((c) < ('a' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = ('a' as ());
        'loop_: while ((c) < ('g' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = ('g' as ());
        'loop_: while ((c) < (256_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
    }
    {
        let mut t: () = unimplemented!();
        let mut c: () = 0_();
        'loop_: while ((c) < ('0' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = ('0' as ());
        'loop_: while ((c) < ('8' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = ('8' as ());
        'loop_: while ((c) < (':' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = (':' as ());
        'loop_: while ((c) < ('A' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = ('A' as ());
        'loop_: while ((c) < ('G' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = ('G' as ());
        'loop_: while ((c) < ('a' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = ('a' as ());
        'loop_: while ((c) < ('g' as ())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
        let mut c: () = ('g' as ());
        'loop_: while ((c) < (65535_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            c.prefix_inc();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
