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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut t1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p5: () = unimplemented!();
        const _: () = assert!(true, "std::get<int>(p5) == 1");;
        const _: () = assert!(true, "std::get<const int>(p5) == 2");;
    }
    {
        let p5: () = unimplemented!();
        let i1: *const () = (unsafe { get_0(&p5) });
        let i2: *const () = (unsafe { get_1(&p5) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t: () = unimplemented!();
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t: () = unimplemented!();
        let p: *const () = (unsafe { get_2(&t) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = 42_();
        let mut y: () = 43_();
        let t: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::is_same<int&, decltype(std::get<int&>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&>(std::move(t)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&, decltype(std::get<int const&>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&>(std::move(t)))");;
    }
    {
        let mut x: () = 42_();
        let mut y: () = 43_();
        let t: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::is_same<int&&, decltype(std::get<int&&>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&&>(std::move(t)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&&, decltype(std::get<int const&&>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&&>(std::move(t)))");;
    }
    {
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::get<int>(std::move(t)) == 1");;
        const _: () = assert!(true, "std::get<const int>(std::move(t)) == 2");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
