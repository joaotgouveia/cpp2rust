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
        let p: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::is_same<const std::unique_ptr<int>&&, decltype(std::get<0>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(p)))");;
        let ptr: *const () = (unsafe { get_0(&(*unimplemented!())) });
        unimplemented!();
    }
    {
        let mut x: () = 42_();
        let y: () = 43_();
        let p: () = std_pair_int_ref__const_int_ref_::new_1({ &mut x }, { &y });
        const _: () = assert!(
            true,
            "std::is_same<int&, decltype(std::get<0>(std::move(p)))>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_same<int const&, decltype(std::get<1>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(p)))");;
        const _: () = assert!(true, "noexcept(std::get<1>(std::move(p)))");;
    }
    {
        let mut x: () = 42_();
        let y: () = 43_();
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
        let p1: () = unimplemented!();
        const _: () = assert!(true, "std::get<0>(std::move(p1)) == 3");;
        const _: () = assert!(true, "std::get<1>(std::move(p1)) == 4");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
