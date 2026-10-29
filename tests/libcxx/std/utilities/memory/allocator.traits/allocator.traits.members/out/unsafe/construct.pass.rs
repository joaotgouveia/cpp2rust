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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_A0_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_A1_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_A2_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A_Holder_NS_Incomplete__ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B_A0_ {
    pub count_: *mut (),
}
impl B_A0_ {
    pub unsafe fn new(count: *mut ()) -> Self {
        let mut this = Self { count_: count };
        this
    }
    pub unsafe fn construct_2_struct_A0_int_ptr(&mut self, mut p: *mut A0, args: *mut *mut ()) {
        (*self.count_).prefix_inc();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B_A1_ {
    pub count_: *mut (),
}
impl B_A1_ {
    pub unsafe fn new(count: *mut ()) -> Self {
        let mut this = Self { count_: count };
        this
    }
    pub unsafe fn construct_2_struct_A1_int_ptr_char(
        &mut self,
        mut p: *mut A1,
        args_1: *mut *mut (),
        args_2: *mut (),
    ) {
        (*self.count_).prefix_inc();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B_A2_ {
    pub count_: *mut (),
}
impl B_A2_ {
    pub unsafe fn new(count: *mut ()) -> Self {
        let mut this = Self { count_: count };
        this
    }
    pub unsafe fn construct_2_struct_A2_int_ptr_char_int(
        &mut self,
        mut p: *mut A2,
        args_1: *mut *mut (),
        args_2: *mut (),
        args_3: *mut (),
    ) {
        (*self.count_).prefix_inc();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A0 {}
impl A0 {
    pub unsafe fn new(mut count: *mut ()) -> Self {
        let mut this = Self {};
        (*count).prefix_inc();
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A1 {}
impl A1 {
    pub unsafe fn new(mut count: *mut (), mut c: ()) -> Self {
        let mut this = Self {};
        unimplemented!();
        (*count).prefix_inc();
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A2 {}
impl A2 {
    pub unsafe fn new(mut count: *mut (), mut c: (), mut i: ()) -> Self {
        let mut this = Self {};
        unimplemented!();
        unimplemented!();
        (*count).prefix_inc();
        this
    }
}
pub unsafe fn test_4() -> () {
    {
        let mut A0_count: () = 0_();
        let mut a: A_A0_ = <A_A0_>::default();
        let mut alloc: () = unimplemented!();
        let mut a0: *mut A0 = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut A1_count: () = 0_();
        let mut a: A_A1_ = <A_A1_>::default();
        let mut alloc: () = unimplemented!();
        let mut a1: *mut A1 = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut A2_count: () = 0_();
        let mut a: A_A2_ = <A_A2_>::default();
        let mut alloc: () = unimplemented!();
        let mut a2: *mut A2 = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: A_Holder_NS_Incomplete__ptr_ = <A_Holder_NS_Incomplete__ptr_>::default();
        let mut alloc: () = unimplemented!();
        let mut vt: *mut *mut Holder_NS_Incomplete_ = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut A0_count: () = 0_();
        let mut b_construct: () = 0_();
        let mut b: B_A0_ = B_A0_::new({ &mut b_construct });
        let mut alloc: () = unimplemented!();
        let mut a0: *mut A0 = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut A1_count: () = 0_();
        let mut b_construct: () = 0_();
        let mut b: B_A1_ = B_A1_::new({ &mut b_construct });
        let mut alloc: () = unimplemented!();
        let mut a1: *mut A1 = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut A2_count: () = 0_();
        let mut b_construct: () = 0_();
        let mut b: B_A2_ = B_A2_::new({ &mut b_construct });
        let mut alloc: () = unimplemented!();
        let mut a2: *mut A2 = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
