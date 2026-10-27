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
pub struct Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct BigType {
    pub buffer: [(); 64],
}
impl Default for BigType {
    fn default() -> Self {
        unsafe {
            BigType {
                buffer: [
                    10_(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                    unimplemented!(),
                ],
            }
        }
    }
}
pub unsafe fn test_empty_0(mut value: ()) {
    {
        let mut result: () = unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "noexcept(SingleView::empty())");;
    }
    {
        let mut sv: () = unimplemented!();
        let mut result: () =
            (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut empty_1))(&mut sv) });
        unimplemented!();
        const _: () = assert!(true, "noexcept(std::ranges::empty(sv))");;
    }
    {
        let sv: () = unimplemented!();
        let mut result: () =
            (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut empty_1))(&sv) });
        unimplemented!();
        const _: () = assert!(true, "noexcept(std::ranges::empty(std::as_const(sv)))");;
    }
}
pub unsafe fn test_empty_2(mut value: Empty) {
    {
        let mut result: () = unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "noexcept(SingleView::empty())");;
    }
    {
        let mut sv: () = unimplemented!();
        let mut result: () =
            (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut empty_1))(&mut sv) });
        unimplemented!();
        const _: () = assert!(true, "noexcept(std::ranges::empty(sv))");;
    }
    {
        let sv: () = unimplemented!();
        let mut result: () =
            (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut empty_1))(&sv) });
        unimplemented!();
        const _: () = assert!(true, "noexcept(std::ranges::empty(std::as_const(sv)))");;
    }
}
pub unsafe fn test_empty_3(mut value: BigType) {
    {
        let mut result: () = unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "noexcept(SingleView::empty())");;
    }
    {
        let mut sv: () = unimplemented!();
        let mut result: () =
            (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut empty_1))(&mut sv) });
        unimplemented!();
        const _: () = assert!(true, "noexcept(std::ranges::empty(sv))");;
    }
    {
        let sv: () = unimplemented!();
        let mut result: () =
            (unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut empty_1))(&sv) });
        unimplemented!();
        const _: () = assert!(true, "noexcept(std::ranges::empty(std::as_const(sv)))");;
    }
}
pub unsafe fn test_4() -> () {
    (unsafe { test_empty_0(92_()) });
    (unsafe { test_empty_2(<Empty>::default()) });
    (unsafe { test_empty_3(<BigType>::default()) });
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
    (unsafe { test_4() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
