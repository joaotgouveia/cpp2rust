extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct my_ns_MyPairLike {}
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::pair<int,int>, my_ns::MyPairLike const&>"
);
pub fn test_0() -> () {
    {
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            const _: () = assert!(
                true,
                "!std::is_constructible_v<std::pair<int, int>, std::array<int, 1>>"
            );;
            const _: () = assert!(
                true,
                "std::is_constructible_v<std::pair<int, int>, std::array<int, 2>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_constructible_v<std::pair<int, int>, std::array<int, 3>>"
            );;
        }
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            const _: () = assert!(
                true,
                "!std::is_constructible_v<std::pair<int, int>, std::tuple<int>>"
            );;
            const _: () = assert!(
                true,
                "std::is_constructible_v<std::pair<int, int>, std::tuple<int, int>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_constructible_v<std::pair<int, int>, std::tuple<int, int, int>>"
            );;
        }
        {
            let data: Value<Box<[()]>> =
                Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let p: Value<()> = Rc::new(RefCell::new(({ (*a.borrow())() })));
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::pair<long, std::string>, std::tuple<char*, std::string>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::pair<long, std::string>, std::tuple<long, void*>>"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible_v<std::pair<long, std::string>, std::tuple<long, std::string>>"
        );;
    }
    {
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        &(*p.borrow_mut());
    }
    {
        {
            let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let p: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            &(*p.borrow_mut());
            const _: () = assert!(
                true,
                "std::is_convertible_v<std::tuple<FromImplicit, float>, std::pair<To, double>>"
            );;
        }
        {
            const _: () = assert!(
                true,
                "std::is_constructible_v<std::pair<To, int>, std::tuple<FromExplicit, int>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_convertible_v<std::tuple<FromExplicit, int>, std::pair<To, int>>"
            );;
            const _: () = assert!(
                true,
                "std::is_constructible_v<std::pair<int, To>, std::tuple<int, FromExplicit>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_convertible_v<std::tuple<int, FromExplicit>, std::pair<int, To>>"
            );;
            const _: () = assert!(
                true,
                "std::is_constructible_v<std::pair<To, To>, std::tuple<FromExplicit, FromExplicit>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_convertible_v<std::tuple<FromExplicit, FromExplicit>, std::pair<To, To>>"
            );;
        }
    }
    return true;
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, Default)]
#[byte_size(1)]
pub struct NoCopy_1 {}
impl NoCopy_1 {
    pub fn move_from(_a0: Ptr<NoCopy_1>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct To_2 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct FromImplicit_3 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct FromExplicit_4 {}
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
pub trait FromExplicit_4Impl {
    fn to_To_2(&self) -> To_2;
}
impl FromExplicit_4Impl for Ptr<FromExplicit_4> {
    fn to_To_2(&self) -> To_2 {
        return <To_2>::default();
    }
}
pub trait FromImplicit_3Impl {
    fn to_To_2(&self) -> To_2;
}
impl FromImplicit_3Impl for Ptr<FromImplicit_3> {
    fn to_To_2(&self) -> To_2 {
        return <To_2>::default();
    }
}
pub fn __cpp2rust_init_globals() {}
