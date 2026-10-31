extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_4)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3)) = 0_usize;
    }
}
pub static mut outstanding_bytes_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_3: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_4: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
pub unsafe fn g_5(mut x: i32, mut y: i32) -> bool {
    return ((x) == (y));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct PredLWG526 {
    pub i_: i32,
}
impl PredLWG526 {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.i_ = -32767_i32;
    }
    pub unsafe fn operator_call(&self, lhs: *const PredLWG526, rhs: *const PredLWG526) -> bool {
        return (((*lhs).i_) == ((*rhs).i_));
    }
    pub unsafe fn operator_eq(&self, mut i: i32) -> bool {
        return ((i) == (self.i_));
    }
}
impl Callable2<*const PredLWG526, *const PredLWG526, bool> for PredLWG526 {
    fn call(&self, a1: *const PredLWG526, a2: *const PredLWG526) -> bool {
        unsafe { PredLWG526::operator_call(self, a1, a2) }
    }
}
pub unsafe fn test_6() -> bool {
    {
        let mut a1: [i32; 9] = [2, 1, 1, 4, 4, 4, 4, 3, 3];
        let mut a2: [i32; 4] = [2, 1, 4, 3];
        let mut c: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(((unimplemented!()) == (5_())));
        assert!(unimplemented!());
    }
    {
        let mut a1: [i32; 9] = [1, 1, 1, 2, 3, 5, 5, 2, 11];
        let mut a2: [i32; 6] = [1, 2, 3, 5, 2, 11];
        let mut c: () = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!(((unimplemented!()) == (6_())));
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (unimplemented!() as usize)) {
            assert!((unsafe { PredLWG526::operator_eq(&(*unimplemented!()), a2[(i)],) }));
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let mut a1: [i32; 9] = [2, 1, 1, 4, 4, 4, 4, 3, 3];
        let mut a2: [i32; 4] = [2, 1, 4, 3];
        let mut c: () = unimplemented!();
        assert!(((unimplemented!()) == (5_())));
        assert!((unimplemented!()));
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
    assert!((unsafe { test_6() }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
