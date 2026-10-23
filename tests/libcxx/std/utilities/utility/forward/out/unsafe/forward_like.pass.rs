extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct U {}
pub static mut u_0: std::cell::LazyCell<U> = std::cell::LazyCell::new(|| unsafe { <U>::default() });
pub static mut cu_1: std::cell::LazyCell<*const U> =
    std::cell::LazyCell::new(|| unsafe { &(*std::cell::LazyCell::force_mut(&mut *&raw mut u_0)) });
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T>(U{{}})), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T>(CU{{}})), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T>(u)), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T>(cu)), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T>(std::move(u))), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T>(std::move(cu))), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT>(U{{}})), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT>(CU{{}})), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT>(u)), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT>(cu)), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT>(std::move(u))), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT>(std::move(cu))), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(U{{}})), U&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(CU{{}})), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(u)), U&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(cu)), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(std::move(u))), U&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(std::move(cu))), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(U{{}})), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(CU{{}})), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(u)), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(cu)), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(std::move(u))), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(std::move(cu))), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(U{{}})), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(CU{{}})), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(u)), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(cu)), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(std::move(u))), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(std::move(cu))), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(U{{}})), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(CU{{}})), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(u)), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(cu)), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(std::move(u))), CU&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(std::move(cu))), CU&&>"
);
const _: () = assert!(true, "noexcept(std::forward_like<T>(u))");
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<U&>(u)), U&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CU&>(cu)), CU&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<U&&>(std::move(u))), U&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CU&&>(std::move(cu))), CU&&>"
);
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NoCtorCopyMove {}
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&&>(std::declval<NoCtorCopyMove>())), const NoCtorCopyMove&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<CT&>(std::declval<NoCtorCopyMove>())), const NoCtorCopyMove&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&&>(std::declval<NoCtorCopyMove>())), NoCtorCopyMove&&>"
);
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::forward_like<T&>(std::declval<NoCtorCopyMove>())), NoCtorCopyMove&>"
);
const _: () = assert!(
    true,
    "noexcept(std::forward_like<T>(std::declval<NoCtorCopyMove>()))"
);
pub unsafe fn test_2() -> () {
    {
        let mut val: () = 1729_();
        let result: *const () = &val;
        const _: () = assert!(true, "std::is_same_v<decltype(result), const int&>");;
        unimplemented!();
    }
    {
        let mut val: () = 1729_();
        let result: *mut () = &mut val;
        const _: () = assert!(true, "std::is_same_v<decltype(result), int&>");;
        unimplemented!();
    }
    {
        let mut val: () = 1729_();
        let result: *const () = &*(&raw const val).cast::<()>();
        const _: () = assert!(true, "std::is_same_v<decltype(result), const int&&>");;
        unimplemented!();
    }
    {
        let mut val: () = 1729_();
        let result: *mut () = &mut val;
        const _: () = assert!(true, "std::is_same_v<decltype(result), int&&>");;
        unimplemented!();
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
    (unsafe { test_2() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const u_0);
    std::cell::LazyCell::force(&*&raw const cu_1);
}
