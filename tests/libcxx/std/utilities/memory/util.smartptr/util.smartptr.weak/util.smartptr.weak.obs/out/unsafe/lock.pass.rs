extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct A {}
impl A {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const A) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_dec();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const A) -> *mut A {
        return &mut (*(self as *mut A));
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        unsafe { A::copy_from(self as *const A) }
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new() }
    }
}
pub static mut count_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });

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
    {
        let mut wp: () = unimplemented!();
        let mut sp: () = unimplemented!();
        assert!(((unimplemented!()) == (0_i64)));
        assert!((unimplemented!()).is_null());
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (0)));
    }
    {
        let mut sp0: () = unimplemented!();
        let mut wp: () = unimplemented!();
        let mut sp: () = unimplemented!();
        assert!(((unimplemented!()) == (2_i64)));
        assert!(((unimplemented!()) == (unimplemented!())));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (1)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (0)));
    {
        let mut sp0: () = unimplemented!();
        let mut wp: () = unimplemented!();
        unimplemented!();
        let mut sp: () = unimplemented!();
        assert!(((unimplemented!()) == (0_i64)));
        assert!((unimplemented!()).is_null());
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (0)));
    }
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
}
