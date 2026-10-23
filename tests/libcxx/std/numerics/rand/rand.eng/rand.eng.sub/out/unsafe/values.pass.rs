extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn where_0(_a0: *const ()) {}
pub unsafe fn test1_1() {
    const _: () = assert!(true, "(E::word_size == 24)");;
    const _: () = assert!(true, "(E::short_lag == 10)");;
    const _: () = assert!(true, "(E::long_lag == 24)");;
    const _: () = assert!(true, "(E::min() == 0)");;
    const _: () = assert!(true, "(E::max() == 0xFFFFFF)");;
    const _: () = assert!(true, "(E::default_seed == 19780503u)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut word_size_2))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut short_lag_3))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut long_lag_4))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_5))) });
}
pub unsafe fn test2_6() {
    const _: () = assert!(true, "(E::word_size == 48)");;
    const _: () = assert!(true, "(E::short_lag == 5)");;
    const _: () = assert!(true, "(E::long_lag == 12)");;
    const _: () = assert!(true, "(E::min() == 0)");;
    const _: () = assert!(true, "(E::max() == 0xFFFFFFFFFFFFull)");;
    const _: () = assert!(true, "(E::default_seed == 19780503u)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut word_size_7))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut short_lag_8))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut long_lag_9))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut default_seed_10))) });
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
    (unsafe { test1_1() });
    (unsafe { test2_6() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
