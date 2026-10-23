extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: (),
    pub throw_after: (),
    pub count: (),
    pub alloc_count: (),
    pub allocated_size: (),
    pub construct_count: (),
    pub destroy_count: (),
    pub copied: (),
    pub moved: (),
    pub converted: (),
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        unimplemented!();
        self.count = 0_();
        self.time_to_throw = 0_();
        self.alloc_count = 0_();
        self.allocated_size = 0_();
        self.construct_count = 0_();
        self.destroy_count = 0_();
        self.throw_after = 2147483647_();
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0_();
        self.moved = 0_();
        self.converted = 0_();
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
pub static mut destructed_value_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_4(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_5(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_4(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0_(),
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: (), mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: (), mut id: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: (),
        mut id: (),
        mut stats: *mut test_allocator_statistics,
    ) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: stats,
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_void_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> () {
        return self.data_;
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const test_allocator_void_,
    ) -> *mut test_allocator_void_ {
        self.data_ = (*_a0).data_;
        self.id_ = (*_a0).id_;
        self.stats_ = (*_a0).stats_;
        return &mut (*(self as *mut test_allocator_void_));
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_4(
                self as *const test_allocator_void_,
                other as *const test_allocator_void_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_void_::copy_from(self as *const test_allocator_void_) }
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        unsafe {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_int_ {}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_int_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ {
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
    }
}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        unsafe {
            test_allocator_int_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Tag_X {}
impl Tag_X {
    pub unsafe fn new_1(mut _a0: Ctor_Tag) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: Ctor_Tag, _a1: *const Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_3(mut _a0: Ctor_Tag, _a1: *mut Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_9)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_6)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8)) = 0_();
    }
}
pub static mut outstanding_bytes_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_9: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_10(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_11(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_void__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_10(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl std::cmp::PartialEq for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
pub unsafe fn test0_12(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_13(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_14(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_15(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_16(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_17(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_18(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_19(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_20(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_21(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_22() {
    (unsafe { test0_12(unimplemented!(), 1_(), unimplemented!(), 0_()) });
    (unsafe { test0_12(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_12(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_12(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_12(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_12(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_12(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_12(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_12(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_12(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_12(unimplemented!(), 3_(), unimplemented!(), 1_()) });
    (unsafe { test0_12(unimplemented!(), 4_(), unimplemented!(), 0_()) });
    (unsafe { test0_12(unimplemented!(), 1_(), unimplemented!(), 3_()) });
    (unsafe { test0_12(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_12(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_12(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_12(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_12(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_12(unimplemented!(), 2_(), unimplemented!(), 2_()) });
    (unsafe { test0_12(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_13(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
    (unsafe { test0_13(unimplemented!(), unimplemented!(), unimplemented!(), 2_()) });
    (unsafe { test0_13(unimplemented!(), unimplemented!(), unimplemented!(), 1_()) });
    (unsafe { test0_13(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
}
pub unsafe fn test_23() {
    (unsafe { test0_14(unimplemented!(), 1_(), unimplemented!(), 0_()) });
    (unsafe { test0_14(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_14(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_14(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_14(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_14(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_14(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_14(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_14(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_14(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_14(unimplemented!(), 3_(), unimplemented!(), 1_()) });
    (unsafe { test0_14(unimplemented!(), 4_(), unimplemented!(), 0_()) });
    (unsafe { test0_14(unimplemented!(), 1_(), unimplemented!(), 3_()) });
    (unsafe { test0_14(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_14(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_14(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_14(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_14(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_14(unimplemented!(), 2_(), unimplemented!(), 2_()) });
    (unsafe { test0_14(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_15(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
    (unsafe { test0_15(unimplemented!(), unimplemented!(), unimplemented!(), 2_()) });
    (unsafe { test0_15(unimplemented!(), unimplemented!(), unimplemented!(), 1_()) });
    (unsafe { test0_15(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
}
pub unsafe fn test_24() {
    (unsafe { test0_16(unimplemented!(), 1_(), unimplemented!(), 0_()) });
    (unsafe { test0_16(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_16(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_16(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_16(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_16(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_16(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_16(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_16(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_16(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_16(unimplemented!(), 3_(), unimplemented!(), 1_()) });
    (unsafe { test0_16(unimplemented!(), 4_(), unimplemented!(), 0_()) });
    (unsafe { test0_16(unimplemented!(), 1_(), unimplemented!(), 3_()) });
    (unsafe { test0_16(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_16(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_16(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_16(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_16(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_16(unimplemented!(), 2_(), unimplemented!(), 2_()) });
    (unsafe { test0_16(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_17(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
    (unsafe { test0_17(unimplemented!(), unimplemented!(), unimplemented!(), 2_()) });
    (unsafe { test0_17(unimplemented!(), unimplemented!(), unimplemented!(), 1_()) });
    (unsafe { test0_17(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
}
pub unsafe fn test_25() {
    (unsafe { test0_18(unimplemented!(), 1_(), unimplemented!(), 0_()) });
    (unsafe { test0_18(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_18(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_18(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_18(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_18(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_18(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_18(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_18(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_18(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_18(unimplemented!(), 3_(), unimplemented!(), 1_()) });
    (unsafe { test0_18(unimplemented!(), 4_(), unimplemented!(), 0_()) });
    (unsafe { test0_18(unimplemented!(), 1_(), unimplemented!(), 3_()) });
    (unsafe { test0_18(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_18(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_18(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_18(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_18(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_18(unimplemented!(), 2_(), unimplemented!(), 2_()) });
    (unsafe { test0_18(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_19(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
    (unsafe { test0_19(unimplemented!(), unimplemented!(), unimplemented!(), 2_()) });
    (unsafe { test0_19(unimplemented!(), unimplemented!(), unimplemented!(), 1_()) });
    (unsafe { test0_19(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
}
pub unsafe fn test_26() {
    (unsafe { test0_20(unimplemented!(), 1_(), unimplemented!(), 0_()) });
    (unsafe { test0_20(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_20(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_20(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_20(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_20(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_20(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_20(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_20(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_20(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_20(unimplemented!(), 3_(), unimplemented!(), 1_()) });
    (unsafe { test0_20(unimplemented!(), 4_(), unimplemented!(), 0_()) });
    (unsafe { test0_20(unimplemented!(), 1_(), unimplemented!(), 3_()) });
    (unsafe { test0_20(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_20(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_20(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_20(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_20(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_20(unimplemented!(), 2_(), unimplemented!(), 2_()) });
    (unsafe { test0_20(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_21(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
    (unsafe { test0_21(unimplemented!(), unimplemented!(), unimplemented!(), 2_()) });
    (unsafe { test0_21(unimplemented!(), unimplemented!(), unimplemented!(), 1_()) });
    (unsafe { test0_21(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
}
pub unsafe fn test_27() -> () {
    (unsafe { test_22() });
    (unsafe { test_23() });
    (unsafe { test_24() });
    (unsafe { test_25() });
    (unsafe { test_26() });
    {
        let mut l: () = unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct Bool_28 {}
impl Bool_28 {
    pub unsafe fn to_(&self) -> () {
        return true;
    }
}
static mut b_30: std::cell::LazyCell<Bool_28> =
    std::cell::LazyCell::new(|| unsafe { <Bool_28>::default() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Int_29 {}
impl Int_29 {
    pub unsafe fn operator_eq(&self, mut _a0: Int_29) -> *mut Bool_28 {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut b_30));
    }
}
impl std::cmp::PartialEq for Int_29 {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            {
                let __cmp = Int_29::operator_eq(self, other.clone());
                Bool_28::to_(&*__cmp)
            }
        }
    }
}
impl std::cmp::Eq for Int_29 {}
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
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_2);
    std::cell::LazyCell::force(&*&raw const moved_value_3);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_6);
    std::cell::LazyCell::force(&*&raw const alloc_count_7);
    std::cell::LazyCell::force(&*&raw const dealloc_count_8);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_9);
}
