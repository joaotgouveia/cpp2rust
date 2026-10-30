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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct A_int_ {
    pub storage: (),
}
impl A_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            storage: unimplemented!(),
        };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        unimplemented!();
        return (&mut self.storage as *mut ());
    }
}
impl Default for A_int_ {
    fn default() -> Self {
        unsafe { A_int_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct A_Holder_NS_Incomplete__ptr_ {
    pub storage: *mut Holder_NS_Incomplete_,
}
impl A_Holder_NS_Incomplete__ptr_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            storage: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut *mut Holder_NS_Incomplete_ {
        unimplemented!();
        return (&mut self.storage as *mut *mut Holder_NS_Incomplete_);
    }
}
impl Default for A_Holder_NS_Incomplete__ptr_ {
    fn default() -> Self {
        unsafe { A_Holder_NS_Incomplete__ptr_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B_int_ {
    pub storage: (),
}
impl B_int_ {
    pub unsafe fn allocate(&mut self, mut n: (), mut p: ()) -> *mut () {
        unimplemented!();
        unimplemented!();
        return (&mut self.storage as *mut ());
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B_Holder_NS_Incomplete__ptr_ {
    pub storage: *mut Holder_NS_Incomplete_,
}
impl B_Holder_NS_Incomplete__ptr_ {
    pub unsafe fn allocate(&mut self, mut n: (), mut p: ()) -> *mut *mut Holder_NS_Incomplete_ {
        unimplemented!();
        unimplemented!();
        return (&mut self.storage as *mut *mut Holder_NS_Incomplete_);
    }
}
pub unsafe fn test_4() -> () {
    {
        let mut a: A_int_ = A_int_::new();
        unimplemented!();
    }
    {
        let mut a: A_Holder_NS_Incomplete__ptr_ = A_Holder_NS_Incomplete__ptr_::new();
        unimplemented!();
    }
    {
        let mut b: B_int_ = <B_int_>::default();
        unimplemented!();
    }
    {
        let mut b: B_Holder_NS_Incomplete__ptr_ = <B_Holder_NS_Incomplete__ptr_>::default();
        unimplemented!();
    }
    return true;
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
    (unsafe { test_4() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct Holder_NS_Incomplete_;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
