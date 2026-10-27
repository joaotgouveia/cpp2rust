extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_set_all_0() {
    let mut v: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_set_all_1() {
    let mut v: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_set_all_2() {
    let mut v: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_set_all_3() {
    let mut v: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_set_all_4() {
    let mut v: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_set_all_5() {
    let mut v: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_set_all_6() {
    let mut v: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_set_all_7() {
    let mut v: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_set_all_8() {
    let mut v: () = unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_9() -> () {
    (unsafe { test_set_all_0() });
    (unsafe { test_set_all_1() });
    (unsafe { test_set_all_2() });
    (unsafe { test_set_all_3() });
    (unsafe { test_set_all_4() });
    (unsafe { test_set_all_5() });
    (unsafe { test_set_all_6() });
    (unsafe { test_set_all_7() });
    (unsafe { test_set_all_8() });
    return true;
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
    (unsafe { test_9() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
