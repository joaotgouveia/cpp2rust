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
        let mut t1: () = {
            let mut __tmp_0: () = 42_();
            let mut __tmp_1: () =
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"Hi".as_ptr() },
                    None,
                );
            let mut __tmp_2: () = unimplemented!();
            std_tuple_int__std_string__std_complex_float__::new(
                { &mut __tmp_0 },
                { &mut __tmp_1 },
                { &mut __tmp_2 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t2: () = {
            let mut __tmp_3: () = 42_();
            let mut __tmp_4: () =
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"Hi".as_ptr() },
                    None,
                );
            let mut __tmp_5: () = 23_();
            let mut __tmp_6: () = unimplemented!();
            std_tuple_int__std_string__int__std_complex_float__::new(
                { &mut __tmp_3 },
                { &mut __tmp_4 },
                { &mut __tmp_5 },
                { &mut __tmp_6 },
            )
        };
        unimplemented!();
        unimplemented!();
    }
    {
        let p5: () = {
            let mut __tmp_7: () = 1_();
            let mut __tmp_8: () = 2_();
            let mut __tmp_9: () = 3.4_f64;
            let mut __tmp_10: () = 5.6_f64;
            std_tuple_int__const_int__double__double_::new(
                { &mut __tmp_7 },
                { &mut __tmp_8 },
                { &mut __tmp_9 },
                { &mut __tmp_10 },
            )
        };
        const _: () = assert!(true, "std::get<int>(p5) == 1");;
        const _: () = assert!(true, "std::get<const int>(p5) == 2");;
    }
    {
        let p5: () = {
            let mut __tmp_11: () = 1_();
            let mut __tmp_12: () = 2_();
            let mut __tmp_13: () = 3.4_f64;
            let mut __tmp_14: () = 5.6_f64;
            std_tuple_int__const_int__double__double_::new(
                { &mut __tmp_11 },
                { &mut __tmp_12 },
                { &mut __tmp_13 },
                { &mut __tmp_14 },
            )
        };
        let i1: *const () = (unsafe { get_0(&p5) });
        let i2: *const () = (unsafe { get_1(&p5) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut t: () = std_unique_ptr_int__std_default_delete_int__::new({
            (Box::leak(Box::new(4_())) as *mut ())
        });
        let mut p: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t: () = std_unique_ptr_int__std_default_delete_int__::new({
            (Box::leak(Box::new(4_())) as *mut ())
        });
        let p: *const () = (unsafe { get_2(&t) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = 42_();
        let mut y: () = 43_();
        let t: () = std_tuple_int_ref__const_int_ref_::new({ &mut x }, { &mut y });
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
        let t: () =
            std_tuple_int_refref__const_int_refref_::new({ &mut *(&raw mut x).cast::<()>() }, {
                &mut *(&raw mut y).cast::<()>()
            });
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
        let t: () = {
            let mut __tmp_15: () = 1_();
            let mut __tmp_16: () = 2_();
            let mut __tmp_17: () = 3.4_f64;
            let mut __tmp_18: () = 5.6_f64;
            std_tuple_int__const_int__double__double_::new(
                { &mut __tmp_15 },
                { &mut __tmp_16 },
                { &mut __tmp_17 },
                { &mut __tmp_18 },
            )
        };
        const _: () = assert!(true, "std::get<int>(std::move(t)) == 1");;
        const _: () = assert!(true, "std::get<const int>(std::move(t)) == 2");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
