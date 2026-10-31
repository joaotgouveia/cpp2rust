extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_1(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_2(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_3(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_4(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_5(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_6(_a0: *const MoveOnly, _a1: *const MoveOnly) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct MoveOnly {
    data_: i32,
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0;
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0;
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_6(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_7(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_9(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_10(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_13(
    _a0: *const TrivialMoveOnly,
    _a1: *const TrivialMoveOnly,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct TrivialMoveOnly {
    data_: i32,
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(_a0: *mut TrivialMoveOnly) -> Self {
        let mut this = Self {
            data_: (*_a0).data_,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TrivialMoveOnly) -> *mut TrivialMoveOnly {
        self.data_ = (*_a0).data_;
        return &mut (*(self as *mut TrivialMoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_13(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_7(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        unsafe { TrivialMoveOnly::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: i32,
    pub throw_after: i32,
    pub count: i32,
    pub alloc_count: i32,
    pub allocated_size: i32,
    pub construct_count: i32,
    pub destroy_count: i32,
    pub copied: i32,
    pub moved: i32,
    pub converted: i32,
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        assert!(
            ((self.count) == (0)) && (!(c"clearing leaking allocator data?".as_ptr()).is_null())
        );
        self.count = 0;
        self.time_to_throw = 0;
        self.alloc_count = 0;
        self.allocated_size = 0;
        self.construct_count = 0;
        self.destroy_count = 0;
        self.throw_after = 2147483647;
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0;
        self.moved = 0;
        self.converted = 0;
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
pub static mut destructed_value_14: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_15: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_16(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_17(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_16(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: i32, mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: i32, mut id: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: i32,
        mut id: i32,
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
    }
    pub unsafe fn get_id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> i32 {
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
            operator_eq_16(
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
                data_: 0,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_18(
    x: *const test_allocator_MoveOnly_,
    y: *const test_allocator_MoveOnly_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_MoveOnly__rebind_MoveOnly_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_MoveOnly__rebind_MoveOnly_ptr_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_MoveOnly_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_MoveOnly_ {
    pub unsafe fn new_1(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_MoveOnly_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14))))
                && (!(c"copying from destroyed allocator".as_ptr()).is_null())
        );
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn move_from(a: *mut test_allocator_MoveOnly_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).moved.prefix_inc();
        }
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14))))
                && (!(c"moving from destroyed allocator".as_ptr()).is_null())
        );
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_15));
        this
    }
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u32,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut MoveOnly {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14)))
        );
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                std::process::abort();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_add(n)) as i32;
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut MoveOnly, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<MoveOnly>() as usize))) as u32);
    }
    pub unsafe fn destroy(&mut self, mut p: *mut MoveOnly) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        };
    }
    pub unsafe fn get_id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn construct_16_class_MoveOnly(&mut self, mut p: *mut MoveOnly, val: *mut MoveOnly) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        unimplemented!();
    }
}
impl std::cmp::PartialEq for test_allocator_MoveOnly_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_18(
                self as *const test_allocator_MoveOnly_,
                other as *const test_allocator_MoveOnly_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_MoveOnly_ {}
impl Clone for test_allocator_MoveOnly_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_MoveOnly_::copy_from(self as *const test_allocator_MoveOnly_) }
    }
}
impl Default for test_allocator_MoveOnly_ {
    fn default() -> Self {
        unsafe {
            test_allocator_MoveOnly_ {
                data_: 0_i32,
                id_: 0,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_MoveOnly_ptr_ {
    pub unsafe fn new_1(a: *const test_allocator_MoveOnly_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).converted.prefix_inc();
        }
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, DestructorUnsafe)]
#[cpp_from(test_allocator_MoveOnly_, new_1, ptr)]
pub struct test_allocator_MoveOnly_ptr_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_MoveOnly_ptr_ {
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u32,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut *mut MoveOnly {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14)))
        );
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                std::process::abort();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_add(n)) as i32;
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut MoveOnly, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_14)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        unimplemented!();
    }
}
impl Default for test_allocator_MoveOnly_ptr_ {
    fn default() -> Self {
        unsafe {
            test_allocator_MoveOnly_ptr_ {
                data_: 0_i32,
                id_: 0_i32,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_19(
    x: *const other_allocator_MoveOnly_,
    y: *const other_allocator_MoveOnly_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_MoveOnly_ {
    data_: i32,
}
impl other_allocator_MoveOnly_ {
    pub unsafe fn new_1(mut i: i32) -> Self {
        let mut this = Self { data_: i };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut MoveOnly {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut MoveOnly, mut s: usize) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for other_allocator_MoveOnly_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_19(
                self as *const other_allocator_MoveOnly_,
                other as *const other_allocator_MoveOnly_,
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_MoveOnly_ {}
impl Default for other_allocator_MoveOnly_ {
    fn default() -> Self {
        unsafe { other_allocator_MoveOnly_ { data_: 0_i32 } }
    }
}
impl other_allocator_MoveOnly_ptr_ {
    pub unsafe fn new_1(a: *const other_allocator_MoveOnly_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(other_allocator_MoveOnly_, new_1, ptr)]
pub struct other_allocator_MoveOnly_ptr_ {
    data_: i32,
}
impl other_allocator_MoveOnly_ptr_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut *mut MoveOnly {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut MoveOnly, mut s: usize) {
        unimplemented!();
    }
}
impl Default for other_allocator_MoveOnly_ptr_ {
    fn default() -> Self {
        unsafe { other_allocator_MoveOnly_ptr_ { data_: 0_i32 } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
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
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_21))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_22)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_21))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_22))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_23)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_20)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_21)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_22)) = 0_usize;
    }
}
pub static mut outstanding_bytes_20: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_21: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_22: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_23: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_24(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<MoveOnly>()) as i64
        as isize);
}
pub unsafe fn operator_eq_25(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_26(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_25(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut MoveOnly,
}
impl min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut MoveOnly) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut MoveOnly {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut MoveOnly {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self
            as *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_dec_10(
        &mut self,
    ) -> *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_dec();
        return &mut (*(self
            as *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::operator_sub_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_25(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_10();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    ptr_: *const MoveOnly,
}
impl min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_27(
    mut x : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__>())
        as i64 as isize);
}
pub unsafe fn operator_lt_28(
    mut x : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) < (y.ptr_));
}
pub unsafe fn operator_eq_29(
    mut x : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_30(
    mut x : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_29(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn to_bool ( &self , ) -> bool { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_10 ( &mut self , ) -> *mut min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : isize , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } pub unsafe fn operator_sub ( &self , mut n : isize , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_sub_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::Ord for  min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn cmp(&self, other: &Self) -> std::cmp::Ordering { unsafe { if operator_lt_28(self.clone(), other.clone()) { std::cmp::Ordering::Less } else if operator_lt_28(other.clone(), self.clone()) { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal } }}}
impl std::cmp::PartialOrd for min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }}
impl std::cmp::PartialEq for min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_29(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn cpp_inc(&mut self) { self.operator_inc_8(); } unsafe fn cpp_dec(&mut self) { self.operator_dec_10(); } }
pub unsafe fn operator_eq_31(
    mut _a0: min_allocator_MoveOnly_,
    mut _a1: min_allocator_MoveOnly_,
) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_MoveOnly_ {}
impl min_allocator_MoveOnly_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for min_allocator_MoveOnly_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_31(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_MoveOnly_ {}
impl min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_MoveOnly_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(min_allocator_MoveOnly_, new_1, value)]
pub struct min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
impl min_allocator_min_pointer_MoveOnly__std_integral_constant_unsigned_long__0___ {}
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
        let mut ab: [i32; 15] = [3, 4, 2, 8, 0, 1, 44, 34, 45, 96, 80, 1, 13, 31, 45];
        let mut an: *mut i32 = ab.as_mut_ptr().offset(
            ((::std::mem::size_of::<[i32; 15]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))) as isize,
        );
        let mut c1: Vec<MoveOnly> = unimplemented!();
        let mut p: *mut i32 = ab.as_mut_ptr();
        'loop_: while ((p) < (an)) {
            c1.push(MoveOnly::new({ Some((*p)) }));
            p.prefix_inc();
        }
        let mut c2: Vec<MoveOnly> = unimplemented!();
        let mut p: *mut i32 = ab.as_mut_ptr();
        'loop_: while ((p) < (an)) {
            c2.push(MoveOnly::new({ Some((*p)) }));
            p.prefix_inc();
        }
        let mut old_a: test_allocator_MoveOnly_ = unimplemented!();
        let _dtor_old_a =
            ScopedDestructorUnsafe::new(&raw mut old_a, test_allocator_MoveOnly_::destructor);
        let mut c3: Vec<MoveOnly> = std::mem::take(&mut c1);
        assert!(unimplemented!());
        assert!(((unimplemented!()) == (0_usize)));
        assert!(
            (unsafe {
                let mut _x: test_allocator_MoveOnly_ = unimplemented!();
                operator_eq_18(&mut _x, &old_a)
            })
        );
        assert!(
            (unsafe {
                let mut _x: test_allocator_MoveOnly_ = unimplemented!();
                let mut _y: test_allocator_MoveOnly_ = test_allocator_MoveOnly_::new_1({ 1 });
                operator_eq_18(&mut _x, &mut _y)
            })
        );
        assert!(
            ((unsafe { test_allocator_MoveOnly_::get_id(&unimplemented!(),) })
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_15)))
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let mut ab: [i32; 15] = [3, 4, 2, 8, 0, 1, 44, 34, 45, 96, 80, 1, 13, 31, 45];
        let mut an: *mut i32 = ab.as_mut_ptr().offset(
            ((::std::mem::size_of::<[i32; 15]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))) as isize,
        );
        let mut c1: Vec<MoveOnly> = unimplemented!();
        let mut p: *mut i32 = ab.as_mut_ptr();
        'loop_: while ((p) < (an)) {
            c1.push(MoveOnly::new({ Some((*p)) }));
            p.prefix_inc();
        }
        let mut c2: Vec<MoveOnly> = unimplemented!();
        let mut p: *mut i32 = ab.as_mut_ptr();
        'loop_: while ((p) < (an)) {
            c2.push(MoveOnly::new({ Some((*p)) }));
            p.prefix_inc();
        }
        let mut c3: Vec<MoveOnly> = std::mem::take(&mut c1);
        assert!(unimplemented!());
        assert!(((unimplemented!()) == (0_usize)));
        assert!(
            (unsafe {
                let mut _x: other_allocator_MoveOnly_ = unimplemented!();
                let mut _y: other_allocator_MoveOnly_ = unimplemented!();
                operator_eq_19(&mut _x, &mut _y)
            })
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let mut ab: [i32; 15] = [3, 4, 2, 8, 0, 1, 44, 34, 45, 96, 80, 1, 13, 31, 45];
        let mut an: *mut i32 = ab.as_mut_ptr().offset(
            ((::std::mem::size_of::<[i32; 15]>() as usize)
                .wrapping_div((::std::mem::size_of::<i32>() as usize))) as isize,
        );
        let mut c1: Vec<MoveOnly> = unimplemented!();
        let mut p: *mut i32 = ab.as_mut_ptr();
        'loop_: while ((p) < (an)) {
            c1.push(MoveOnly::new({ Some((*p)) }));
            p.prefix_inc();
        }
        let mut c2: Vec<MoveOnly> = unimplemented!();
        let mut p: *mut i32 = ab.as_mut_ptr();
        'loop_: while ((p) < (an)) {
            c2.push(MoveOnly::new({ Some((*p)) }));
            p.prefix_inc();
        }
        let mut c3: Vec<MoveOnly> = std::mem::take(&mut c1);
        assert!(unimplemented!());
        assert!(((unimplemented!()) == (0_usize)));
        assert!(
            (unsafe {
                let _arg0: min_allocator_MoveOnly_ = unimplemented!();
                let _arg1: min_allocator_MoveOnly_ = unimplemented!();
                operator_eq_31(_arg0, _arg1)
            })
        );
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_14);
    std::cell::LazyCell::force(&*&raw const moved_value_15);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_20);
    std::cell::LazyCell::force(&*&raw const alloc_count_21);
    std::cell::LazyCell::force(&*&raw const dealloc_count_22);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_23);
}
