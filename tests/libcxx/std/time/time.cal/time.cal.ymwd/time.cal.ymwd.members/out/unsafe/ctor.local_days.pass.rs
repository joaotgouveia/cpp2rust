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
    const _: () = assert!(true, "");;
    {
        let sd: () = unimplemented!();
        let ymwd: () = unimplemented!();
        const _: () = assert!(true, "ymwd.ok()");;
        const _: () = assert!(true, "ymwd.year()            == year{{1970}}");;
        const _: () = assert!(true, "ymwd.month()           == std::chrono::January");;
        const _: () = assert!(true, "ymwd.weekday()         == std::chrono::Thursday");;
        const _: () = assert!(true, "ymwd.index()           == 1");;
        const _: () = assert!(
            true,
            "ymwd.weekday_indexed() == weekday_indexed{{std::chrono::Thursday, 1}}"
        );;
        const _: () = assert!(
            true,
            "ymwd                   == year_month_weekday{{local_days{{ymwd}}}}"
        );;
    }
    {
        let sd: () = unimplemented!();
        let ymwd: () = unimplemented!();
        const _: () = assert!(true, "ymwd.ok()");;
        const _: () = assert!(true, "ymwd.year()            == year{{2000}}");;
        const _: () = assert!(true, "ymwd.month()           == std::chrono::February");;
        const _: () = assert!(true, "ymwd.weekday()         == std::chrono::Wednesday");;
        const _: () = assert!(true, "ymwd.index()           == 1");;
        const _: () = assert!(
            true,
            "ymwd.weekday_indexed() == weekday_indexed{{std::chrono::Wednesday, 1}}"
        );;
        const _: () = assert!(
            true,
            "ymwd                   == year_month_weekday{{local_days{{ymwd}}}}"
        );;
    }
    {
        let sd: () = unimplemented!();
        let ymwd: () = unimplemented!();
        const _: () = assert!(true, "ymwd.ok()");;
        const _: () = assert!(true, "ymwd.year()            == year{{1940}}");;
        const _: () = assert!(true, "ymwd.month()           == std::chrono::January");;
        const _: () = assert!(true, "ymwd.weekday()         == std::chrono::Tuesday");;
        const _: () = assert!(true, "ymwd.index()           == 1");;
        const _: () = assert!(
            true,
            "ymwd.weekday_indexed() == weekday_indexed{{std::chrono::Tuesday, 1}}"
        );;
        const _: () = assert!(
            true,
            "ymwd                   == year_month_weekday{{local_days{{ymwd}}}}"
        );;
    }
    {
        let mut sd: () = unimplemented!();
        let mut ymwd: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
