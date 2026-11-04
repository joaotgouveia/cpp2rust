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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    const _: () = assert!(true, "!is_transparent<std::plus<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::plus<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::plus<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::plus<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::minus<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::minus<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::minus<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::minus<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::multiplies<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::multiplies<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::multiplies<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::multiplies<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::divides<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::divides<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::divides<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::divides<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::modulus<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::modulus<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::modulus<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::modulus<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::negate<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::negate<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::negate<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::negate<>>::value");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
