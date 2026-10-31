extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_year_month__std_chrono_year_month_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_2629746__1____std_chrono_duration_long__std_ratio_2629746__1___
{}
pub unsafe fn test_2() -> bool {
    {
        let mut ym: () = unimplemented!();
        let mut i: i32 = 0;
        'loop_: while ((i) <= (10)) {
            let mut ym1: () = unimplemented!();
            assert!(((unimplemented!()) == ((1234) - (i))));
            assert!(unimplemented!());
            i.prefix_inc();
        }
    }
    {
        let mut ym: () = unimplemented!();
        let mut i: i32 = 0;
        'loop_: while ((i) <= (10)) {
            let mut ym1: () = unimplemented!();
            assert!(((unimplemented!()) == (1234)));
            assert!(unimplemented!());
            i.prefix_inc();
        }
        let mut i: i32 = 12;
        'loop_: while ((i) <= (15)) {
            let mut ym1: () = unimplemented!();
            assert!(((unimplemented!()) == (1233)));
            assert!(unimplemented!());
            i.prefix_inc();
        }
    }
    {
        let mut y: () = unimplemented!();
        let mut i: i32 = 1;
        'loop_: while ((i) <= (12)) {
            let mut j: i32 = 1;
            'loop_: while ((j) <= (12)) {
                let mut diff: () = unimplemented!();
                assert!(((unimplemented!()) == (((i) - (j)) as ())));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    (unsafe { test_2() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
