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
pub struct Pointer {
    pub value: (),
}
impl Pointer {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct A {
    pub n: (),
}
impl A {
    pub unsafe fn new() -> Self {
        let mut this = Self { n: 42_() };
        this
    }
    pub unsafe fn operator_addr(&self) {}
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new() }
    }
}
pub static mut i_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut d_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_f64 });
pub static mut a_2: std::cell::LazyCell<A> = std::cell::LazyCell::new(|| unsafe { A::new() });

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
    const _: () = assert!(true, "std::addressof(i) == &i");;
    const _: () = assert!(true, "std::addressof(d) == &d");;
    let ap: *const A = unimplemented!();
    const _: () = assert!(true, "&ap->n == &a.n");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const i_0);
    std::cell::LazyCell::force(&*&raw const d_1);
    std::cell::LazyCell::force(&*&raw const a_2);
}
