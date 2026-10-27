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
pub struct test_macros_detail_is_same_std_chrono_month_day_last__std_chrono_month_day_last_ {}
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
    let February: () = unimplemented!();
    let last: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        let mdl: () = unimplemented!();
        const _: () = assert!(true, "mdl.month() == February");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "(last/February).month() == February");;
        const _: () = assert!(true, "(February/last).month() == February");;
        let mut i: () = 1_();
        'loop_: while ((i) < (12_())) {
            let mut m: () = unimplemented!();
            let mut mdl1: () = unimplemented!();
            let mut mdl2: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "(last/2).month() == February");;
        const _: () = assert!(true, "(2/last).month() == February");;
        let mut i: () = 1_();
        'loop_: while ((i) < (12_())) {
            let mut m: () = unimplemented!();
            let mut mdl1: () = unimplemented!();
            let mut mdl2: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
