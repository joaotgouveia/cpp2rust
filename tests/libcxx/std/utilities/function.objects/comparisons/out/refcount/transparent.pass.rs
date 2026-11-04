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
    const _: () = assert!(true, "!is_transparent<std::less<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::less<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::less<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::less<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::less_equal<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::less_equal<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::less_equal<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::less_equal<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::equal_to<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::equal_to<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::equal_to<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::equal_to<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::not_equal_to<int>>::value");;
    const _: () = assert!(
        true,
        "!is_transparent<std::not_equal_to<std::string>>::value"
    );;
    const _: () = assert!(true, "is_transparent<std::not_equal_to<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::not_equal_to<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::greater<int>>::value");;
    const _: () = assert!(true, "!is_transparent<std::greater<std::string>>::value");;
    const _: () = assert!(true, "is_transparent<std::greater<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::greater<>>::value");;
    const _: () = assert!(true, "!is_transparent<std::greater_equal<int>>::value");;
    const _: () = assert!(
        true,
        "!is_transparent<std::greater_equal<std::string>>::value"
    );;
    const _: () = assert!(true, "is_transparent<std::greater_equal<void>>::value");;
    const _: () = assert!(true, "is_transparent<std::greater_equal<>>::value");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
