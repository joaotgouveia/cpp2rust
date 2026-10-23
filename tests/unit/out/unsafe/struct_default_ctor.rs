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
pub struct S {
    pub a: (),
    pub b: (),
}
impl S {
    pub unsafe fn new() -> Self {
        let mut this = Self { a: 11_(), b: true };
        this
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Declared {
    pub v: (),
}
impl Declared {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Holder {
    pub items: [S; 2],
}
pub static mut kInit_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 3_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct FromStatic {
    pub v: (),
}
impl Default for FromStatic {
    fn default() -> Self {
        unsafe {
            FromStatic {
                v: (*std::cell::LazyCell::force_mut(&mut *&raw mut kInit_0)),
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut d: *mut Declared = std::ptr::null_mut();
    unimplemented!();
    let mut s: S = S::new();
    unimplemented!();
    unimplemented!();
    let mut h: *mut Holder = Box::leak(
        (0..1_())
            .map(|_| <Holder>::default())
            .collect::<Box<[Holder]>>(),
    )
    .as_mut_ptr();
    unimplemented!();
    {
        let __p = h;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<Holder>(),
            )))
        }
    };
    let mut fs: *mut FromStatic = Box::leak(
        (0..2_())
            .map(|_| <FromStatic>::default())
            .collect::<Box<[FromStatic]>>(),
    )
    .as_mut_ptr();
    unimplemented!();
    {
        let __p = fs;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<FromStatic>(),
            )))
        }
    };
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const kInit_0);
}
