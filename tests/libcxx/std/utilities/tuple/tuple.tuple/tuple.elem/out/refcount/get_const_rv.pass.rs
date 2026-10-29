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
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::is_same<const int&&, decltype(std::get<0>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(t)))");;
        let i: Ptr<()> = unimplemented!();
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::is_same<const std::string&&, decltype(std::get<0>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(t)))");;
        const _: () = assert!(
            true,
            "std::is_same<const int&&, decltype(std::get<1>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<1>(std::move(t)))");;
        let s: Ptr<()> = unimplemented!();
        let i: Ptr<()> = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = 42_();
        let y: Value<()> = Rc::new(RefCell::new(43_()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::is_same<int&, decltype(std::get<0>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(p)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&, decltype(std::get<1>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<1>(std::move(p)))");;
    }
    {
        let mut x: () = 42_();
        let y: Value<()> = Rc::new(RefCell::new(43_()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::is_same<int&&, decltype(std::get<0>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(p)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&&, decltype(std::get<1>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<1>(std::move(p)))");;
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::get<0>(std::move(t)) == 2.718");;
        const _: () = assert!(true, "std::get<1>(std::move(t)) == 5");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
