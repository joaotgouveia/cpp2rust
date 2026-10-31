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
pub struct C {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct B {
    #[offset(0)]
    pub v: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(B, new_1, value)]
#[byte_size(8)]
pub struct A {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl A {
    pub fn new_1(mut b: B) -> Self {
        Self {
            x: b.v,
            y: (b.v + 1),
        }
    }
    pub fn new_2(mut _x: i32, mut _y: i32) -> Self {
        Self { x: _x, y: _y }
    }
}
impl Default for A {
    fn default() -> Self {
        { A { x: 111, y: 222 } }
    }
}
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::repeat_view<A, int>,\n                                      std::piecewise_construct_t,\n                                      std::tuple<int, int>,\n                                      std::tuple<int>>"
);
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::repeat_view<A, int>,\n                                      std::piecewise_construct_t,\n                                      std::tuple<B>,\n                                      std::tuple<int>>"
);
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::repeat_view<A, int>,\n                                      std::piecewise_construct_t,\n                                      std::tuple<>,\n                                      std::tuple<int>>"
);
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::repeat_view<A>,\n                                      std::piecewise_construct_t,\n                                      std::tuple<int, int>,\n                                      std::tuple<std::unreachable_sentinel_t>>"
);
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::repeat_view<A>,\n                                      std::piecewise_construct_t,\n                                      std::tuple<B>,\n                                      std::tuple<std::unreachable_sentinel_t>>"
);
const _: () = assert!(
    true,
    "std::constructible_from<std::ranges::repeat_view<A>,\n                                      std::piecewise_construct_t,\n                                      std::tuple<>,\n                                      std::tuple<std::unreachable_sentinel_t>>"
);
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::ranges::repeat_view<A, int>,\n                                       std::piecewise_construct_t,\n                                       std::tuple<C>,\n                                       std::tuple<int>>"
);
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::ranges::repeat_view<A>,\n                                       std::piecewise_construct_t,\n                                       std::tuple<C>,\n                                       std::tuple<std::unreachable_sentinel_t>>"
);
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::ranges::repeat_view<A, int>,\n                                       std::piecewise_construct_t,\n                                       std::tuple<int, int, int>,\n                                       std::tuple<int>>"
);
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::ranges::repeat_view<A>,\n                                       std::piecewise_construct_t,\n                                       std::tuple<int, int, int>,\n                                       std::tuple<std::unreachable_sentinel_t>>"
);
const _: () = assert!(
    !(false),
    "!std::constructible_from<std::ranges::repeat_view<A>, std::piecewise_construct_t, std::tuple<B>, std::tuple<int>>"
);
pub fn test_0() -> bool {
    {
        let rv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 3_()));
        assert!(({ (unimplemented!()).clone().x } == 111));
        assert!(({ (unimplemented!()).clone().y } == 222));
        assert!(unimplemented!());
    }
    {
        let rv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ (unimplemented!()).clone().x } == 111));
        assert!(({ (unimplemented!()).clone().y } == 222));
        assert!(unimplemented!() != unimplemented!());
    }
    {
        let rv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 3_()));
        assert!(({ (unimplemented!()).clone().x } == 1));
        assert!(({ (unimplemented!()).clone().y } == 2));
        assert!(unimplemented!());
    }
    {
        let rv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ (unimplemented!()).clone().x } == 1));
        assert!(({ (unimplemented!()).clone().y } == 2));
        assert!(unimplemented!() != unimplemented!());
    }
    {
        let rv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == 3_()));
        assert!(({ (unimplemented!()).clone().x } == 11));
        assert!(({ (unimplemented!()).clone().y } == 12));
        assert!(unimplemented!());
    }
    {
        let rv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ (unimplemented!()).clone().x } == 10));
        assert!(({ (unimplemented!()).clone().y } == 11));
        assert!(unimplemented!() != unimplemented!());
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
