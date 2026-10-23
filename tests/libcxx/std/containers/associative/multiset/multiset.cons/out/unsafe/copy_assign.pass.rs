extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(a: *const test_less_int_, b: *const test_less_int_) -> () {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_less_int_ {
    pub data_: (),
}
impl test_less_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { data_: 0_() };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, a: *const (), b: *const ()) -> () {
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
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_4)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3)) = 0_();
    }
}
pub static mut outstanding_bytes_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct tracking_allocator_int_ {
    allocs_: *mut (),
}
impl tracking_allocator_int_ {
    pub unsafe fn new(allocs: *mut ()) -> Self {
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
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 2] = [4_(), 5_()];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 3] = [4_(), 5_(), 6_()];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 3] = [4_(), 5_(), 6_()];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut orig: () = unimplemented!();
            let mut i: () = 0_();
            'loop_: while ((i) != (50_())) {
                unimplemented!();
                i.prefix_inc();
            }
            let mut copy: () = unimplemented!();
            unimplemented!();
            let mut i: () = 0_();
            'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut v: () = (*unimplemented!());
                unimplemented!();
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
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 2] = [4_(), 5_()];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 3] = [4_(), 5_(), 6_()];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 3] = [4_(), 5_(), 6_()];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut orig: () = unimplemented!();
            let mut i: () = 0_();
            'loop_: while ((i) != (50_())) {
                unimplemented!();
                i.prefix_inc();
            }
            let mut copy: () = unimplemented!();
            unimplemented!();
            let mut i: () = 0_();
            'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut v: () = (*unimplemented!());
                unimplemented!();
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
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
        {
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [1_(), 2_(), 2_()];
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 2] = [4_(), 5_()];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 3] = [4_(), 5_(), 6_()];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [1_(), 2_(), 2_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 3] = [4_(), 5_(), 6_()];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
        {
            let mut orig: () = unimplemented!();
            let mut i: () = 0_();
            'loop_: while ((i) != (50_())) {
                unimplemented!();
                i.prefix_inc();
            }
            let mut copy: () = unimplemented!();
            unimplemented!();
            let mut i: () = 0_();
            'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut v: () = (*unimplemented!());
                unimplemented!();
            }
        }
        (unsafe { AssertEmpty_8::operator_call(&mut check_alloc_invariant) });
    }
}
pub unsafe fn test_9() -> () {
    (unsafe { test_alloc_5(None, None, None) });
    (unsafe { test_alloc_6(None, None, None) });
    {
        let mut lhs_allocs: () = unimplemented!();
        let mut rhs_allocs: () = unimplemented!();
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
        let mut arr: [(); 3] = [1_(), 2_(), 2_()];
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct AssertEmpty_8 {
    lhs_allocs_: *mut (),
    rhs_allocs_: *mut (),
}
impl AssertEmpty_8 {
    pub unsafe fn new(lhs_allocs: *mut (), rhs_allocs: *mut ()) -> Self {
        let mut this = Self {
            lhs_allocs_: (lhs_allocs),
            rhs_allocs_: (rhs_allocs),
        };
        this
    }
    pub unsafe fn operator_call(&mut self) {
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_9() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
