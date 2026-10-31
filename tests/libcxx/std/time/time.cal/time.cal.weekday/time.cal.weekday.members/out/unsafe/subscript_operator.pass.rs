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
pub struct test_macros_detail_is_same_std_chrono_weekday_indexed__std_chrono_weekday_indexed_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_weekday_last__std_chrono_weekday_last_ {}
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
    let Sunday: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "Sunday[2].weekday() == Sunday");;
    const _: () = assert!(true, "Sunday[2].index  () == 2");;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) <= (6_u32)) {
        let mut wd: () = unimplemented!();
        let mut wdl: () = unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
        i.prefix_inc();
    }
    let mut i: u32 = 0_u32;
    'loop_: while ((i) <= (6_u32)) {
        let mut j: u32 = 1_u32;
        'loop_: while ((j) <= (5_u32)) {
            let mut wd: () = unimplemented!();
            let mut wdi: () = unimplemented!();
            assert!(unimplemented!());
            assert!(((unimplemented!()) == (j)));
            assert!(unimplemented!());
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
