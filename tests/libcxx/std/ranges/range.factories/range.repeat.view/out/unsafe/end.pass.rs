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
        let mut rv: () = unimplemented!();
        unimplemented!();
        let mut iter: () = unimplemented!();
        const _: () = assert!(true, "std::same_as<decltype(*iter), const int&>");;
        'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
            let i: *const () = &(*unimplemented!());
            unimplemented!();
        }
    }
    {
        let mut rv: () = unimplemented!();
        unimplemented!();
        const _: () = assert!(
            true,
            "std::same_as<decltype(rv.end()), std::unreachable_sentinel_t>"
        );;
        const _: () = assert!(true, "noexcept(rv.end())");;
        let mut __tmp_0: std_ranges_repeat_view_int__long_ = unimplemented!();
        let __range1: *mut std_ranges_repeat_view_int__long_ = &mut __tmp_0;
        'loop_: for i in IterRange::new(unimplemented!(), unimplemented!()) {
            let i: *const () = &(*unimplemented!());
            unimplemented!();
        }
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
