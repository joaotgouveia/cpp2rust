extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test2739_0() {
    let d: () = {
        let mut __tmp_0: () = 5_();
        std_chrono_duration_int__std_ratio_1__1__::new_1({ &mut __tmp_0 })
    };
    let mut t0: () = unimplemented!();
    let mut t1: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test2739_1() {
    let d: () = {
        let mut __tmp_1: () = 5_();
        std_chrono_duration_unsigned_int__std_ratio_1__1__::new_1({ &mut __tmp_1 })
    };
    let mut t0: () = unimplemented!();
    let mut t1: () = unimplemented!();
    unimplemented!();
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
    {
        let mut t1: () = unimplemented!();
        let mut t2: () = unimplemented!();
        unimplemented!();
    }
    {
        let t1: () = unimplemented!();
        let t2: () = unimplemented!();
        const _: () = assert!(true, "t2.time_since_epoch() == Duration2(2995)");;
    }
    (unsafe { test2739_0() });
    (unsafe { test2739_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
