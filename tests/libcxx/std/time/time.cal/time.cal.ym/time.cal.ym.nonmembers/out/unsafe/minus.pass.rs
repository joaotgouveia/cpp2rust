extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_year_month__std_chrono_year_month_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_int__std_ratio_2629746__1____std_chrono_duration_int__std_ratio_2629746__1___
{}
pub unsafe fn test_2() -> () {
    {
        let mut ym: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) <= (10_())) {
            let mut ym1: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut ym: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) <= (10_())) {
            let mut ym1: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        let mut i: () = 12_();
        'loop_: while ((i) <= (15_())) {
            let mut ym1: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut y: () = unimplemented!();
        let mut i: () = 1_();
        'loop_: while ((i) <= (12_())) {
            let mut j: () = 1_();
            'loop_: while ((j) <= (12_())) {
                let mut diff: () = unimplemented!();
                unimplemented!();
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return true;
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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    (unsafe { test_2() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
