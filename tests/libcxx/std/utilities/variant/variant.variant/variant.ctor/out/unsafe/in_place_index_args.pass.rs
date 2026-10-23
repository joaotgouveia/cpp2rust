extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_convertible_imp_0(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_imp_1(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_imp_2(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_imp_3(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_imp_4(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_5() -> () {
    return (unsafe { test_convertible_imp_0(0_()) });
}
pub unsafe fn test_convertible_6() -> () {
    return (unsafe { test_convertible_imp_1(0_()) });
}
pub unsafe fn test_convertible_7() -> () {
    return (unsafe { test_convertible_imp_2(0_()) });
}
pub unsafe fn test_convertible_8() -> () {
    return (unsafe { test_convertible_imp_3(0_()) });
}
pub unsafe fn test_convertible_9() -> () {
    return (unsafe { test_convertible_imp_4(0_()) });
}
pub unsafe fn test_ctor_sfinae_10() {
    {
        const _: () = assert!(
            true,
            "std::is_constructible<V, std::in_place_index_t<0>, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_index_t<0>, int>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "std::is_constructible<V, std::in_place_index_t<1>, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_index_t<1>, int>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "std::is_constructible<V, std::in_place_index_t<2>, int *>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_index_t<2>, int *>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_index_t<0>, int *>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_index_t<0>, int *>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_index_t<3>, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_index_t<3>, int>()"
        );;
    }
}
pub unsafe fn test_ctor_basic_11() {
    {
        let v: () = {
            let mut __tmp_0: () = 42_();
            std_variant_int_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_12)).clone() },
                { &mut __tmp_0 },
            )
        };
        const _: () = assert!(true, "v.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v) == 42");;
    }
    {
        let v: () = {
            let mut __tmp_1: () = 42_();
            std_variant_int__long__long_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_13)).clone() },
                { &mut __tmp_1 },
            )
        };
        const _: () = assert!(true, "v.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v) == 42");;
    }
    {
        let v: () = {
            let mut __tmp_2: () = 42_();
            std_variant_int__const_int__long_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_13)).clone() },
                { &mut __tmp_2 },
            )
        };
        const _: () = assert!(true, "v.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v) == 42");;
    }
    {
        let mut x: () = 42_();
        let mut v: () = std_variant_const_int__volatile_int__int_::new_1(
            { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_12)).clone() },
            { &mut x },
        );
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = 42_();
        let mut v: () = std_variant_const_int__volatile_int__int_::new_2(
            { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_13)).clone() },
            { &mut x },
        );
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = 42_();
        let mut v: () = std_variant_const_int__volatile_int__int_::new_3(
            { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_14)).clone() },
            { &mut x },
        );
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_ctor_basic_11() });
    (unsafe { test_ctor_sfinae_10() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
