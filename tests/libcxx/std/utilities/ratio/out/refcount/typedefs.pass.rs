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
pub fn __cpp2rust_init_globals() {}
