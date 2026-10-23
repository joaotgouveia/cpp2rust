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
pub struct NonTDtor {
    pub count: *mut (),
}
impl NonTDtor {
    pub unsafe fn new(mut a: *mut (), mut _a1: *mut ()) -> Self {
        let mut this = Self { count: a };
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*self.count).prefix_inc();
    }
}
const _: () = assert!(true, "!std::is_trivially_destructible<NonTDtor>::value");
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct NonTDtor1 {
    pub count: *mut (),
}
impl NonTDtor1 {
    pub unsafe fn new(mut _a0: *mut (), mut b: *mut ()) -> Self {
        let mut this = Self { count: b };
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*self.count).prefix_inc();
    }
}
const _: () = assert!(true, "!std::is_trivially_destructible<NonTDtor1>::value");
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct TDtor {}
impl TDtor {
    pub unsafe fn copy_from(_a0: *const TDtor) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const TDtor) -> *mut TDtor {
        return &mut (*(self as *mut TDtor));
    }
}
impl Clone for TDtor {
    fn clone(&self) -> Self {
        unsafe { TDtor::copy_from(self as *const TDtor) }
    }
}
const _: () = assert!(true, "!std::is_trivially_copy_constructible<TDtor>::value");
const _: () = assert!(true, "std::is_trivially_destructible<TDtor>::value");
pub unsafe fn test_0() -> () {
    {
        const _: () = assert!(true, "std::is_trivially_destructible<V>::value");;
        let mut v: () = unimplemented!();
    }
    {
        const _: () = assert!(true, "!std::is_trivially_destructible<V>::value");;
        {
            let mut count0: () = 0_();
            let mut count1: () = 0_();
            {
                let mut v: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
        }
        {
            let mut count0: () = 0_();
            let mut count1: () = 0_();
            {
                let mut v: () = unimplemented!();
            }
            unimplemented!();
            unimplemented!();
        }
        {
            let mut count0: () = 0_();
            let mut count1: () = 0_();
            {
                let mut v: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            unimplemented!();
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
