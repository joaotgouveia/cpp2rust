extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(a: *const test_less_int_, b: *const test_less_int_) -> bool {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_less_int_ {
    pub data_: i32,
}
impl test_less_int_ {
    pub unsafe fn new_1(mut data: i32) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, a: *const i32, b: *const i32) -> bool {
        return ((*a) < (*b));
    }
}
impl std::cmp::PartialEq for test_less_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const test_less_int_,
                other as *const test_less_int_,
            )
        }
    }
}
impl std::cmp::Eq for test_less_int_ {}
impl Callable2<*const i32, *const i32, bool> for test_less_int_ {
    fn call(&self, a1: *const i32, a2: *const i32) -> bool {
        unsafe { test_less_int_::operator_call(self, a1, a2) }
    }
}
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
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (i32, f64),
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__double__ {}
pub unsafe fn test_5() -> bool {
    {
        let mut ar: [(i32, f64); 9] = [
            (1, (1 as f64)),
            (1, 1.5_f64),
            (1, (2 as f64)),
            (2, (1 as f64)),
            (2, 1.5_f64),
            (2, (2 as f64)),
            (3, (1 as f64)),
            (3, 1.5_f64),
            (3, (2 as f64)),
        ];
        let mut m: UnsafeMap<i32, (i32, f64)> = unimplemented!();
        assert!(
            (unsafe {
                let mut _a: test_less_int_ = unimplemented!();
                let mut _b: test_less_int_ = test_less_int_::new_1({ 5 });
                operator_eq_0(&mut _a, &mut _b)
            })
        );
        assert!(((m.len()) == (3_usize)));
        assert!(((unimplemented!()) == (3_isize)));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    {
        let mut ar: [(i32, f64); 9] = [
            (1, (1 as f64)),
            (1, 1.5_f64),
            (1, (2 as f64)),
            (2, (1 as f64)),
            (2, 1.5_f64),
            (2, (2 as f64)),
            (3, (1 as f64)),
            (3, 1.5_f64),
            (3, (2 as f64)),
        ];
        let mut m: UnsafeMap<i32, (i32, f64)> = unimplemented!();
        assert!(
            (unsafe {
                let mut _a: test_less_int_ = unimplemented!();
                let mut _b: test_less_int_ = test_less_int_::new_1({ 5 });
                operator_eq_0(&mut _a, &mut _b)
            })
        );
        assert!(((m.len()) == (3_usize)));
        assert!(((unimplemented!()) == (3_isize)));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
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
    (unsafe { test_5() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
