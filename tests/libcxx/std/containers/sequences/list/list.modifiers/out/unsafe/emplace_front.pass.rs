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
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_usize;
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_A__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut A,
}
impl min_pointer_A__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_A_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct A {
    i_: i32,
    d_: f64,
}
impl A {
    pub unsafe fn new(mut i: i32, mut d: f64) -> Self {
        let mut this = Self { i_: i, d_: d };
        this
    }
    pub unsafe fn geti(&self) -> i32 {
        return self.i_;
    }
    pub unsafe fn getd(&self) -> f64 {
        return self.d_;
    }
}
pub unsafe fn test_4() -> bool {
    {
        let mut c: () = unimplemented!();
        let r1: *mut A = unimplemented!();
        assert!(((unimplemented!()) == (1_())));
        assert!(((r1) == (unimplemented!())));
        assert!(((unsafe { A::geti(&(*unimplemented!()),) }) == (2)));
        assert!(((unsafe { A::getd(&(*unimplemented!()),) }) == (3.5_f64)));
        let r2: *mut A = unimplemented!();
        assert!(((unimplemented!()) == (2_())));
        assert!(((r2) == (unimplemented!())));
        assert!(((unsafe { A::geti(&(*unimplemented!()),) }) == (3)));
        assert!(((unsafe { A::getd(&(*unimplemented!()),) }) == (4.5_f64)));
        assert!(((unsafe { A::geti(&(*unimplemented!()),) }) == (2)));
        assert!(((unsafe { A::getd(&(*unimplemented!()),) }) == (3.5_f64)));
    }
    {
        let mut c: () = unimplemented!();
        let r1: *mut A = unimplemented!();
        assert!(((unimplemented!()) == (1_())));
        assert!(((r1) == (unimplemented!())));
        assert!(((unsafe { A::geti(&(*unimplemented!()),) }) == (2)));
        assert!(((unsafe { A::getd(&(*unimplemented!()),) }) == (3.5_f64)));
        let r2: *mut A = unimplemented!();
        assert!(((unimplemented!()) == (2_())));
        assert!(((r2) == (unimplemented!())));
        assert!(((unsafe { A::geti(&(*unimplemented!()),) }) == (3)));
        assert!(((unsafe { A::getd(&(*unimplemented!()),) }) == (4.5_f64)));
        assert!(((unsafe { A::geti(&(*unimplemented!()),) }) == (2)));
        assert!(((unsafe { A::getd(&(*unimplemented!()),) }) == (3.5_f64)));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    assert!((unsafe { test_4() }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
