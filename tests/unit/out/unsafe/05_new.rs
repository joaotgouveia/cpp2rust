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
pub struct A {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct D {}
impl D {
    pub unsafe fn operator_call(&self, mut ptr: *const A) {
        {
            let __p = ptr;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p.cast_mut()))
            }
        };
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut d: *mut () = (Box::leak(Box::new(0_())) as *mut ());
    (*d) = 5_();
    {
        let __p = d;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut c: *const () = (Box::leak(Box::new(3_())) as *mut ()).cast_const();
    {
        let __p = c;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p.cast_mut()))
        }
    };
    (unsafe {
        D::operator_call(
            &<D>::default(),
            (Box::leak(Box::new(<A>::default())) as *mut A).cast_const(),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
