extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};

pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
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
pub fn __cpp2rust_init_globals() {}
