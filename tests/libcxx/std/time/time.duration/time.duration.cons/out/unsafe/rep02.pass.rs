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
    let mut d: () = {
        let mut __tmp_0: () = 5_();
        std_chrono_duration_double__std_ratio_1__1__::new_1({ &mut __tmp_0 })
    };
    unimplemented!();
    let d2: () = {
        let mut __tmp_1: () = 5_();
        std_chrono_duration_double__std_ratio_1__1__::new_1({ &mut __tmp_1 })
    };
    const _: () = assert!(true, "d2.count() == 5");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
