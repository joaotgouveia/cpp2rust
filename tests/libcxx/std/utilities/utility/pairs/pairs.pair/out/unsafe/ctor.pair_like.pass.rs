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
pub unsafe fn test_0() -> bool {
    {
        {
            let mut a: Vec<i32> = Vec::from([1, 2]);
            let mut p: (i32, i32) = unimplemented!();
            assert!(((p.0) == (1)));
            assert!(((p.1) == (2)));
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
            let mut p: (i32, i32) = unimplemented!();
            assert!(((p.0) == (1)));
            assert!(((p.1) == (2)));
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
            let mut data: [i32; 5] = [1, 2, 3, 4, 5];
            let a: () = unimplemented!();
            let mut p: (*mut i32, *mut i32) = (unsafe { a() });
            assert!(((p.0) == (data.as_mut_ptr().offset((0) as isize))));
            assert!(((p.1) == (data.as_mut_ptr().offset((5) as isize))));
        }
    }
    {
        let mut a: () = unimplemented!();
        let mut p: (i64, Vec<libc::c_char>) = unimplemented!();
        assert!(((p.0) == (34_i64)));
        assert!(unimplemented!());
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
        let mut p: (NoCopy_1, NoCopy_1) = unimplemented!();
        &(p);
    }
    {
        {
            let mut a: () = unimplemented!();
            let mut p: (To_2, f64) = unimplemented!();
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(To_2, to_To_2)]
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
