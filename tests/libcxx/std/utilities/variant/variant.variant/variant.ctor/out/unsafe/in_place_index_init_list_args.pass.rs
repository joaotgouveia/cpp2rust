extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_convertible_imp_0(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_imp_1(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_imp_2(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_imp_3(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_imp_4(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_imp_5(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_6() -> bool {
    return (unsafe { test_convertible_imp_0(0_i64) });
}
pub unsafe fn test_convertible_7() -> bool {
    return (unsafe { test_convertible_imp_1(0_i64) });
}
pub unsafe fn test_convertible_8() -> bool {
    return (unsafe { test_convertible_imp_2(0_i64) });
}
pub unsafe fn test_convertible_9() -> bool {
    return (unsafe { test_convertible_imp_3(0_i64) });
}
pub unsafe fn test_convertible_10() -> bool {
    return (unsafe { test_convertible_imp_4(0_i64) });
}
pub unsafe fn test_convertible_11() -> bool {
    return (unsafe { test_convertible_imp_5(0_i64) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Vec<i32 > , new, value)]
pub struct InitList {
    pub size: usize,
}
impl InitList {
    pub unsafe fn new(mut il: Vec<i32>) -> Self {
        let mut this = Self { size: il.len() };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct InitListArg {
    pub size: usize,
    pub value: i32,
}
impl InitListArg {
    pub unsafe fn new(mut il: Vec<i32>, mut v: i32) -> Self {
        let mut this = Self {
            size: il.len(),
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
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 0");;
        const _: () = assert!(true, "std::get<0>(v).size == 3");;
    }
    {
        let v: () = unimplemented!();
        const _: () = assert!(true, "v.index() == 2");;
        const _: () = assert!(true, "std::get<2>(v).size == 3");;
    }
    {
        let v: () = unimplemented!();
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_ctor_basic_13() });
    (unsafe { test_ctor_sfinae_12() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
