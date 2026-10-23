extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut sink_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn test_cat_1() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_2() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_3() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_4() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_5() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_6() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_7() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_8() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_9() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_10() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_11() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_12() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_13() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
}
pub unsafe fn test_cat_14() {
    const _: () = assert!(
        true,
        "std::is_same<CatT, std::common_comparison_category_t<Args...>>::value"
    );;
    const _: () = assert!(true, "std::is_same<CatT, Expect>::value");;
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
    {
        (unsafe { test_cat_1() });
        (unsafe { test_cat_2() });
        (unsafe { test_cat_3() });
        (unsafe { test_cat_4() });
        (unsafe { test_cat_5() });
        (unsafe { test_cat_6() });
    }
    {
        (unsafe { test_cat_7() });
        (unsafe { test_cat_8() });
        (unsafe { test_cat_9() });
    }
    {
        (unsafe { test_cat_10() });
        (unsafe { test_cat_11() });
    }
    {
        (unsafe { test_cat_12() });
        (unsafe { test_cat_13() });
    }
    {
        (unsafe { test_cat_14() });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const sink_0);
}
