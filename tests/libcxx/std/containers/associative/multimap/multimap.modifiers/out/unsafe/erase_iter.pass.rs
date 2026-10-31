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
impl TemplateConstructor {
    pub unsafe fn new(_a0: *const i32) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, ptr)]
pub struct TemplateConstructor {}
impl std::cmp::Ord for TemplateConstructor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_4(
                self as *const TemplateConstructor,
                other as *const TemplateConstructor,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(
                other as *const TemplateConstructor,
                self as *const TemplateConstructor,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for TemplateConstructor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TemplateConstructor {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_4(
                self as *const TemplateConstructor,
                other as *const TemplateConstructor,
            )) && !(operator_lt_4(
                other as *const TemplateConstructor,
                self as *const TemplateConstructor,
            ))
        }
    }
}
impl std::cmp::Eq for TemplateConstructor {}
pub unsafe fn operator_lt_4(
    _a0: *const TemplateConstructor,
    _a1: *const TemplateConstructor,
) -> bool {
    return false;
}
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
        let mut m: () = unimplemented!();
        assert!(((unimplemented!()) == (9_())));
        let mut i: () = unimplemented!();
        assert!(((unimplemented!()) == (8_())));
        assert!(i == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        assert!((((*unimplemented!().iter_deref()).0) == (1)));
        assert!((((*unimplemented!().iter_deref()).1) == (1_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (1)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).0) == (1)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).0) == (2)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()).1) == (1_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 6_isize).iter_deref()).0) == (3)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 6_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 7_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 7_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!();
        assert!(((unimplemented!()) == (7_())));
        assert!(i == unimplemented!());
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (1)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (1)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).0) == (2)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).1) == (1_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()).0) == (3)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 6_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 6_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!();
        assert!(((unimplemented!()) == (6_())));
        assert!(i == libcc2rs::prev_unsafe(unimplemented!(), 1_isize));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (1)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (1)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).0) == (2)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).1) == (1_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!();
        assert!(((unimplemented!()) == (5_())));
        assert!(i == libcc2rs::next_unsafe(unimplemented!(), 1_isize));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (1)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).0) == (2)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).1) == (1_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!();
        assert!(((unimplemented!()) == (4_())));
        assert!(i == libcc2rs::next_unsafe(unimplemented!(), 2_isize));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (1)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).1) == (1_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!();
        assert!(((unimplemented!()) == (3_())));
        assert!(i == libcc2rs::next_unsafe(unimplemented!(), 2_isize));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (1)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!();
        assert!(((unimplemented!()) == (2_())));
        assert!(i == libcc2rs::next_unsafe(unimplemented!(), 0_isize));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!();
        assert!(((unimplemented!()) == (1_())));
        assert!(i == unimplemented!());
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        i = unimplemented!();
        assert!(((unimplemented!()) == (0_())));
        assert!(i == unimplemented!());
        assert!(i == unimplemented!());
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
        let mut m: () = unimplemented!();
        assert!(((unimplemented!()) == (9_())));
        let mut i: () = unimplemented!();
        assert!(((unimplemented!()) == (8_())));
        assert!(i == libcc2rs::next_unsafe(unimplemented!(), 3_isize));
        assert!((((*unimplemented!().iter_deref()).0) == (1)));
        assert!((((*unimplemented!().iter_deref()).1) == (1_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (1)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).0) == (1)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).0) == (2)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()).1) == (1_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 6_isize).iter_deref()).0) == (3)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 6_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 7_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 7_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!().clone();
        assert!(((unimplemented!()) == (7_())));
        assert!(i == unimplemented!());
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (1)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (1)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).0) == (2)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).1) == (1_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()).0) == (3)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 6_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 6_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!().clone();
        assert!(((unimplemented!()) == (6_())));
        assert!(i == libcc2rs::prev_unsafe(unimplemented!(), 1_isize));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (1)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (1)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).0) == (2)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).1) == (1_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 5_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!().clone();
        assert!(((unimplemented!()) == (5_())));
        assert!(i == libcc2rs::next_unsafe(unimplemented!(), 1_isize));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (1)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).0) == (2)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).1) == (2_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).1) == (1_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 4_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!().clone();
        assert!(((unimplemented!()) == (4_())));
        assert!(i == libcc2rs::next_unsafe(unimplemented!(), 2_isize));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (1)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).1) == (1_f64)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 3_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!().clone();
        assert!(((unimplemented!()) == (3_())));
        assert!(i == libcc2rs::next_unsafe(unimplemented!(), 2_isize));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (1)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 2_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!().clone();
        assert!(((unimplemented!()) == (2_())));
        assert!(i == libcc2rs::next_unsafe(unimplemented!(), 0_isize));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).0) == (3)));
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 1_isize).iter_deref()).1) == (2_f64)));
        i = unimplemented!().clone();
        assert!(((unimplemented!()) == (1_())));
        assert!(i == unimplemented!());
        assert!((((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).0) == (2)));
        assert!(
            (((*libcc2rs::next_unsafe(unimplemented!(), 0_isize).iter_deref()).1) == (1.5_f64))
        );
        i = unimplemented!().clone();
        assert!(((unimplemented!()) == (0_())));
        assert!(i == unimplemented!());
        assert!(i == unimplemented!());
    }
    {
        let mut c: () = unimplemented!();
        let mut a: TemplateConstructor = {
            let mut __tmp_0: i32 = 0;
            TemplateConstructor::new({ &mut __tmp_0 })
        };
        let mut it: () = unimplemented!();
        if it != unimplemented!() {
            unimplemented!();
        }
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
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
