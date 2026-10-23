extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut has_ceil_v_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { true });
pub static mut has_ceil_v_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
const _: () = assert!(true, "has_ceil_v<std::chrono::seconds>");
const _: () = assert!(true, "!has_ceil_v<int>");
pub unsafe fn test_2(df: *const (), d: *const ()) {
    {
        let mut f: () = unimplemented!();
        let mut t: () = unimplemented!();
        const _: () = assert!(true, "(std::is_same<R, ToTimePoint>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_3(df: *const (), d: *const ()) {
    {
        let mut f: () = unimplemented!();
        let mut t: () = unimplemented!();
        const _: () = assert!(true, "(std::is_same<R, ToTimePoint>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_constexpr_4() {
    {
        let f: () = unimplemented!();
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::chrono::ceil<ToDuration>(f) == t");;
    }
}
pub unsafe fn test_constexpr_5() {
    {
        let f: () = unimplemented!();
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::chrono::ceil<ToDuration>(f) == t");;
    }
}
pub unsafe fn test_constexpr_6() {
    {
        let f: () = unimplemented!();
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::chrono::ceil<ToDuration>(f) == t");;
    }
}
pub unsafe fn test_constexpr_7() {
    {
        let f: () = unimplemented!();
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::chrono::ceil<ToDuration>(f) == t");;
    }
}
pub unsafe fn test_constexpr_8() {
    {
        let f: () = unimplemented!();
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::chrono::ceil<ToDuration>(f) == t");;
    }
}
pub unsafe fn test_constexpr_9() {
    {
        let f: () = unimplemented!();
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::chrono::ceil<ToDuration>(f) == t");;
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
    (unsafe {
        let mut _df: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_2(&mut _df, &mut _d)
    });
    (unsafe {
        let mut _df: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_2(&mut _df, &mut _d)
    });
    (unsafe {
        let mut _df: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_3(&mut _df, &mut _d)
    });
    (unsafe {
        let mut _df: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_3(&mut _df, &mut _d)
    });
    (unsafe { test_constexpr_4() });
    (unsafe { test_constexpr_5() });
    (unsafe { test_constexpr_6() });
    (unsafe { test_constexpr_7() });
    (unsafe { test_constexpr_8() });
    (unsafe { test_constexpr_9() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const has_ceil_v_0);
    std::cell::LazyCell::force(&*&raw const has_ceil_v_1);
}
