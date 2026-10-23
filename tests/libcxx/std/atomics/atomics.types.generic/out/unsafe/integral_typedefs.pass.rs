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
        "(std::is_same<std::atomic<char>, std::atomic_char>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<signed char>, std::atomic_schar>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<unsigned char>, std::atomic_uchar>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<short>, std::atomic_short>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<unsigned short>, std::atomic_ushort>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<int>, std::atomic_int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<unsigned int>, std::atomic_uint>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<long>, std::atomic_long>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<unsigned long>, std::atomic_ulong>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<long long>, std::atomic_llong>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<unsigned long long>, std::atomic_ullong>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<wchar_t>, std::atomic_wchar_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<char8_t>, std::atomic_char8_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<char16_t>, std::atomic_char16_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<char32_t>, std::atomic_char32_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<std::intptr_t>,  std::atomic_intptr_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<std::uintptr_t>, std::atomic_uintptr_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<int8_t>,    std::atomic_int8_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<uint8_t>,   std::atomic_uint8_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<int16_t>,   std::atomic_int16_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<uint16_t>,  std::atomic_uint16_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<int32_t>,   std::atomic_int32_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<uint32_t>,  std::atomic_uint32_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<int64_t>,   std::atomic_int64_t>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<std::atomic<uint64_t>,  std::atomic_uint64_t>::value)"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
