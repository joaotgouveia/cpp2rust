extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut pointer_count_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct Pointer_int_ {
    pub ptr_: *mut (),
}
impl Pointer_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut pointer_count_0)).prefix_inc();
        this
    }
    pub unsafe fn new_2(mut ptr: *mut ()) -> Self {
        let mut this = Self { ptr_: ptr };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut pointer_count_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(other: *const Pointer_int_) -> Self {
        let mut this = Self {
            ptr_: (*other).ptr_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut pointer_count_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, other: *const Pointer_int_) -> *mut Pointer_int_ {
        self.ptr_ = (*other).ptr_;
        return &mut (*(self as *mut Pointer_int_));
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut pointer_count_0)).prefix_dec();
    }
    pub unsafe fn to_pmut(&mut self) -> *mut () {
        return self.ptr_;
    }
}
impl Clone for Pointer_int_ {
    fn clone(&self) -> Self {
        unsafe { Pointer_int_::copy_from(self as *const Pointer_int_) }
    }
}
impl Default for Pointer_int_ {
    fn default() -> Self {
        unsafe { Pointer_int_::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Deleter_int_ {}
impl Deleter_int_ {
    pub unsafe fn operator_call(&self, mut ptr: Pointer_int_) {
        {
            let __p = ptr.ptr_;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
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
        let mut ptr: () = unimplemented!();
        let mut v: () = unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const pointer_count_0);
}
