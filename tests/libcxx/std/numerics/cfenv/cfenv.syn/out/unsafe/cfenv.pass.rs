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
    let mut fenv: () = unimplemented!();
    let mut fex: () = unimplemented!();
    (&(fenv));
    (&(fex));
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::feclearexcept(0)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fegetexceptflag(&fex, 0)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::feraiseexcept(0)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fesetexceptflag(&fex, 0)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fetestexcept(0)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fegetround()), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fesetround(0)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fegetenv(&fenv)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::feholdexcept(&fenv)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::fesetenv(&fenv)), int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::feupdateenv(&fenv)), int>::value)"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
