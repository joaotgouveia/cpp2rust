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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{-32768}}, month{{}}, day{{}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{-32768}}, January, day{{1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2019}},   month{{}}, day{{1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2019}},   January, day{{}} }}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{-32768}}, month{{}}, day{{1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2019}},   month{{}}, day{{}} }}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{-32768}}, January, day{{}} }}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2019}},   January, day{{1}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 1}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2020}},   month{{ 2}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 3}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2020}},   month{{ 4}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 5}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2020}},   month{{ 6}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 7}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 8}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2020}},   month{{ 9}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{10}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2020}},   month{{11}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{12}}, day{{31}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 1}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2020}},   month{{ 2}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 3}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 4}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 5}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 6}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 7}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 8}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{ 9}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{10}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{11}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   month{{12}}, day{{30}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "!year_month_day{{year{{2019}},   std::chrono::February, day{{29}}}}.ok()"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{year{{2020}},   std::chrono::February, day{{29}}}}.ok()"
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
