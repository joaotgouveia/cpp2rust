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
        "(std::is_same<std::atomic<  std::int_least8_t>,   std::atomic_int_least8_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic< std::uint_least8_t>,  std::atomic_uint_least8_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic< std::int_least16_t>,  std::atomic_int_least16_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<std::uint_least16_t>, std::atomic_uint_least16_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic< std::int_least32_t>,  std::atomic_int_least32_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<std::uint_least32_t>, std::atomic_uint_least32_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic< std::int_least64_t>,  std::atomic_int_least64_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<std::uint_least64_t>, std::atomic_uint_least64_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<  std::int_fast8_t>,   std::atomic_int_fast8_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic< std::uint_fast8_t>,  std::atomic_uint_fast8_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic< std::int_fast16_t>,  std::atomic_int_fast16_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<std::uint_fast16_t>, std::atomic_uint_fast16_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic< std::int_fast32_t>,  std::atomic_int_fast32_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<std::uint_fast32_t>, std::atomic_uint_fast32_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic< std::int_fast64_t>,  std::atomic_int_fast64_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<std::uint_fast64_t>, std::atomic_uint_fast64_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic< std::intptr_t>,  std::atomic_intptr_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<std::uintptr_t>, std::atomic_uintptr_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<   std::size_t>,    std::atomic_size_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<std::ptrdiff_t>, std::atomic_ptrdiff_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic< std::intmax_t>,  std::atomic_intmax_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<std::uintmax_t>, std::atomic_uintmax_t>::value)"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
