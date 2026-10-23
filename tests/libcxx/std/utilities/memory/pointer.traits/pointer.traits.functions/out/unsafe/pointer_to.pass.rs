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
struct A_int__nat {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_int_ {
    pub t_: *mut (),
}
impl A_int_ {
    pub unsafe fn new(mut t: *mut ()) -> Self {
        let mut this = Self { t_: t };
        this
    }
    pub unsafe fn pointer_to(et: *mut ()) -> A_int_ {
        return A_int_::new({ (et) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct A_long__nat {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_long_ {
    pub t_: *mut (),
}
impl A_long_ {
    pub unsafe fn new(mut t: *mut ()) -> Self {
        let mut this = Self { t_: t };
        this
    }
    pub unsafe fn pointer_to(et: *mut ()) -> A_long_ {
        return A_long_::new({ (et) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct A_void__nat {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_void_ {
    pub t_: (),
}
impl A_void_ {}
pub unsafe fn test_0() -> () {
    let mut obj: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_same<Pointer, decltype(std::pointer_traits<Pointer>::pointer_to(obj))>::value"
    );;
    let mut p: A_int_ = (unsafe {
        std___pointer_traits_impl_A_int___void_::pointer_to(&mut *(&raw mut obj).cast::<()>())
    });
    unimplemented!();
    return true;
}
pub unsafe fn test_1() -> () {
    let mut obj: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_same<Pointer, decltype(std::pointer_traits<Pointer>::pointer_to(obj))>::value"
    );;
    let mut p: A_long_ = (unsafe {
        std___pointer_traits_impl_A_long___void_::pointer_to(&mut *(&raw mut obj).cast::<()>())
    });
    unimplemented!();
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
    (unsafe { test_1() });
    {
        &(0_());
    }
    const _: () = assert!(true, "test<A<int> >()");;
    const _: () = assert!(true, "test<A<long> >()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
