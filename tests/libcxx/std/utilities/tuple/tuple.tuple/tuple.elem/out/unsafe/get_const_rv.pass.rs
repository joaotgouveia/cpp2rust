extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
    {
        let t: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::is_same<const int&&, decltype(std::get<0>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(t)))");;
        let i: *const i32 = (unimplemented!() as *const ()).cast::<i32>();
        assert!(((*i) == (3)));
    }
    {
        let t: () = unimplemented!();
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
        let s: *const Vec<libc::c_char> =
            (unimplemented!() as *const ()).cast::<Vec<libc::c_char>>();
        let i: *const i32 = (unimplemented!() as *const ()).cast::<i32>();
        assert!(
            (*s) == std::slice::from_raw_parts(
                c"high".as_ptr(),
                libcc2rs::c_str_len(c"high".as_ptr()) + 1
            )
            .to_vec()
        );
        assert!(((*i) == (5)));
    }
    {
        let mut x: i32 = 42;
        let y: i32 = 43;
        let p: () = unimplemented!();
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
        let mut x: i32 = 42;
        let y: i32 = 43;
        let p: () = unimplemented!();
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
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::get<0>(std::move(t)) == 2.718");;
        const _: () = assert!(true, "std::get<1>(std::move(t)) == 5");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
