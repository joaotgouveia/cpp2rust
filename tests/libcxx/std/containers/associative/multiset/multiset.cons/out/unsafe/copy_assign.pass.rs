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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_less_int_ {
    pub data_: i32,
}
impl test_less_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { data_: 0 };
        this
    }
    pub unsafe fn new_2(mut data: i32) -> Self {
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
impl Default for test_less_int_ {
    fn default() -> Self {
        unsafe { test_less_int_::new_1() }
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Vec<*mut ::libc::c_void > , new, mut_ptr)]
pub struct tracking_allocator_int_ {
    allocs_: *mut Vec<*mut ::libc::c_void>,
}
impl tracking_allocator_int_ {
    pub unsafe fn new(allocs: *mut Vec<*mut ::libc::c_void>) -> Self {
        let mut this = Self { allocs_: (allocs) };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoOp {}
impl NoOp {
    pub unsafe fn operator_call(&mut self) {}
}
pub unsafe fn test_alloc_5(
    lhs_alloc: Option<*const ()>,
    rhs_alloc: Option<*const ()>,
    mut check_alloc_invariant: Option<NoOp>,
) {
    let mut __lhs_alloc_default: Option<()> = None;
    let mut lhs_alloc: *const () = lhs_alloc
        .unwrap_or_else(|| unsafe { __lhs_alloc_default.insert(unimplemented!()) as *const () });
    let mut __rhs_alloc_default: Option<()> = None;
    let mut rhs_alloc: *const () = rhs_alloc
        .unwrap_or_else(|| unsafe { __rhs_alloc_default.insert(unimplemented!()) as *const () });
    let mut check_alloc_invariant: NoOp =
        check_alloc_invariant.unwrap_or_else(|| unsafe { <NoOp>::default() });
    {
        {
            let mut arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
        }
    }
    {
        {
            let mut arr: [i32; 3] = [1, 2, 2];
            let mut orig: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        {
            let mut orig: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
        }
    }
    {
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut rhs_arr: [i32; 2] = [4, 5];
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut rhs_arr: [i32; 3] = [4, 5, 6];
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut rhs_arr: [i32; 3] = [4, 5, 6];
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut orig: () = unimplemented!();
            let mut i: i32 = 0;
            'loop_: while ((i) != (50)) {
                unimplemented!();
                i.prefix_inc();
            }
            let mut copy: () = unimplemented!();
            unimplemented!();
            let mut i: i32 = 0;
            'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut v: i32 = (*v.iter_deref());
                assert!(((v) == (i.postfix_inc())));
            }
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
}
pub unsafe fn test_alloc_6(
    lhs_alloc: Option<*const min_allocator_int_>,
    rhs_alloc: Option<*const min_allocator_int_>,
    mut check_alloc_invariant: Option<NoOp>,
) {
    let mut __lhs_alloc_default: Option<min_allocator_int_> = None;
    let mut lhs_alloc: *const min_allocator_int_ = lhs_alloc.unwrap_or_else(|| unsafe {
        __lhs_alloc_default.insert(<min_allocator_int_>::default()) as *const min_allocator_int_
    });
    let mut __rhs_alloc_default: Option<min_allocator_int_> = None;
    let mut rhs_alloc: *const min_allocator_int_ = rhs_alloc.unwrap_or_else(|| unsafe {
        __rhs_alloc_default.insert(<min_allocator_int_>::default()) as *const min_allocator_int_
    });
    let mut check_alloc_invariant: NoOp =
        check_alloc_invariant.unwrap_or_else(|| unsafe { <NoOp>::default() });
    {
        {
            let mut arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
        }
    }
    {
        {
            let mut arr: [i32; 3] = [1, 2, 2];
            let mut orig: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        {
            let mut orig: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
        }
    }
    {
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut rhs_arr: [i32; 2] = [4, 5];
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut rhs_arr: [i32; 3] = [4, 5, 6];
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut rhs_arr: [i32; 3] = [4, 5, 6];
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut orig: () = unimplemented!();
            let mut i: i32 = 0;
            'loop_: while ((i) != (50)) {
                unimplemented!();
                i.prefix_inc();
            }
            let mut copy: () = unimplemented!();
            unimplemented!();
            let mut i: i32 = 0;
            'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut v: i32 = (*v.iter_deref());
                assert!(((v) == (i.postfix_inc())));
            }
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
}
pub unsafe fn test_alloc_7(
    lhs_alloc: *const tracking_allocator_int_,
    rhs_alloc: *const tracking_allocator_int_,
    mut check_alloc_invariant: AssertEmpty_8,
) {
    {
        {
            let mut arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
        {
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
        }
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
        }
    }
    {
        {
            let mut arr: [i32; 3] = [1, 2, 2];
            let mut orig: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        {
            let mut orig: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (0_())));
            assert!(unimplemented!() == unimplemented!());
        }
    }
    {
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut rhs_arr: [i32; 2] = [4, 5];
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut rhs_arr: [i32; 3] = [4, 5, 6];
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [i32; 3] = [1, 2, 2];
            let orig: () = unimplemented!();
            let mut rhs_arr: [i32; 3] = [4, 5, 6];
            let mut copy: () = unimplemented!();
            unimplemented!();
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
            assert!(((unimplemented!()) == (3_())));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()) == (1)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()) == (2)));
            assert!(((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()) == (2)));
            assert!(libcc2rs::next_unsafe(unimplemented!(), 3_isize) == unimplemented!());
        }
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
        {
            let mut orig: () = unimplemented!();
            let mut i: i32 = 0;
            'loop_: while ((i) != (50)) {
                unimplemented!();
                i.prefix_inc();
            }
            let mut copy: () = unimplemented!();
            unimplemented!();
            let mut i: i32 = 0;
            'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut v: i32 = (*v.iter_deref());
                assert!(((v) == (i.postfix_inc())));
            }
        }
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
    }
}
pub unsafe fn test_9() -> bool {
    (unsafe { test_alloc_5(None, None, None) });
    (unsafe { test_alloc_6(None, None, None) });
    {
        let mut lhs_allocs: Vec<*mut ::libc::c_void> = Vec::new();
        let mut rhs_allocs: Vec<*mut ::libc::c_void> = Vec::new();
        (unsafe {
            let mut _lhs_alloc: tracking_allocator_int_ =
                tracking_allocator_int_::new({ &mut lhs_allocs });
            let mut _rhs_alloc: tracking_allocator_int_ =
                tracking_allocator_int_::new({ &mut rhs_allocs });
            let _check_alloc_invariant: AssertEmpty_8 =
                AssertEmpty_8::new({ &mut lhs_allocs }, { &mut rhs_allocs });
            test_alloc_7(&mut _lhs_alloc, &mut _rhs_alloc, _check_alloc_invariant)
        });
    }
    {
        let mut arr: [i32; 3] = [1, 2, 2];
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!(
            (unsafe {
                let mut _a: test_less_int_ = unimplemented!();
                let mut _b: test_less_int_ = test_less_int_::new_2({ 3 });
                operator_eq_0(&mut _a, &mut _b)
            })
        );
        assert!(((unimplemented!()) == (3_())));
        assert!(
            (unsafe {
                let mut _a: test_less_int_ = unimplemented!();
                let mut _b: test_less_int_ = test_less_int_::new_2({ 3 });
                operator_eq_0(&mut _a, &mut _b)
            })
        );
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct AssertEmpty_8 {
    lhs_allocs_: *mut Vec<*mut ::libc::c_void>,
    rhs_allocs_: *mut Vec<*mut ::libc::c_void>,
}
impl AssertEmpty_8 {
    pub unsafe fn new(
        lhs_allocs: *mut Vec<*mut ::libc::c_void>,
        rhs_allocs: *mut Vec<*mut ::libc::c_void>,
    ) -> Self {
        let mut this = Self {
            lhs_allocs_: (lhs_allocs),
            rhs_allocs_: (rhs_allocs),
        };
        this
    }
    pub unsafe fn operator_call(&mut self) {
        assert!((*(self.lhs_allocs_).cast_const()).is_empty());
        assert!((*(self.rhs_allocs_).cast_const()).is_empty());
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_9() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
