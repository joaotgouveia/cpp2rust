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
        let p: () = {
            let mut __tmp_0: () = std_unique_ptr_int__std_default_delete_int__::new({
                (Box::leak(Box::new(3_())) as *mut ())
            });
            let mut __tmp_1: () = 4_();
            std_pair_std_unique_ptr_int__std_default_delete_int____short_::new({ &mut __tmp_0 }, {
                &mut __tmp_1
            })
        };
        const _: () = assert!(
            true,
            "std::is_same<const std::unique_ptr<int>&&, decltype(std::get<0>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(p)))");;
        let ptr: *const () = (unsafe { get_0(&p) });
        unimplemented!();
    }
    {
        let mut x: () = 42_();
        let y: () = 43_();
        let p: () = std_pair_int_ref__const_int_ref_::new({ &mut x }, { &y });
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
        let p: () =
            std_pair_int_refref__const_int_refref_::new({ &mut *(&raw mut x).cast::<()>() }, {
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
        let p1: () = {
            let mut __tmp_2: () = 3_();
            let mut __tmp_3: () = 4_();
            std_pair_int__short_::new({ &mut __tmp_2 }, { &mut __tmp_3 })
        };
        const _: () = assert!(true, "std::get<0>(std::move(p1)) == 3");;
        const _: () = assert!(true, "std::get<1>(std::move(p1)) == 4");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
