extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() -> () {
    {
        let i: Value<()> = Rc::new(RefCell::new(1_()));
        let j: Value<()> = Rc::new(RefCell::new(2_()));
        {
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let cp: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let t1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let i1: Ptr<()> = unimplemented!();
        let i2: Ptr<()> = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = 42_();
        let mut y: () = 43_();
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::is_same<int&, decltype(std::get<int&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&>(std::move(p)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&, decltype(std::get<int const&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&>(std::move(p)))");;
    }
    {
        let x: Value<()> = Rc::new(RefCell::new(42_()));
        let y: Value<()> = Rc::new(RefCell::new(43_()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::is_same<int&&, decltype(std::get<int&&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&&>(std::move(p)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&&, decltype(std::get<int const&&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&&>(std::move(p)))");;
    }
    {
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "std::get<int>(std::move(p)) == 1");;
        const _: () = assert!(true, "std::get<const int>(std::move(p)) == 2");;
    }
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
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let t: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(
            true,
            "std::is_same<const upint&&, decltype(std::get<upint>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<upint>(std::move(t)))");;
        const _: () = assert!(
            true,
            "std::is_same<const int&&, decltype(std::get<int>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int>(std::move(t)))");;
        let p: Ptr<()> = unimplemented!();
        let i: Ptr<()> = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
