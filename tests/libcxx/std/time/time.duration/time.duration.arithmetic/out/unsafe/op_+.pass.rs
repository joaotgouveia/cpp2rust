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
    {
        let m: () = {
            let mut __tmp_0: () = 3_();
            std_chrono_duration_long__std_ratio_60__1__::new_1({ &mut __tmp_0 })
        };
        let mut m2: () = unimplemented!();
        unimplemented!();
    }
    {
        let m: () = {
            let mut __tmp_1: () = 3_();
            std_chrono_duration_long__std_ratio_60__1__::new_1({ &mut __tmp_1 })
        };
        let m2: () = unimplemented!();
        const _: () = assert!(true, "m.count() == m2.count()");;
    }
    {
        let mut zero: () = {
            let mut __tmp_2: () = 0_();
            std_chrono_duration_int__std_ratio_10__10__::new_1({ &mut __tmp_2 })
        };
        let mut one: () = {
            let mut __tmp_3: () = 1_();
            std_chrono_duration_int__std_ratio_10__10__::new_1({ &mut __tmp_3 })
        };
        const _: () = assert!(
            true,
            "(std::is_same< decltype(+one), decltype(zero-one) >::value)"
        );;
        const _: () = assert!(true, "(std::is_same< decltype(zero+one), D1>::value)");;
        const _: () = assert!(true, "(std::is_same< decltype(+one),     D1>::value)");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
