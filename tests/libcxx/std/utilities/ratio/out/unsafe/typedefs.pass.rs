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
    const _: () = assert!(
        true,
        "std::atto::num == 1 && std::atto::den == 1000000000000000000ULL"
    );;
    const _: () = assert!(
        true,
        "std::femto::num == 1 && std::femto::den == 1000000000000000ULL"
    );;
    const _: () = assert!(
        true,
        "std::pico::num == 1 && std::pico::den == 1000000000000ULL"
    );;
    const _: () = assert!(
        true,
        "std::nano::num == 1 && std::nano::den == 1000000000ULL"
    );;
    const _: () = assert!(
        true,
        "std::micro::num == 1 && std::micro::den == 1000000ULL"
    );;
    const _: () = assert!(true, "std::milli::num == 1 && std::milli::den == 1000ULL");;
    const _: () = assert!(true, "std::centi::num == 1 && std::centi::den == 100ULL");;
    const _: () = assert!(true, "std::deci::num == 1 && std::deci::den == 10ULL");;
    const _: () = assert!(true, "std::deca::num == 10ULL && std::deca::den == 1");;
    const _: () = assert!(true, "std::hecto::num == 100ULL && std::hecto::den == 1");;
    const _: () = assert!(true, "std::kilo::num == 1000ULL && std::kilo::den == 1");;
    const _: () = assert!(true, "std::mega::num == 1000000ULL && std::mega::den == 1");;
    const _: () = assert!(
        true,
        "std::giga::num == 1000000000ULL && std::giga::den == 1"
    );;
    const _: () = assert!(
        true,
        "std::tera::num == 1000000000000ULL && std::tera::den == 1"
    );;
    const _: () = assert!(
        true,
        "std::peta::num == 1000000000000000ULL && std::peta::den == 1"
    );;
    const _: () = assert!(
        true,
        "std::exa::num == 1000000000000000000ULL && std::exa::den == 1"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
