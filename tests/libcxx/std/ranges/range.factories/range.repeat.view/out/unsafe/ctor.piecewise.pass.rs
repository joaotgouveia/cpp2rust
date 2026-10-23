extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct C {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B {
    pub v: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct A {
    pub x: (),
    pub y: (),
}
impl A {
    pub unsafe fn new_1(mut b: B) -> Self {
        let mut this = Self {
            x: b.v,
            y: ((b.v) + (1_())),
        };
        this
    }
    pub unsafe fn new_2(mut _x: (), mut _y: ()) -> Self {
        let mut this = Self { x: _x, y: _y };
        this
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe {
            A {
                x: 111_(),
                y: 222_(),
            }
        }
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
pub unsafe fn test_0() -> () {
    {
        let mut rv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut rv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut rv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut rv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut rv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut rv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
