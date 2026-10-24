extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct X {
    _cb: (),
}
impl X {
    unsafe fn destructor(&mut self) {}
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct IncompleteReturnType {
    pub fn_: (),
}
pub static mut called_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn test_fn_1() -> IncompleteReturnType {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut called_0)).prefix_inc();
    let mut I: IncompleteReturnType = <IncompleteReturnType>::default();
    return I.clone();
}
pub unsafe fn test_pr34298_2() {
    const _: () = assert!(
        true,
        "std::is_copy_constructible<IncompleteReturnType>::value"
    );;
    const _: () = assert!(true, "std::is_copy_assignable<IncompleteReturnType>::value");;
    {
        let mut X: IncompleteReturnType = <IncompleteReturnType>::default();
        unimplemented!();
        let CX: *const IncompleteReturnType = &X;
        let mut X2: IncompleteReturnType = (*CX).clone();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut Empty: IncompleteReturnType = <IncompleteReturnType>::default();
        let mut X2: IncompleteReturnType = Empty.clone();
        unimplemented!();
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
    (unsafe { test_pr34298_2() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const called_0);
}
