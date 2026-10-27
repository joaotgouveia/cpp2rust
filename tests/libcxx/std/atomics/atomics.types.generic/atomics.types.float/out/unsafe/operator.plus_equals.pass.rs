extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_impl_0() {
    const _: () = assert!(
        true,
        "HasVolatilePlusEquals<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() += T(0))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_impl_1() {
    const _: () = assert!(
        true,
        "HasVolatilePlusEquals<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() += T(0))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_impl_2() {
    const _: () = assert!(
        true,
        "HasVolatilePlusEquals<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() += T(0))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_impl_3() {
    const _: () = assert!(
        true,
        "HasVolatilePlusEquals<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() += T(0))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_4() {
    (unsafe { test_impl_0() });
    {
        (unsafe { test_impl_1() });
    }
}
pub unsafe fn test_5() {
    (unsafe { test_impl_2() });
    {
        (unsafe { test_impl_3() });
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
    (unsafe { test_4() });
    (unsafe { test_5() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
