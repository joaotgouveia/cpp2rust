extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut A_constructed_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct A {
    pub data_: (),
}
impl A {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut A_constructed_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(a: *const A) -> Self {
        let mut this = Self { data_: (*a).data_ };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut A_constructed_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut A_constructed_0)).prefix_dec();
        self.data_ = 0_();
    }
    pub unsafe fn operator_eq(&self, mut i: ()) -> () {
        return ((self.data_) == (i));
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const A) -> *mut A {
        self.data_ = (*_a0).data_;
        return &mut (*(self as *mut A));
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        unsafe { A::copy_from(self as *const A) }
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
    let mut buffer: () = unimplemented!();
    let mut it: () =
        std_raw_storage_iterator_A_ptr__A_::new({ ((&mut buffer as *mut ()) as *mut A) });
    unimplemented!();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (3_())) {
        unimplemented!();
        let mut ap: *mut A = ((&mut buffer as *mut ()) as *mut A).offset((i) as isize);
        unimplemented!();
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const A_constructed_0);
}
