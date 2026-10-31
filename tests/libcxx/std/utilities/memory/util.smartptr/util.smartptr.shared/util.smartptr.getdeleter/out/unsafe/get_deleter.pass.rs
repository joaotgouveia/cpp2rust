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
pub struct test_deleter_base {}
pub static mut count_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut dealloc_count_5: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct test_deleter_A_ {
    state_: i32,
}
impl test_deleter_A_ {
    pub unsafe fn new_1(mut s: i32) -> Self {
        let mut this = Self { state_: s };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(d: *const test_deleter_A_) -> Self {
        let mut this = Self {
            state_: (*d).state_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        assert!(((self.state_) >= (0)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)).prefix_dec();
        self.state_ = -1_i32;
    }
    pub unsafe fn state(&self) -> i32 {
        return self.state_;
    }
    pub unsafe fn operator_call(&mut self, mut p: *mut A) {
        assert!(((self.state_) >= (0)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)).prefix_inc();
        {
            let __p = p;
            if !__p.is_null() {
                A::destructor(&mut *__p);
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
    }
}
impl Clone for test_deleter_A_ {
    fn clone(&self) -> Self {
        unsafe { test_deleter_A_::copy_from(self as *const test_deleter_A_) }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct A {}
impl A {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const A) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)).prefix_dec();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const A) -> *mut A {
        return &mut (*(self as *mut A));
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        unsafe { A::copy_from(self as *const A) }
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new() }
    }
}
pub static mut count_6: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });

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
    {
        {
            let mut ptr: *mut A = (Box::leak(Box::new(A::new())) as *mut A);
            let mut p: () = unimplemented!();
            let mut d: *mut test_deleter_A_ = unimplemented!();
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)) == (1)));
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) == (0)));
            assert!(!(d).is_null());
            assert!(((unsafe { test_deleter_A_::state(&(*(d).cast_const()),) }) == (3)));
        }
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)) == (0)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)) == (0)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) == (1)));
    }
    (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) = 0;
    {
        {
            let mut p: () = unimplemented!();
            let mut d: *mut test_deleter_A_ = unimplemented!();
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)) == (1)));
            assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) == (0)));
            assert!(!(d).is_null());
            assert!(((unsafe { test_deleter_A_::state(&(*(d).cast_const()),) }) == (3)));
        }
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_6)) == (0)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_4)) == (0)));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) == (1)));
    }
    (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) = 0;
    {
        let mut p: () = unimplemented!();
        let mut d: *mut () = unimplemented!();
        assert!((d).is_null());
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const count_4);
    std::cell::LazyCell::force(&*&raw const dealloc_count_5);
    std::cell::LazyCell::force(&*&raw const count_6);
}
