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
    pub v: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(B, new_1, value)]
pub struct A {
    pub x: i32,
    pub y: i32,
}
impl A {
    pub unsafe fn new_1(mut b: B) -> Self {
        let mut this = Self {
            x: b.v,
            y: ((b.v) + (1)),
        };
        this
    }
    pub unsafe fn new_2(mut _x: i32, mut _y: i32) -> Self {
        let mut this = Self { x: _x, y: _y };
        this
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A { x: 111, y: 222 } }
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
pub unsafe fn test_0() -> bool {
    {
        let mut rv: () = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!((((*unimplemented!()).x) == (111)));
        assert!((((*unimplemented!()).y) == (222)));
        assert!(unimplemented!());
    }
    {
        let mut rv: () = unimplemented!();
        assert!((((*unimplemented!()).x) == (111)));
        assert!((((*unimplemented!()).y) == (222)));
        assert!(unimplemented!() != unimplemented!());
    }
    {
        let mut rv: () = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!((((*unimplemented!()).x) == (1)));
        assert!((((*unimplemented!()).y) == (2)));
        assert!(unimplemented!());
    }
    {
        let mut rv: () = unimplemented!();
        assert!((((*unimplemented!()).x) == (1)));
        assert!((((*unimplemented!()).y) == (2)));
        assert!(unimplemented!() != unimplemented!());
    }
    {
        let mut rv: () = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!((((*unimplemented!()).x) == (11)));
        assert!((((*unimplemented!()).y) == (12)));
        assert!(unimplemented!());
    }
    {
        let mut rv: () = unimplemented!();
        assert!((((*unimplemented!()).x) == (10)));
        assert!((((*unimplemented!()).y) == (11)));
        assert!(unimplemented!() != unimplemented!());
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
