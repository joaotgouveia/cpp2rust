extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int__const_int_ {}
pub unsafe fn test_1(_a0: *const ()) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<T&>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T&>::value == Expected");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T&> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T&> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<T&> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T&> == Expected");;
}
pub unsafe fn test_2(_a0: *const ()) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<T&>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T&>::value == Expected");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T&> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T&> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<T&> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T&> == Expected");;
}
pub unsafe fn test_3(_a0: *const ()) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<T&>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T&>::value == Expected");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T&> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T&> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<T&> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T&> == Expected");;
}
pub unsafe fn test_4(_a0: *const C) {
    const _: () = assert!(true, "std::is_placeholder<T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<T&>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T>::value == Expected");;
    const _: () = assert!(true, "std::is_placeholder<const T&>::value == Expected");;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<T&> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_base_of<std::integral_constant<int, Expected>, std::is_placeholder<const T&> >::value"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::is_placeholder_v<T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<T&> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T> == Expected");;
    const _: () = assert!(true, "std::is_placeholder_v<const T&> == Expected");;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct C {}
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
    (unsafe { test_5(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _1_6))) });
    (unsafe { test_7(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _2_8))) });
    (unsafe { test_9(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _3_10))) });
    (unsafe { test_11(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _4_12))) });
    (unsafe { test_13(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _5_14))) });
    (unsafe { test_15(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _6_16))) });
    (unsafe { test_17(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _7_18))) });
    (unsafe { test_19(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _8_20))) });
    (unsafe { test_21(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _9_22))) });
    (unsafe { test_23(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _10_24))) });
    (unsafe {
        let mut _arg0: () = 4_();
        test_1(&mut _arg0)
    });
    (unsafe {
        let mut _arg0: () = 5.5_f64;
        test_2(&mut _arg0)
    });
    (unsafe {
        let mut _arg0: () = ('a' as ());
        test_3(&mut _arg0)
    });
    (unsafe {
        let mut _arg0: C = <C>::default();
        test_4(&mut _arg0)
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
