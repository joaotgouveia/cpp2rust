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
    pub count: *mut i32,
}
impl NonTDtor {
    pub unsafe fn new(mut a: *mut i32, mut _a1: *mut i32) -> Self {
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
    pub count: *mut i32,
}
impl NonTDtor1 {
    pub unsafe fn new(mut _a0: *mut i32, mut b: *mut i32) -> Self {
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
pub unsafe fn test_0() -> bool {
    {
        const _: () = assert!(true, "std::is_trivially_destructible<V>::value");;
        let mut v: () = unimplemented!();
    }
    {
        const _: () = assert!(true, "!std::is_trivially_destructible<V>::value");;
        {
            let mut count0: i32 = 0;
            let mut count1: i32 = 0;
            {
                let mut v: () = unimplemented!();
                assert!(((count0) == (0)));
                assert!(((count1) == (0)));
            }
            assert!(((count0) == (1)));
            assert!(((count1) == (0)));
        }
        {
            let mut count0: i32 = 0;
            let mut count1: i32 = 0;
            {
                let mut v: () = unimplemented!();
            }
            assert!(((count0) == (0)));
            assert!(((count1) == (0)));
        }
        {
            let mut count0: i32 = 0;
            let mut count1: i32 = 0;
            {
                let mut v: () = unimplemented!();
                assert!(((count0) == (0)));
                assert!(((count1) == (0)));
            }
            assert!(((count0) == (0)));
            assert!(((count1) == (1)));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
