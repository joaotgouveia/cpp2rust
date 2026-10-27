extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> () {
    const _: () = assert!(true, "std::is_empty_v<std::unreachable_sentinel_t>");;
    const _: () = assert!(true, "std::semiregular<std::unreachable_sentinel_t>");;
    const _: () = assert!(
        true,
        "std::same_as<decltype(std::unreachable_sentinel), const std::unreachable_sentinel_t>"
    );;
    let sentinel: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let i: Value<()> = Rc::new(RefCell::new(42_()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let p: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::null()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(
        true,
        "weakly_equality_comparable_with<std::unreachable_sentinel_t, int>"
    );;
    const _: () = assert!(
        true,
        "weakly_equality_comparable_with<std::unreachable_sentinel_t, int*>"
    );;
    const _: () = assert!(
        !(false),
        "!weakly_equality_comparable_with<std::unreachable_sentinel_t, void*>"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return true;
}
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
