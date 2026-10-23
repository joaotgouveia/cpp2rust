extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn RunTheExample_0() {
    const _: () = assert!(
        true,
        "year_month_day{{sys_days{{year{{2017}}/January/0}}}}  == year{{2016}}/December/31"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{sys_days{{year{{2017}}/January/31}}}} == year{{2017}}/January/31"
    );;
    const _: () = assert!(
        true,
        "year_month_day{{sys_days{{year{{2017}}/January/32}}}} == year{{2017}}/February/1"
    );;
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
    (unsafe { RunTheExample_0() });
    {
        let ymd: () = unimplemented!();
        let sd: () = unimplemented!();
        const _: () = assert!(true, "sd.time_since_epoch() == days{{0}}");;
        const _: () = assert!(true, "year_month_day{{sd}} == ymd");;
    }
    {
        let ymd: () = unimplemented!();
        let sd: () = unimplemented!();
        const _: () = assert!(true, "sd.time_since_epoch() == days{{10957+32}}");;
        const _: () = assert!(true, "year_month_day{{sd}} == ymd");;
    }
    {
        let ymd: () = unimplemented!();
        let sd: () = unimplemented!();
        const _: () = assert!(true, "sd.time_since_epoch() == days{{-10957}}");;
        const _: () = assert!(true, "year_month_day{{sd}} == ymd");;
    }
    {
        let mut ymd: () = unimplemented!();
        let mut sd: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let ymd: () = unimplemented!();
        const _: () = assert!(true, "!ymd.ok()");;
        const _: () = assert!(true, "ymd.year().ok()");;
        const _: () = assert!(true, "ymd.month().ok()");;
        const _: () = assert!(true, "!ymd.day().ok()");;
        let sd: () = unimplemented!();
        const _: () = assert!(true, "sd.time_since_epoch() == days{{364}}");;
        const _: () = assert!(
            true,
            "sd == sys_days{{ymd.year()/ymd.month()/day{{1}}}} + (ymd.day() - day{{1}})"
        );;
    }
    {
        let ymd: () = unimplemented!();
        const _: () = assert!(true, "!ymd.ok()");;
        const _: () = assert!(true, "ymd.year().ok()");;
        const _: () = assert!(true, "ymd.month().ok()");;
        const _: () = assert!(true, "!ymd.day().ok()");;
        let sd: () = unimplemented!();
        const _: () = assert!(true, "sd.time_since_epoch() == days{{365}}");;
        const _: () = assert!(
            true,
            "sd == sys_days{{ymd.year()/ymd.month()/day{{1}}}} + (ymd.day() - day{{1}})"
        );;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
