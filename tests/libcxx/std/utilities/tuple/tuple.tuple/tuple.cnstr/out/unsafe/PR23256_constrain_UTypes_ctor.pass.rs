extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct UnconstrainedCtor {
    pub value_: (),
}
impl UnconstrainedCtor {
    pub unsafe fn new() -> Self {
        let mut this = Self { value_: 0_() };
        this
    }
}
impl Default for UnconstrainedCtor {
    fn default() -> Self {
        unsafe { UnconstrainedCtor::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ExplicitUnconstrainedCtor {
    pub value_: (),
}
impl ExplicitUnconstrainedCtor {
    pub unsafe fn new() -> Self {
        let mut this = Self { value_: 0_() };
        this
    }
}
impl Default for ExplicitUnconstrainedCtor {
    fn default() -> Self {
        unsafe { ExplicitUnconstrainedCtor::new() }
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
    {
        const _: () = assert!(true, "std::is_copy_constructible<std::tuple<A>>::value");;
        const _: () = assert!(true, "std::is_move_constructible<std::tuple<A>>::value");;
        const _: () = assert!(
            true,
            "std::is_copy_constructible<std::tuple<ExplicitA>>::value"
        );;
        const _: () = assert!(
            true,
            "std::is_move_constructible<std::tuple<ExplicitA>>::value"
        );;
    }
    {
        const _: () = assert!(
            true,
            "std::is_constructible<\n            std::tuple<A>,\n            std::allocator_arg_t, std::allocator<int>,\n            std::tuple<A> const&\n        >::value"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible<\n            std::tuple<A>,\n            std::allocator_arg_t, std::allocator<int>,\n            std::tuple<A> &&\n        >::value"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible<\n            std::tuple<ExplicitA>,\n            std::allocator_arg_t, std::allocator<int>,\n            std::tuple<ExplicitA> const&\n        >::value"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible<\n            std::tuple<ExplicitA>,\n            std::allocator_arg_t, std::allocator<int>,\n            std::tuple<ExplicitA> &&\n        >::value"
        );;
    }
    {
        let mut t: () = unimplemented!();
        (&(t));
        let mut t2: () = unimplemented!();
        (&(t2));
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
