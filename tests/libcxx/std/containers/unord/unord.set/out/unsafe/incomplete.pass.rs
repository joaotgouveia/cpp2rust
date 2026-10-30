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
pub struct MyHash_A_ {}
impl MyHash_A_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for MyHash_A_ {
    fn default() -> Self {
        unsafe { MyHash_A_::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct A {
    pub m: (),
    pub it: (),
    pub cit: (),
    pub lit: (),
    pub clit: (),
}
impl A {
    pub unsafe fn move_from(_a0: *mut A) -> Self {
        let mut this = Self {
            m: unimplemented!(),
            it: (*_a0).it,
            cit: (*_a0).cit,
            lit: (*_a0).lit,
            clit: (*_a0).clit,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut A) -> *mut A {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        return &mut (*(self as *mut A));
    }
}
impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const A, other as *const A) }
    }
}
impl std::cmp::Eq for A {}
pub unsafe fn operator_eq_0(L: *const A, R: *const A) -> () {
    return ((L) == (R));
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
    let mut a: A = <A>::default();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
