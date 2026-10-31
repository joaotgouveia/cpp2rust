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
pub struct test_macros_detail_is_same_std_chrono_year_month_weekday__std_chrono_year_month_weekday_
{}
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
    let Tuesday: () = unimplemented!();
    let February: () = unimplemented!();
    {
        let Feb2018: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "(Feb2018/weekday_indexed{{Tuesday, 2}}).year()    == year{{2018}}"
        );;
        const _: () = assert!(
            true,
            "(Feb2018/weekday_indexed{{Tuesday, 2}}).month()   == February"
        );;
        const _: () = assert!(
            true,
            "(Feb2018/weekday_indexed{{Tuesday, 2}}).weekday() == Tuesday"
        );;
        let mut i: i32 = 1000;
        'loop_: while ((i) < (1010)) {
            let mut j: i32 = 1;
            'loop_: while ((j) <= (12)) {
                let mut k: u32 = 0_u32;
                'loop_: while ((k) <= (6_u32)) {
                    let mut l: u32 = 1_u32;
                    'loop_: while ((l) <= (5_u32)) {
                        let mut y: () = unimplemented!();
                        let mut m: () = unimplemented!();
                        let mut wd: () = unimplemented!();
                        let mut ymd: () = unimplemented!();
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        l.prefix_inc();
                    }
                    k.prefix_inc();
                }
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let Feb1stTues: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "(year{{2018}}/Feb1stTues).year()    == year{{2018}}");;
        const _: () = assert!(true, "(year{{2018}}/Feb1stTues).month()   == February");;
        const _: () = assert!(true, "(year{{2018}}/Feb1stTues).weekday() == Tuesday");;
        let mut i: i32 = 1000;
        'loop_: while ((i) < (1010)) {
            let mut j: i32 = 1;
            'loop_: while ((j) <= (12)) {
                let mut k: u32 = 0_u32;
                'loop_: while ((k) <= (6_u32)) {
                    let mut l: u32 = 1_u32;
                    'loop_: while ((l) <= (5_u32)) {
                        let mut y: () = unimplemented!();
                        let mut m: () = unimplemented!();
                        let mut wd: () = unimplemented!();
                        let mut mwd: () = unimplemented!();
                        let mut ymd1: () = unimplemented!();
                        let mut ymd2: () = unimplemented!();
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        l.prefix_inc();
                    }
                    k.prefix_inc();
                }
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let Feb1stTues: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "(2018/Feb1stTues).year()    == year{{2018}}");;
        const _: () = assert!(true, "(2018/Feb1stTues).month()   == February");;
        const _: () = assert!(true, "(2018/Feb1stTues).weekday() == Tuesday");;
        let mut i: i32 = 1000;
        'loop_: while ((i) < (1010)) {
            let mut j: i32 = 1;
            'loop_: while ((j) <= (12)) {
                let mut k: u32 = 0_u32;
                'loop_: while ((k) <= (6_u32)) {
                    let mut l: u32 = 1_u32;
                    'loop_: while ((l) <= (5_u32)) {
                        let mut y: () = unimplemented!();
                        let mut m: () = unimplemented!();
                        let mut wd: () = unimplemented!();
                        let mut mwd: () = unimplemented!();
                        let mut ymd1: () = unimplemented!();
                        let mut ymd2: () = unimplemented!();
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        assert!(unimplemented!());
                        l.prefix_inc();
                    }
                    k.prefix_inc();
                }
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
