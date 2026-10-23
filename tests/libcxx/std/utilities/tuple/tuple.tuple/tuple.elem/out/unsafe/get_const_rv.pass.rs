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
        let t: () = 3_();
        const _: () = assert!(
            true,
            "std::is_same<const int&&, decltype(std::get<0>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(t)))");;
        let i: *const () = (unsafe { get_0(&t) });
        unimplemented!();
    }
    {
        let t: () = {
            let mut __tmp_0: () = 5_();
            std_tuple_std_string__int_::new({ &std::mem::transmute(*b"high\0") }, { &mut __tmp_0 })
        };
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
        let s: *const () = (unsafe { get_1(&t) });
        let i: *const () = (unsafe { get_2(&t) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = 42_();
        let y: () = 43_();
        let p: () = std_tuple_int_ref__const_int_ref_::new({ &mut x }, { &y });
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
        let mut x: () = 42_();
        let y: () = 43_();
        let p: () =
            std_tuple_int_refref__const_int_refref_::new({ &mut *(&raw mut x).cast::<()>() }, {
                &y
            });
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
        let t: () = {
            let mut __tmp_1: () = 2.718_f64;
            let mut __tmp_2: () = 5_();
            std_tuple_double__int_::new({ &mut __tmp_1 }, { &mut __tmp_2 })
        };
        const _: () = assert!(true, "std::get<0>(std::move(t)) == 2.718");;
        const _: () = assert!(true, "std::get<1>(std::move(t)) == 5");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
