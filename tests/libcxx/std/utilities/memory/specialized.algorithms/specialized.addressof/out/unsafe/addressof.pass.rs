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
impl A {
    pub unsafe fn operator_addr(&self) {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct nothing {}
impl nothing {
    pub unsafe fn to_pmut(&mut self) -> *mut () {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_0));
    }
}
static mut c_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });

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
        let mut i: () = unimplemented!();
        let mut d: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut tp: *mut A = (Box::leak(Box::new(<A>::default())) as *mut A);
        let mut ctp: *const A = (tp).cast_const();
        unimplemented!();
        unimplemented!();
        {
            let __p = tp;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
    }
    {
        let mut anon_1: anon_1 = <anon_1>::default();
        unimplemented!();
    }
    return 0_();
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_1 {
    pub n: nothing,
    pub i: (),
}
impl Default for anon_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
