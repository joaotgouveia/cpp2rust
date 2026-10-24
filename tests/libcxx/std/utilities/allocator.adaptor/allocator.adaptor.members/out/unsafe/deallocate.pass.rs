extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut deallocate_called_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct A1_int_ {
    id_: (),
}
impl A1_int_ {
    pub unsafe fn new(mut id: Option<()>) -> Self {
        let mut id: () = id.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { id_: id };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocate_called_3)) = unimplemented!();
    }
}
impl Default for A1_int_ {
    fn default() -> Self {
        unsafe { A1_int_::new(None) }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct A2_int_ {
    id_: (),
}
impl A2_int_ {
    pub unsafe fn new(mut id: Option<()>) -> Self {
        let mut id: () = id.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { id_: id };
        this
    }
}
impl Default for A2_int_ {
    fn default() -> Self {
        unsafe { A2_int_::new(None) }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct A3_int_ {
    id_: (),
}
impl A3_int_ {
    pub unsafe fn new(mut id: Option<()>) -> Self {
        let mut id: () = id.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { id_: id };
        this
    }
}
impl Default for A3_int_ {
    fn default() -> Self {
        unsafe { A3_int_::new(None) }
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
    {
        let mut a: () = unimplemented!();
        (unsafe { a.deallocate((10_() as *mut ()), 20_()) });
        unimplemented!();
    }
    {
        let mut a: () = unimplemented!();
        (unsafe { a.deallocate((10_() as *mut ()), 20_()) });
        unimplemented!();
    }
    {
        let mut a: () = unimplemented!();
        (unsafe { a.deallocate((10_() as *mut ()), 20_()) });
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const deallocate_called_3);
}
