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
pub struct test_macros_detail_is_same_bool__bool_ {}
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
    let January: () = unimplemented!();
    let Monday: () = unimplemented!();
    let Tuesday: () = unimplemented!();
    let Wednesday: () = unimplemented!();
    let Thursday: () = unimplemented!();
    let Friday: () = unimplemented!();
    let Saturday: () = unimplemented!();
    let Sunday: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "!year_month_weekday{{}}.ok()");;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{-32768}}, month{{}}, weekday_indexed{{}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{-32768}}, January, weekday_indexed{{Tuesday, 1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   month{{}}, weekday_indexed{{Tuesday, 1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{}} }}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{-32768}}, month{{}}, weekday_indexed{{Tuesday, 1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   month{{}}, weekday_indexed{{}} }}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{-32768}}, January, weekday_indexed{{}} }}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{Tuesday, static_cast<unsigned>(-1)}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{Wednesday, 0}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_weekday{{year{{2019}},   January, weekday_indexed{{Tuesday, 1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_weekday{{year{{2019}},   January, weekday_indexed{{Tuesday, 4}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{Monday, 5}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_weekday{{year{{2019}},   January, weekday_indexed{{Tuesday, 5}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_weekday{{year{{2019}},   January, weekday_indexed{{Wednesday, 5}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_weekday{{year{{2019}},   January, weekday_indexed{{Thursday, 5}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{Friday, 5}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{Saturday, 5}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_weekday{{year{{2019}},   January, weekday_indexed{{Sunday, 5}}}}.ok()"
    );;
    let mut i: () = 0_();
    'loop_: while ((i) <= (50_())) {
        let mut ym: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) <= (50_())) {
        let mut ym: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) <= (50_())) {
        let mut ym: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    let ymax: () = unimplemented!();
    let mut i: () = ((ymax) - (100_()));
    'loop_: while ((i) <= ((ymax) + (100_()))) {
        let mut ym: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
