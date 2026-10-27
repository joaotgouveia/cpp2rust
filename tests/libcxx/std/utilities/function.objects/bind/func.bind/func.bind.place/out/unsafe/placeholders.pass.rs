extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_all_0() -> () {
    (unsafe { test_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _1_2))) });
    (unsafe { test_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _2_4))) });
    (unsafe { test_5(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _3_6))) });
    (unsafe { test_7(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _4_8))) });
    (unsafe { test_9(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _5_10))) });
    (unsafe { test_11(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _6_12))) });
    (unsafe { test_13(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _7_14))) });
    (unsafe { test_15(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _8_16))) });
    (unsafe { test_17(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _9_18))) });
    (unsafe { test_19(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _10_20))) });
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
    (unsafe { test_all_0() });
    const _: () = assert!(true, "test_all()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
