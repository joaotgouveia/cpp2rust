extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn DoNotOptimize_0(value: *const *mut ::libc::c_void) -> *const *mut ::libc::c_void {
    c"";
    return value;
}
pub unsafe fn operator_eq_1(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_2(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_3(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_4(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_5(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_6(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_7(_a0: *const MoveOnly, _a1: *const MoveOnly) -> std::cmp::Ordering {
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
        unsafe { operator_cmp_7(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_8(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_9(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_10(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_13(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_14(
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
            operator_cmp_14(
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
            operator_eq_8(
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
pub static mut destructed_value_15: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_16: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_17(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_18(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_17(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
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
            operator_eq_17(
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
pub unsafe fn operator_eq_19(
    x: *const test_allocator_MoveOnly_,
    y: *const test_allocator_MoveOnly_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_MoveOnly__rebind_MoveOnly_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_MoveOnly_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_MoveOnly_ {
    pub unsafe fn new_1(mut data: i32, mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: stats,
        };
        if !((stats).is_null()) {
            (*this.stats_).count.prefix_inc();
        }
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
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15))))
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
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15))))
                && (!(c"moving from destroyed allocator".as_ptr()).is_null())
        );
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_16));
        this
    }
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u32,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut MoveOnly {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
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
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
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
            operator_eq_19(
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
pub unsafe fn operator_eq_20(x: *const test_allocator_int_, y: *const test_allocator_int_) -> bool {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_int_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_int_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ {
    pub unsafe fn new_1(
        mut data: i32,
        mut id: i32,
        mut stats: *mut test_allocator_statistics,
    ) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: stats,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
        }
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_int_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15))))
                && (!(c"copying from destroyed allocator".as_ptr()).is_null())
        );
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn move_from(a: *mut test_allocator_int_) -> Self {
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
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15))))
                && (!(c"moving from destroyed allocator".as_ptr()).is_null())
        );
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_16));
        this
    }
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        unimplemented!();
    }
    pub unsafe fn destroy(&mut self, mut p: *mut i32) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        };
    }
    pub unsafe fn get_data(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn get_id(&self) -> i32 {
        return self.id_;
    }
}
impl std::cmp::PartialEq for test_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_20(
                self as *const test_allocator_int_,
                other as *const test_allocator_int_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_int_ {}
impl Clone for test_allocator_int_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_int_::copy_from(self as *const test_allocator_int_) }
    }
}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        unsafe {
            test_allocator_int_ {
                data_: 0_i32,
                id_: 0_i32,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_21(
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
            operator_eq_21(
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
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_23))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_24)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_23))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_24))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_25)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_22)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_23)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_24)) = 0_usize;
    }
}
pub static mut outstanding_bytes_22: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_23: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_24: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_25: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_26(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<MoveOnly>()) as i64
        as isize);
}
pub unsafe fn operator_eq_27(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_28(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_27(_x, y)
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
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
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
        unsafe { operator_eq_27(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_29(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_30(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(
    min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
pub struct min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    ptr_: *const MoveOnly,
}
impl min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(
        mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const MoveOnly {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self
            as *mut min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__));
    }
}
impl std::cmp::PartialEq for min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_29(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *const ::libc::c_void,
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_31(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_32(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_33(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_32(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut i32) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut i32 {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_dec_10(
        &mut self,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_int__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_int__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_int__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_32(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_10();
    }
}
pub unsafe fn operator_sub_34(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(
    min_pointer_int__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const i32,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
}
pub unsafe fn operator_eq_35(
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
        unsafe { operator_eq_35(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_MoveOnly_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub unsafe fn operator_eq_36(
    mut _a0: safe_allocator_MoveOnly_,
    mut _a1: safe_allocator_MoveOnly_,
) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_MoveOnly_ {}
impl safe_allocator_MoveOnly_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut MoveOnly {
        let mut memory: *mut MoveOnly = unimplemented!();
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in 0..((::std::mem::size_of::<MoveOnly>() as u64)
                    .wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut MoveOnly, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in 0..((::std::mem::size_of::<MoveOnly>() as u64)
                        .wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl std::cmp::PartialEq for safe_allocator_MoveOnly_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_36(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for safe_allocator_MoveOnly_ {}
impl Default for safe_allocator_MoveOnly_ {
    fn default() -> Self {
        unsafe { safe_allocator_MoveOnly_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        let mut memory: *mut i32 = unimplemented!();
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in
                    0..((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in
                        0..((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        unsafe { safe_allocator_int_::new() }
    }
}
pub unsafe fn is_contiguous_container_asan_correct_37(_a0: *const Vec<MoveOnly>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_38(_a0: *const Vec<MoveOnly>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_39(_a0: *const Vec<i32>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_40(_a0: *const Vec<MoveOnly>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_41(_a0: *const Vec<i32>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_42(_a0: *const Vec<MoveOnly>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_43(_a0: *const Vec<i32>) -> bool {
    return true;
}
pub unsafe fn tests_44() -> bool {
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        let mut l: Vec<MoveOnly> = unimplemented!();
        let mut lo: Vec<MoveOnly> = unimplemented!();
        assert!((unsafe { is_contiguous_container_asan_correct_37(&l,) }));
        assert!((unsafe { is_contiguous_container_asan_correct_37(&lo,) }));
        let mut i: i32 = 1;
        'loop_: while ((i) <= (3)) {
            {
                let __a1 = MoveOnly::new({ Some(i) });
                l.push(__a1)
            };
            {
                let __a1 = MoveOnly::new({ Some(i) });
                lo.push(__a1)
            };
            i.prefix_inc();
        }
        assert!((unsafe { is_contiguous_container_asan_correct_37(&l,) }));
        assert!((unsafe { is_contiguous_container_asan_correct_37(&lo,) }));
        let mut l2: Vec<MoveOnly> = std::mem::take(&mut l);
        assert!(unimplemented!());
        assert!(l.is_empty());
        assert!(
            (unsafe {
                let mut _x: test_allocator_MoveOnly_ = unimplemented!();
                let mut _y: test_allocator_MoveOnly_ = unimplemented!();
                operator_eq_19(&mut _x, &mut _y)
            })
        );
        assert!((unsafe { is_contiguous_container_asan_correct_37(&l2,) }));
    }
    {
        let mut l: Vec<MoveOnly> = unimplemented!();
        let mut lo: Vec<MoveOnly> = unimplemented!();
        assert!((unsafe { is_contiguous_container_asan_correct_38(&l,) }));
        assert!((unsafe { is_contiguous_container_asan_correct_38(&lo,) }));
        let mut i: i32 = 1;
        'loop_: while ((i) <= (3)) {
            {
                let __a1 = MoveOnly::new({ Some(i) });
                l.push(__a1)
            };
            {
                let __a1 = MoveOnly::new({ Some(i) });
                lo.push(__a1)
            };
            i.prefix_inc();
        }
        assert!((unsafe { is_contiguous_container_asan_correct_38(&l,) }));
        assert!((unsafe { is_contiguous_container_asan_correct_38(&lo,) }));
        let mut l2: Vec<MoveOnly> = std::mem::take(&mut l);
        assert!(unimplemented!());
        assert!(l.is_empty());
        assert!(
            (unsafe {
                let mut _x: other_allocator_MoveOnly_ = unimplemented!();
                let mut _y: other_allocator_MoveOnly_ = unimplemented!();
                operator_eq_21(&mut _x, &mut _y)
            })
        );
        assert!((unsafe { is_contiguous_container_asan_correct_38(&l2,) }));
    }
    {
        let mut a1: [i32; 5] = [1, 3, 7, 9, 10];
        let mut c1: Vec<i32> = {
            let mut __v = Vec::new();
            let mut __it = a1.as_mut_ptr();
            while __it
                != a1.as_mut_ptr().offset(
                    ((::std::mem::size_of::<[i32; 5]>() as usize)
                        .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                        as isize,
                )
            {
                __v.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        };
        assert!((unsafe { is_contiguous_container_asan_correct_39(&c1,) }));
        let mut i: UnsafeRandomAccessIterator<i32> =
            <UnsafeRandomAccessIterator<i32>>::from(c1.begin());
        let mut c2: Vec<i32> = std::mem::take(&mut c1);
        assert!((unsafe { is_contiguous_container_asan_correct_39(&c2,) }));
        let mut j: UnsafeRandomAccessIterator<i32> = {
            let __it = i.clone();
            let pos =
                __it.iter_distance(&UnsafeRandomAccessIterator::Vec(c2.as_mut_ptr())) as usize;
            c2.remove(pos);
            __it
        };
        assert!(((*j.iter_deref()) == (3)));
        assert!((unsafe { is_contiguous_container_asan_correct_39(&c2,) }));
    }
    {
        let mut l: Vec<MoveOnly> = unimplemented!();
        let mut lo: Vec<MoveOnly> = unimplemented!();
        assert!((unsafe { is_contiguous_container_asan_correct_40(&l,) }));
        assert!((unsafe { is_contiguous_container_asan_correct_40(&lo,) }));
        let mut i: i32 = 1;
        'loop_: while ((i) <= (3)) {
            {
                let __a1 = MoveOnly::new({ Some(i) });
                l.push(__a1)
            };
            {
                let __a1 = MoveOnly::new({ Some(i) });
                lo.push(__a1)
            };
            i.prefix_inc();
        }
        assert!((unsafe { is_contiguous_container_asan_correct_40(&l,) }));
        assert!((unsafe { is_contiguous_container_asan_correct_40(&lo,) }));
        let mut l2: Vec<MoveOnly> = std::mem::take(&mut l);
        assert!(unimplemented!());
        assert!(l.is_empty());
        assert!(
            (unsafe {
                let _arg0: min_allocator_MoveOnly_ = unimplemented!();
                let _arg1: min_allocator_MoveOnly_ = unimplemented!();
                operator_eq_35(_arg0, _arg1)
            })
        );
        assert!((unsafe { is_contiguous_container_asan_correct_40(&l2,) }));
    }
    {
        let mut a1: [i32; 5] = [1, 3, 7, 9, 10];
        let mut c1: Vec<i32> = {
            let mut __v = Vec::new();
            let mut __it = a1.as_mut_ptr();
            while __it
                != a1.as_mut_ptr().offset(
                    ((::std::mem::size_of::<[i32; 5]>() as usize)
                        .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                        as isize,
                )
            {
                __v.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        };
        assert!((unsafe { is_contiguous_container_asan_correct_41(&c1,) }));
        let mut i: UnsafeRandomAccessIterator<i32> = <()>::from(c1.begin());
        let mut c2: Vec<i32> = std::mem::take(&mut c1);
        assert!((unsafe { is_contiguous_container_asan_correct_41(&c2,) }));
        let mut j: UnsafeRandomAccessIterator<i32> = {
            let __it = i.clone();
            let pos =
                __it.iter_distance(&UnsafeRandomAccessIterator::Vec(c2.as_mut_ptr())) as usize;
            c2.remove(pos);
            __it
        };
        assert!(((*j.iter_deref()) == (3)));
        assert!((unsafe { is_contiguous_container_asan_correct_41(&c2,) }));
    }
    {
        let mut l: Vec<MoveOnly> = unimplemented!();
        let mut lo: Vec<MoveOnly> = unimplemented!();
        assert!((unsafe { is_contiguous_container_asan_correct_42(&l,) }));
        assert!((unsafe { is_contiguous_container_asan_correct_42(&lo,) }));
        let mut i: i32 = 1;
        'loop_: while ((i) <= (3)) {
            {
                let __a1 = MoveOnly::new({ Some(i) });
                l.push(__a1)
            };
            {
                let __a1 = MoveOnly::new({ Some(i) });
                lo.push(__a1)
            };
            i.prefix_inc();
        }
        assert!((unsafe { is_contiguous_container_asan_correct_42(&l,) }));
        assert!((unsafe { is_contiguous_container_asan_correct_42(&lo,) }));
        let mut l2: Vec<MoveOnly> = std::mem::take(&mut l);
        assert!(unimplemented!());
        assert!(l.is_empty());
        assert!(
            (unsafe {
                let _arg0: safe_allocator_MoveOnly_ = unimplemented!();
                let _arg1: safe_allocator_MoveOnly_ = unimplemented!();
                operator_eq_36(_arg0, _arg1)
            })
        );
        assert!((unsafe { is_contiguous_container_asan_correct_42(&l2,) }));
    }
    {
        let mut a1: [i32; 5] = [1, 3, 7, 9, 10];
        let mut c1: Vec<i32> = {
            let mut __v = Vec::new();
            let mut __it = a1.as_mut_ptr();
            while __it
                != a1.as_mut_ptr().offset(
                    ((::std::mem::size_of::<[i32; 5]>() as usize)
                        .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                        as isize,
                )
            {
                __v.push((*__it.iter_deref()));
                __it.iter_inc();
            }
            __v
        };
        assert!((unsafe { is_contiguous_container_asan_correct_43(&c1,) }));
        let mut i: UnsafeRandomAccessIterator<i32> =
            <UnsafeRandomAccessIterator<i32>>::from(c1.begin());
        let mut c2: Vec<i32> = std::mem::take(&mut c1);
        assert!((unsafe { is_contiguous_container_asan_correct_43(&c2,) }));
        let mut j: UnsafeRandomAccessIterator<i32> = {
            let __it = i.clone();
            let pos =
                __it.iter_distance(&UnsafeRandomAccessIterator::Vec(c2.as_mut_ptr())) as usize;
            c2.remove(pos);
            __it
        };
        assert!(((*j.iter_deref()) == (3)));
        assert!((unsafe { is_contiguous_container_asan_correct_43(&c2,) }));
    }
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: Vec<i32> = unimplemented!();
        assert!(((alloc_stats.count) == (1)));
        assert!(((alloc_stats.copied) == (1)));
        assert!(((alloc_stats.moved) == (0)));
        {
            let mut __tmp_0: test_allocator_int_ = unimplemented!();
            let a: *const test_allocator_int_ = &mut __tmp_0;
            assert!(((unsafe { test_allocator_int_::get_data(&(*a),) }) == (42)));
            assert!(((unsafe { test_allocator_int_::get_id(&(*a),) }) == (101)));
        }
        assert!(((alloc_stats.count) == (1)));
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: Vec<i32> = std::mem::take(&mut v);
        assert!(((alloc_stats.count) == (2)));
        assert!(((alloc_stats.copied) == (0)));
        assert!(((alloc_stats.moved) == (1)));
        {
            let mut __tmp_1: test_allocator_int_ = unimplemented!();
            let a1: *const test_allocator_int_ = &mut __tmp_1;
            assert!(
                ((unsafe { test_allocator_int_::get_id(&(*a1),) })
                    == (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_16)))
            );
            assert!(((unsafe { test_allocator_int_::get_data(&(*a1),) }) == (42)));
            let mut __tmp_2: test_allocator_int_ = unimplemented!();
            let a2: *const test_allocator_int_ = &mut __tmp_2;
            assert!(((unsafe { test_allocator_int_::get_id(&(*a2),) }) == (101)));
            assert!(((unsafe { test_allocator_int_::get_data(&(*a2),) }) == (42)));
            assert!(
                (unsafe {
                    let _x: *const test_allocator_int_ = a1;
                    let _y: *const test_allocator_int_ = a2;
                    operator_eq_20(_x, _y)
                })
            );
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
    (unsafe { tests_44() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_15);
    std::cell::LazyCell::force(&*&raw const moved_value_16);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_22);
    std::cell::LazyCell::force(&*&raw const alloc_count_23);
    std::cell::LazyCell::force(&*&raw const dealloc_count_24);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_25);
}
