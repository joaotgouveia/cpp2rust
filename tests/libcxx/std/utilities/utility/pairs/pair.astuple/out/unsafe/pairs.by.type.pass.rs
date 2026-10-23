extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> () {
    {
        let mut i: () = 1_();
        let mut j: () = 2_();
        {
            let mut p: () =
                std_pair_int_ref__int_refref_::new({ &mut i }, { &mut *(&raw mut j).cast::<()>() });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let cp: () =
                std_pair_int_ref__int_refref_::new({ &mut i }, { &mut *(&raw mut j).cast::<()>() });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut p: () =
                std_pair_int_refref__int_ref_::new({ &mut *(&raw mut i).cast::<()>() }, { &mut j });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let cp: () =
                std_pair_int_refref__int_ref_::new({ &mut *(&raw mut i).cast::<()>() }, { &mut j });
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut t1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let p1: () = {
            let mut __tmp_0: () = 1_();
            let mut __tmp_1: () = 2_();
            std_pair_int__const_int_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
        };
        let i1: *const () = unimplemented!();
        let i2: *const () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = 42_();
        let y: () = 43_();
        let p: () = std_pair_int_ref__const_int_ref_::new_1({ &mut x }, { &y });
        const _: () = assert!(
            true,
            "std::is_same<int&, decltype(std::get<int&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&>(std::move(p)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&, decltype(std::get<int const&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&>(std::move(p)))");;
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
            "std::is_same<int&&, decltype(std::get<int&&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&&>(std::move(p)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&&, decltype(std::get<int const&&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&&>(std::move(p)))");;
    }
    {
        let p: () = {
            let mut __tmp_2: () = 1_();
            let mut __tmp_3: () = 2_();
            std_pair_int__const_int_::new({ &mut __tmp_2 }, { &mut __tmp_3 })
        };
        const _: () = assert!(true, "std::get<int>(std::move(p)) == 1");;
        const _: () = assert!(true, "std::get<const int>(std::move(p)) == 2");;
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
    {
        let mut t: () = {
            let mut __tmp_4: () = std_unique_ptr_int__std_default_delete_int__::new({
                (Box::leak(Box::new(4_())) as *mut ())
            });
            let mut __tmp_5: () = 42_();
            std_pair_std_unique_ptr_int__std_default_delete_int____int_::new({ &mut __tmp_4 }, {
                &mut __tmp_5
            })
        };
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t: () = {
            let mut __tmp_6: () = std_unique_ptr_int__std_default_delete_int__::new({
                (Box::leak(Box::new(4_())) as *mut ())
            });
            let mut __tmp_7: () = 42_();
            std_pair_std_unique_ptr_int__std_default_delete_int____int_::new({ &mut __tmp_6 }, {
                &mut __tmp_7
            })
        };
        const _: () = assert!(
            true,
            "std::is_same<const upint&&, decltype(std::get<upint>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<upint>(std::move(t)))");;
        const _: () = assert!(
            true,
            "std::is_same<const int&&, decltype(std::get<int>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int>(std::move(t)))");;
        let p: *const () = unimplemented!();
        let i: *const () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
