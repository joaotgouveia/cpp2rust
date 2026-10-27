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
pub struct test_macros_detail_is_same_std_chrono_day__std_chrono_day_ {}
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
    const _: () = assert!(
        true,
        "year_month_day_last{{year{{2020}}, month_day_last{{month{{ 1}}}}}}.day() == day{{31}}"
    );;
    const _: () = assert!(
        true,
        "year_month_day_last{{year{{2020}}, month_day_last{{month{{ 2}}}}}}.day() == day{{29}}"
    );;
    const _: () = assert!(
        true,
        "year_month_day_last{{year{{2020}}, month_day_last{{month{{ 3}}}}}}.day() == day{{31}}"
    );;
    const _: () = assert!(
        true,
        "year_month_day_last{{year{{2020}}, month_day_last{{month{{ 4}}}}}}.day() == day{{30}}"
    );;
    const _: () = assert!(
        true,
        "year_month_day_last{{year{{2020}}, month_day_last{{month{{ 5}}}}}}.day() == day{{31}}"
    );;
    const _: () = assert!(
        true,
        "year_month_day_last{{year{{2020}}, month_day_last{{month{{ 6}}}}}}.day() == day{{30}}"
    );;
    const _: () = assert!(
        true,
        "year_month_day_last{{year{{2020}}, month_day_last{{month{{ 7}}}}}}.day() == day{{31}}"
    );;
    const _: () = assert!(
        true,
        "year_month_day_last{{year{{2020}}, month_day_last{{month{{ 8}}}}}}.day() == day{{31}}"
    );;
    const _: () = assert!(
        true,
        "year_month_day_last{{year{{2020}}, month_day_last{{month{{ 9}}}}}}.day() == day{{30}}"
    );;
    const _: () = assert!(
        true,
        "year_month_day_last{{year{{2020}}, month_day_last{{month{{10}}}}}}.day() == day{{31}}"
    );;
    const _: () = assert!(
        true,
        "year_month_day_last{{year{{2020}}, month_day_last{{month{{11}}}}}}.day() == day{{30}}"
    );;
    const _: () = assert!(
        true,
        "year_month_day_last{{year{{2020}}, month_day_last{{month{{12}}}}}}.day() == day{{31}}"
    );;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
