extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct A {
    pub ptr_: Option<Box<A>>,
}
impl A {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ptr_: Some(Box::from_raw(&raw mut this)),
        };
        this
    }
    pub unsafe fn reset(&mut self) {
        {
            let _a0: *mut A = ::std::ptr::null_mut();
            self.ptr_ = if _a0.is_null() {
                None
            } else {
                Some(Box::from_raw(_a0))
            }
        };
    }
    pub unsafe fn move_from(_a0: *mut A) -> Self {
        let mut this = Self {
            ptr_: (*_a0).ptr_.take(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut A) -> *mut A {
        self.ptr_ = (*_a0).ptr_.take();
        return &mut (*(self as *mut A));
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new() }
    }
}
pub unsafe fn test_0() -> bool {
    (unsafe { A::reset(&mut (*(Box::leak(Box::new(A::new())) as *mut A))) });
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
