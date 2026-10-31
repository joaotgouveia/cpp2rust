extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg)]
pub struct A1_B_ {
    id_: i32,
}
impl A1_B_ {
    pub unsafe fn new(mut id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { id_: id };
        this
    }
}
impl Default for A1_B_ {
    fn default() -> Self {
        unsafe { A1_B_::new(None) }
    }
}
pub static mut constructed_6: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub static mut destroy_called_7: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct A3_B_ {
    id_: i32,
}
impl A3_B_ {
    pub unsafe fn new(mut id: Option<i32>) -> Self {
        let mut id: i32 = id.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn construct_7_struct_B(&mut self, mut p: *mut B) {
        (Box::leak(Box::new(B::new())) as *mut B);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)) = true;
    }
    pub unsafe fn destroy_8_struct_B(&mut self, mut p: *mut B) {
        (unsafe { B::destructor(&mut (*p)) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut destroy_called_7)) = true;
    }
}
impl Default for A3_B_ {
    fn default() -> Self {
        unsafe { A3_B_::new(None) }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct B {}
impl B {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_8)) = true;
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_8)) = false;
    }
}
impl Default for B {
    fn default() -> Self {
        unsafe { B::new() }
    }
}
pub static mut constructed_8: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });

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
        let mut a: () = unimplemented!();
        let mut buf: [libc::c_char; 100] = [(0 as libc::c_char); 100];
        let mut s: *mut B = (buf.as_mut_ptr() as *mut B);
        assert!(!(*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_8)));
        unimplemented!();
        assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_8)));
        unimplemented!();
        assert!(!(*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_8)));
    }
    {
        let mut a: () = unimplemented!();
        let mut buf: [libc::c_char; 100] = [(0 as libc::c_char); 100];
        let mut s: *mut B = (buf.as_mut_ptr() as *mut B);
        assert!(!(*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_8)));
        assert!(!(*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)));
        assert!(!(*std::cell::LazyCell::force_mut(&mut *&raw mut destroy_called_7)));
        unimplemented!();
        assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_8)));
        assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)));
        assert!(!(*std::cell::LazyCell::force_mut(&mut *&raw mut destroy_called_7)));
        unimplemented!();
        assert!(!(*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_8)));
        assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)));
        assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut destroy_called_7)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const constructed_6);
    std::cell::LazyCell::force(&*&raw const destroy_called_7);
    std::cell::LazyCell::force(&*&raw const constructed_8);
}
