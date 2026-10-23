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
pub unsafe fn test_convertible_imp_5(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_6() -> () {
    return (unsafe { test_convertible_imp_0(0_()) });
}
pub unsafe fn test_convertible_7() -> () {
    return (unsafe { test_convertible_imp_1(0_()) });
}
pub unsafe fn test_convertible_8() -> () {
    return (unsafe { test_convertible_imp_2(0_()) });
}
pub unsafe fn test_convertible_9() -> () {
    return (unsafe { test_convertible_imp_3(0_()) });
}
pub unsafe fn test_convertible_10() -> () {
    return (unsafe { test_convertible_imp_4(0_()) });
}
pub unsafe fn test_convertible_11() -> () {
    return (unsafe { test_convertible_imp_5(0_()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InitList {
    pub size: (),
}
impl InitList {
    pub unsafe fn new(mut il: ()) -> Self {
        let mut this = Self {
            size: unimplemented!(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InitListArg {
    pub size: (),
    pub value: (),
}
impl InitListArg {
    pub unsafe fn new(mut il: (), mut v: ()) -> Self {
        let mut this = Self {
            size: unimplemented!(),
            value: v,
        };
        this
    }
}
pub unsafe fn test_ctor_sfinae_12() {
    {
        const _: () = assert!(
            true,
            "std::is_constructible<V, std::in_place_index_t<0>, IL>::value"
        );;
        const _: () = assert!(true, "!test_convertible<V, std::in_place_index_t<0>, IL>()");;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_index_t<0>, IL, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_index_t<0>, IL, int>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_index_t<1>, IL>::value"
        );;
        const _: () = assert!(true, "!test_convertible<V, std::in_place_index_t<1>, IL>()");;
    }
    {
        const _: () = assert!(
            true,
            "std::is_constructible<V, std::in_place_index_t<1>, IL, int>::value"
        );;
        const _: () = assert!(
            true,
            "!test_convertible<V, std::in_place_index_t<1>, IL, int>()"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_index_t<2>, IL>::value"
        );;
        const _: () = assert!(true, "!test_convertible<V, std::in_place_index_t<2>, IL>()");;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible<V, std::in_place_index_t<3>, IL>::value"
        );;
        const _: () = assert!(true, "!test_convertible<V, std::in_place_index_t<3>, IL>()");;
    }
}
pub unsafe fn test_ctor_basic_13() {
    {
        let v: () = std_variant_InitList__InitListArg__InitList_::new_1(
            { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_14)).clone() },
            { unimplemented!() },
        );
        const _: () = assert!(true, "v.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v).size == 3");;
    }
    {
        let v: () = std_variant_InitList__InitListArg__InitList_::new_2(
            { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_15)).clone() },
            { unimplemented!() },
        );
        const _: () = assert!(true, "v.index() == 2");;
        const _: () = assert!(true, "std::get<2>(v).size == 3");;
    }
    {
        let v: () = {
            let mut __tmp_0: () = 42_();
            std_variant_InitList__InitListArg__InitListArg_::new(
                { (*std::cell::LazyCell::force_mut(&mut *&raw mut in_place_index_16)).clone() },
                { unimplemented!() },
                { &mut __tmp_0 },
            )
        };
        const _: () = assert!(true, "v.index() == 1");;
        const _: () = assert!(true, "std::get<1>(v).size == 4");;
        const _: () = assert!(true, "std::get<1>(v).value == 42");;
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
    (unsafe { test_ctor_basic_13() });
    (unsafe { test_ctor_sfinae_12() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
