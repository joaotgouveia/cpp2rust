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
pub struct my_ns_MyPairLike {}
const _: () = assert!(
    true,
    "!std::is_constructible_v<std::pair<int,int>, my_ns::MyPairLike const&>"
);
pub unsafe fn test_0() -> () {
    {
        {
            let mut a: () = unimplemented!();
            let mut p: () = unimplemented!();
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
            let mut a: () = unimplemented!();
            let mut p: () = unimplemented!();
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
            let mut data: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
            let a: () = unimplemented!();
            let mut p: () = (unsafe { a() });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        let mut p: () = unimplemented!();
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
        let mut a: () = unimplemented!();
        let mut p: () = unimplemented!();
        &(p);
    }
    {
        {
            let mut a: () = unimplemented!();
            let mut p: () = unimplemented!();
            &(p);
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
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct NoCopy_1 {}
impl NoCopy_1 {
    pub unsafe fn move_from(_a0: *mut NoCopy_1) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct To_2 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FromImplicit_3 {}
impl FromImplicit_3 {
    pub unsafe fn to_To_2(&self) -> To_2 {
        return <To_2>::default();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct FromExplicit_4 {}
impl FromExplicit_4 {
    pub unsafe fn to_To_2(&self) -> To_2 {
        return <To_2>::default();
    }
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
