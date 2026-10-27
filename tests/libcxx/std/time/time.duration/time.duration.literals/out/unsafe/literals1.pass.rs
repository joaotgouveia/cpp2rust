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
    let mut h: () = (unsafe { operator_literal_h_0(4_()) });
    unimplemented!();
    let mut h2: () = (unsafe { operator_literal_h_1(4_f64) });
    unimplemented!();
    let mut min: () = (unsafe { operator_literal_min_2(36_()) });
    unimplemented!();
    let mut min2: () = (unsafe { operator_literal_min_3(36_f64) });
    unimplemented!();
    let mut s: () = (unsafe { operator_literal_s_4(24_()) });
    unimplemented!();
    let mut s2: () = (unsafe { operator_literal_s_5(24_f64) });
    unimplemented!();
    let mut ms: () = (unsafe { operator_literal_ms_6(247_()) });
    unimplemented!();
    let mut ms2: () = (unsafe { operator_literal_ms_7(247_f64) });
    unimplemented!();
    let mut us: () = (unsafe { operator_literal_us_8(867_()) });
    unimplemented!();
    let mut us2: () = (unsafe { operator_literal_us_9(867_f64) });
    unimplemented!();
    let mut ns: () = (unsafe { operator_literal_ns_10(645_()) });
    unimplemented!();
    let mut ns2: () = (unsafe { operator_literal_ns_11(645_f64) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
