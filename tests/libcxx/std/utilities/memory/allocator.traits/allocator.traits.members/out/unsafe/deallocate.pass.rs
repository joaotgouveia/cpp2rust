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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, mut_ptr)]
pub struct A_int_ {
    pub called_: *mut i32,
    pub storage: i32,
}
impl A_int_ {
    pub unsafe fn new(called: *mut i32) -> Self {
        let mut this = Self {
            called_: called,
            storage: 0_i32,
        };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut n: usize) {
        assert!(((p) == (&mut self.storage as *mut i32)));
        assert!(((n) == (10_usize)));
        (*self.called_).prefix_inc();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, mut_ptr)]
pub struct A_Holder_NS_Incomplete__ptr_ {
    pub called_: *mut i32,
    pub storage: *mut Holder_NS_Incomplete_,
}
impl A_Holder_NS_Incomplete__ptr_ {
    pub unsafe fn new(called: *mut i32) -> Self {
        let mut this = Self {
            called_: called,
            storage: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut Holder_NS_Incomplete_, mut n: usize) {
        assert!(((p) == (&mut self.storage as *mut *mut Holder_NS_Incomplete_)));
        assert!(((n) == (10_usize)));
        (*self.called_).prefix_inc();
    }
}
pub unsafe fn test_4() -> bool {
    {
        let mut called: i32 = 0;
        let mut a: A_int_ = A_int_::new({ &mut called });
        unimplemented!();
        assert!(((called) == (1)));
    }
    {
        let mut called: i32 = 0;
        let mut a: A_Holder_NS_Incomplete__ptr_ =
            A_Holder_NS_Incomplete__ptr_::new({ &mut called });
        unimplemented!();
        assert!(((called) == (1)));
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
    (unsafe { test_4() });
    const _: () = assert!(true, "test()");;
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct Holder_NS_Incomplete_;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
