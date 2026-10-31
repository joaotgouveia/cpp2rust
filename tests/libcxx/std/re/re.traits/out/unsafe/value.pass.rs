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
        let mut t: () = unimplemented!();
        let mut c: libc::c_char = (0 as libc::c_char);
        'loop_: while ((c as i32) < (('0' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            c.prefix_inc();
        }
        let mut c: libc::c_char = ('0' as libc::c_char);
        'loop_: while ((c as i32) < (('8' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == ((c as i32) - (('0' as libc::c_char) as i32))));
            assert!(((unimplemented!()) == ((c as i32) - (('0' as libc::c_char) as i32))));
            assert!(((unimplemented!()) == ((c as i32) - (('0' as libc::c_char) as i32))));
            c.prefix_inc();
        }
        let mut c: libc::c_char = ('8' as libc::c_char);
        'loop_: while ((c as i32) < ((':' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == ((c as i32) - (('0' as libc::c_char) as i32))));
            assert!(((unimplemented!()) == ((c as i32) - (('0' as libc::c_char) as i32))));
            c.prefix_inc();
        }
        let mut c: libc::c_char = (':' as libc::c_char);
        'loop_: while ((c as i32) < (('A' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            c.prefix_inc();
        }
        let mut c: libc::c_char = ('A' as libc::c_char);
        'loop_: while ((c as i32) < (('G' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (((c as i32) - (('A' as libc::c_char) as i32)) + (10))));
            c.prefix_inc();
        }
        let mut c: libc::c_char = ('G' as libc::c_char);
        'loop_: while ((c as i32) < (('a' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            c.prefix_inc();
        }
        let mut c: libc::c_char = ('a' as libc::c_char);
        'loop_: while ((c as i32) < (('g' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (((c as i32) - (('a' as libc::c_char) as i32)) + (10))));
            c.prefix_inc();
        }
        let mut c: i32 = (('g' as libc::c_char) as i32);
        'loop_: while ((c) < (256)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            c.prefix_inc();
        }
    }
    {
        let mut t: () = unimplemented!();
        let mut c: i32 = 0_i32;
        'loop_: while ((c) < (('0' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            c.prefix_inc();
        }
        let mut c: i32 = (('0' as libc::c_char) as i32);
        'loop_: while ((c) < (('8' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == ((c) - (('0' as libc::c_char) as i32))));
            assert!(((unimplemented!()) == ((c) - (('0' as libc::c_char) as i32))));
            assert!(((unimplemented!()) == ((c) - (('0' as libc::c_char) as i32))));
            c.prefix_inc();
        }
        let mut c: i32 = (('8' as libc::c_char) as i32);
        'loop_: while ((c) < ((':' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == ((c) - (('0' as libc::c_char) as i32))));
            assert!(((unimplemented!()) == ((c) - (('0' as libc::c_char) as i32))));
            c.prefix_inc();
        }
        let mut c: i32 = ((':' as libc::c_char) as i32);
        'loop_: while ((c) < (('A' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            c.prefix_inc();
        }
        let mut c: i32 = (('A' as libc::c_char) as i32);
        'loop_: while ((c) < (('G' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (((c) - (('A' as libc::c_char) as i32)) + (10))));
            c.prefix_inc();
        }
        let mut c: i32 = (('G' as libc::c_char) as i32);
        'loop_: while ((c) < (('a' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            c.prefix_inc();
        }
        let mut c: i32 = (('a' as libc::c_char) as i32);
        'loop_: while ((c) < (('g' as libc::c_char) as i32)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (((c) - (('a' as libc::c_char) as i32)) + (10))));
            c.prefix_inc();
        }
        let mut c: i32 = (('g' as libc::c_char) as i32);
        'loop_: while ((c) < (65535)) {
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            assert!(((unimplemented!()) == (-1_i32)));
            c.prefix_inc();
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
