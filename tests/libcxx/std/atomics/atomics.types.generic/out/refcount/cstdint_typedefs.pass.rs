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
pub fn __cpp2rust_init_globals() {}
