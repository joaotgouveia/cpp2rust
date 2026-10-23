extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_1(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_2(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_3(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_4(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_5(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_6(_a0: *const MoveOnly, _a1: *const MoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {
    data_: (),
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0_();
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> () {
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
pub unsafe fn operator_eq_7(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_9(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_10(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_11(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_12(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_13(_a0: *const TrivialMoveOnly, _a1: *const TrivialMoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct TrivialMoveOnly {
    data_: (),
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
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
    pub unsafe fn get(&self) -> () {
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
pub unsafe fn operator_eq_14(a: *const test_less_MoveOnly_, b: *const test_less_MoveOnly_) -> () {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_less_MoveOnly_ {
    pub data_: (),
}
impl test_less_MoveOnly_ {
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, a: *const MoveOnly, b: *const MoveOnly) -> () {
        return (unsafe {
            let _x: *const MoveOnly = a;
            let _y: *const MoveOnly = b;
            operator_lt_2(_x, _y)
        });
    }
}
impl std::cmp::PartialEq for test_less_MoveOnly_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_14(
                self as *const test_less_MoveOnly_,
                other as *const test_less_MoveOnly_,
            )
        }
    }
}
impl std::cmp::Eq for test_less_MoveOnly_ {}
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
pub static mut destructed_value_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_17(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_18(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_17(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
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
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_19(
    x: *const test_allocator_std_pair_const_MoveOnly__MoveOnly__,
    y: *const test_allocator_std_pair_const_MoveOnly__MoveOnly__,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_MoveOnly__MoveOnly___rebind_std_pair_const_MoveOnly__MoveOnly__
{}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_std_pair_const_MoveOnly__MoveOnly__) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        unimplemented!();
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
    }
}
impl std::cmp::PartialEq for test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_19(
                self as *const test_allocator_std_pair_const_MoveOnly__MoveOnly__,
                other as *const test_allocator_std_pair_const_MoveOnly__MoveOnly__,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl Clone for test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_std_pair_const_MoveOnly__MoveOnly__::copy_from(
                self as *const test_allocator_std_pair_const_MoveOnly__MoveOnly__,
            )
        }
    }
}
impl Default for test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
                data_: unimplemented!(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_20(
    x: *const test_allocator_std_pair_const_int__Counter_int___,
    y: *const test_allocator_std_pair_const_int__Counter_int___,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__Counter_int____rebind_std_pair_const_int__Counter_int___
{}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_pair_const_int__Counter_int___ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__Counter_int___ {
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_std_pair_const_int__Counter_int___) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        unimplemented!();
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_15));
    }
}
impl std::cmp::PartialEq for test_allocator_std_pair_const_int__Counter_int___ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_20(
                self as *const test_allocator_std_pair_const_int__Counter_int___,
                other as *const test_allocator_std_pair_const_int__Counter_int___,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_std_pair_const_int__Counter_int___ {}
impl Clone for test_allocator_std_pair_const_int__Counter_int___ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_std_pair_const_int__Counter_int___::copy_from(
                self as *const test_allocator_std_pair_const_int__Counter_int___,
            )
        }
    }
}
impl Default for test_allocator_std_pair_const_int__Counter_int___ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_int__Counter_int___ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_21(
    x: *const other_allocator_std_pair_const_MoveOnly__MoveOnly__,
    y: *const other_allocator_std_pair_const_MoveOnly__MoveOnly__,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    data_: (),
}
impl other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    pub unsafe fn new_1(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        this
    }
}
impl std::cmp::PartialEq for other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_21(
                self as *const other_allocator_std_pair_const_MoveOnly__MoveOnly__,
                other as *const other_allocator_std_pair_const_MoveOnly__MoveOnly__,
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl Default for other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn default() -> Self {
        unsafe {
            other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
                data_: unimplemented!(),
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
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_23))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_24))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_25)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_22)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_23)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_24)) = 0_();
    }
}
pub static mut outstanding_bytes_22: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_24: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_25: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__
{
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_MoveOnly__MoveOnly___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_26(
    mut _a0: min_allocator_std_pair_const_MoveOnly__MoveOnly__,
    mut _a1: min_allocator_std_pair_const_MoveOnly__MoveOnly__,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl std::cmp::PartialEq for min_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_26(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
pub unsafe fn operator_eq_27(
    mut _a0: explicit_allocator_std_pair_const_MoveOnly__MoveOnly__,
    mut _a1: explicit_allocator_std_pair_const_MoveOnly__MoveOnly__,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl std::cmp::PartialEq for explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_27(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl Default for explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn default() -> Self {
        unsafe { explicit_allocator_std_pair_const_MoveOnly__MoveOnly__::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Counter_base {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct Counter_int_ {
    data_: (),
}
impl Counter_int_ {
    pub unsafe fn new_1(data: *const ()) -> Self {
        let mut this = Self { data_: (*data) };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_28)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(rhs: *const Counter_int_) -> Self {
        let mut this = Self {
            data_: (*rhs).data_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_28)).prefix_inc();
        this
    }
    pub unsafe fn move_from(rhs: *mut Counter_int_) -> Self {
        let mut this = Self {
            data_: ((*rhs).data_ as ()),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_28)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_28)).prefix_dec();
    }
    pub unsafe fn operator_eq(&self, x: *const Counter_int_) -> () {
        return ((self.data_) == ((*x).data_));
    }
}
impl std::cmp::PartialEq for Counter_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Counter_int_::operator_eq(self, other as *const Counter_int_) }
    }
}
impl std::cmp::Eq for Counter_int_ {}
impl Clone for Counter_int_ {
    fn clone(&self) -> Self {
        unsafe { Counter_int_::copy_from(self as *const Counter_int_) }
    }
}
pub static mut gConstructed_28: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn test_29() -> () {
    {
        let mut a1: [(); 9] = [
            {
                let mut __tmp_0: () = 1_();
                let mut __tmp_1: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
            },
            {
                let mut __tmp_2: () = 1_();
                let mut __tmp_3: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_2 }, { &mut __tmp_3 })
            },
            {
                let mut __tmp_4: () = 1_();
                let mut __tmp_5: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_4 }, { &mut __tmp_5 })
            },
            {
                let mut __tmp_6: () = 2_();
                let mut __tmp_7: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_6 }, { &mut __tmp_7 })
            },
            {
                let mut __tmp_8: () = 2_();
                let mut __tmp_9: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_8 }, { &mut __tmp_9 })
            },
            {
                let mut __tmp_10: () = 2_();
                let mut __tmp_11: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_10 }, { &mut __tmp_11 })
            },
            {
                let mut __tmp_12: () = 3_();
                let mut __tmp_13: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_12 }, { &mut __tmp_13 })
            },
            {
                let mut __tmp_14: () = 3_();
                let mut __tmp_15: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_14 }, { &mut __tmp_15 })
            },
            {
                let mut __tmp_16: () = 3_();
                let mut __tmp_17: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_16 }, { &mut __tmp_17 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [(); 9] = [
            {
                let mut __tmp_18: () = 1_();
                let mut __tmp_19: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_18 }, { &mut __tmp_19 })
            },
            {
                let mut __tmp_20: () = 1_();
                let mut __tmp_21: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_20 }, { &mut __tmp_21 })
            },
            {
                let mut __tmp_22: () = 1_();
                let mut __tmp_23: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_22 }, { &mut __tmp_23 })
            },
            {
                let mut __tmp_24: () = 2_();
                let mut __tmp_25: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_24 }, { &mut __tmp_25 })
            },
            {
                let mut __tmp_26: () = 2_();
                let mut __tmp_27: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_26 }, { &mut __tmp_27 })
            },
            {
                let mut __tmp_28: () = 2_();
                let mut __tmp_29: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_28 }, { &mut __tmp_29 })
            },
            {
                let mut __tmp_30: () = 3_();
                let mut __tmp_31: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_30 }, { &mut __tmp_31 })
            },
            {
                let mut __tmp_32: () = 3_();
                let mut __tmp_33: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_32 }, { &mut __tmp_33 })
            },
            {
                let mut __tmp_34: () = 3_();
                let mut __tmp_35: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_34 }, { &mut __tmp_35 })
            },
        ];
        let mut m2: () = unimplemented!();
        let mut m3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a1: [(); 9] = [
            {
                let mut __tmp_36: () = 1_();
                let mut __tmp_37: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_36 }, { &mut __tmp_37 })
            },
            {
                let mut __tmp_38: () = 1_();
                let mut __tmp_39: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_38 }, { &mut __tmp_39 })
            },
            {
                let mut __tmp_40: () = 1_();
                let mut __tmp_41: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_40 }, { &mut __tmp_41 })
            },
            {
                let mut __tmp_42: () = 2_();
                let mut __tmp_43: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_42 }, { &mut __tmp_43 })
            },
            {
                let mut __tmp_44: () = 2_();
                let mut __tmp_45: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_44 }, { &mut __tmp_45 })
            },
            {
                let mut __tmp_46: () = 2_();
                let mut __tmp_47: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_46 }, { &mut __tmp_47 })
            },
            {
                let mut __tmp_48: () = 3_();
                let mut __tmp_49: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_48 }, { &mut __tmp_49 })
            },
            {
                let mut __tmp_50: () = 3_();
                let mut __tmp_51: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_50 }, { &mut __tmp_51 })
            },
            {
                let mut __tmp_52: () = 3_();
                let mut __tmp_53: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_52 }, { &mut __tmp_53 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [(); 9] = [
            {
                let mut __tmp_54: () = 1_();
                let mut __tmp_55: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_54 }, { &mut __tmp_55 })
            },
            {
                let mut __tmp_56: () = 1_();
                let mut __tmp_57: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_56 }, { &mut __tmp_57 })
            },
            {
                let mut __tmp_58: () = 1_();
                let mut __tmp_59: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_58 }, { &mut __tmp_59 })
            },
            {
                let mut __tmp_60: () = 2_();
                let mut __tmp_61: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_60 }, { &mut __tmp_61 })
            },
            {
                let mut __tmp_62: () = 2_();
                let mut __tmp_63: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_62 }, { &mut __tmp_63 })
            },
            {
                let mut __tmp_64: () = 2_();
                let mut __tmp_65: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_64 }, { &mut __tmp_65 })
            },
            {
                let mut __tmp_66: () = 3_();
                let mut __tmp_67: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_66 }, { &mut __tmp_67 })
            },
            {
                let mut __tmp_68: () = 3_();
                let mut __tmp_69: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_68 }, { &mut __tmp_69 })
            },
            {
                let mut __tmp_70: () = 3_();
                let mut __tmp_71: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_70 }, { &mut __tmp_71 })
            },
        ];
        let mut m2: () = unimplemented!();
        let mut m3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a1: [(); 9] = [
            {
                let mut __tmp_72: () = 1_();
                let mut __tmp_73: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_72 }, { &mut __tmp_73 })
            },
            {
                let mut __tmp_74: () = 1_();
                let mut __tmp_75: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_74 }, { &mut __tmp_75 })
            },
            {
                let mut __tmp_76: () = 1_();
                let mut __tmp_77: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_76 }, { &mut __tmp_77 })
            },
            {
                let mut __tmp_78: () = 2_();
                let mut __tmp_79: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_78 }, { &mut __tmp_79 })
            },
            {
                let mut __tmp_80: () = 2_();
                let mut __tmp_81: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_80 }, { &mut __tmp_81 })
            },
            {
                let mut __tmp_82: () = 2_();
                let mut __tmp_83: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_82 }, { &mut __tmp_83 })
            },
            {
                let mut __tmp_84: () = 3_();
                let mut __tmp_85: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_84 }, { &mut __tmp_85 })
            },
            {
                let mut __tmp_86: () = 3_();
                let mut __tmp_87: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_86 }, { &mut __tmp_87 })
            },
            {
                let mut __tmp_88: () = 3_();
                let mut __tmp_89: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_88 }, { &mut __tmp_89 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [(); 9] = [
            {
                let mut __tmp_90: () = 1_();
                let mut __tmp_91: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_90 }, { &mut __tmp_91 })
            },
            {
                let mut __tmp_92: () = 1_();
                let mut __tmp_93: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_92 }, { &mut __tmp_93 })
            },
            {
                let mut __tmp_94: () = 1_();
                let mut __tmp_95: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_94 }, { &mut __tmp_95 })
            },
            {
                let mut __tmp_96: () = 2_();
                let mut __tmp_97: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_96 }, { &mut __tmp_97 })
            },
            {
                let mut __tmp_98: () = 2_();
                let mut __tmp_99: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_98 }, { &mut __tmp_99 })
            },
            {
                let mut __tmp_100: () = 2_();
                let mut __tmp_101: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_100 }, { &mut __tmp_101 })
            },
            {
                let mut __tmp_102: () = 3_();
                let mut __tmp_103: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_102 }, { &mut __tmp_103 })
            },
            {
                let mut __tmp_104: () = 3_();
                let mut __tmp_105: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_104 }, { &mut __tmp_105 })
            },
            {
                let mut __tmp_106: () = 3_();
                let mut __tmp_107: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_106 }, { &mut __tmp_107 })
            },
        ];
        let mut m2: () = unimplemented!();
        let mut m3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_28)) = 0_();
        {
            let mut a1: [(); 9] = [
                {
                    let mut __tmp_108: () = 1_();
                    let mut __tmp_109: () = 1_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_108 }, { &mut __tmp_109 })
                },
                {
                    let mut __tmp_110: () = 1_();
                    let mut __tmp_111: () = 2_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_110 }, { &mut __tmp_111 })
                },
                {
                    let mut __tmp_112: () = 1_();
                    let mut __tmp_113: () = 3_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_112 }, { &mut __tmp_113 })
                },
                {
                    let mut __tmp_114: () = 2_();
                    let mut __tmp_115: () = 1_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_114 }, { &mut __tmp_115 })
                },
                {
                    let mut __tmp_116: () = 2_();
                    let mut __tmp_117: () = 2_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_116 }, { &mut __tmp_117 })
                },
                {
                    let mut __tmp_118: () = 2_();
                    let mut __tmp_119: () = 3_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_118 }, { &mut __tmp_119 })
                },
                {
                    let mut __tmp_120: () = 3_();
                    let mut __tmp_121: () = 1_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_120 }, { &mut __tmp_121 })
                },
                {
                    let mut __tmp_122: () = 3_();
                    let mut __tmp_123: () = 2_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_122 }, { &mut __tmp_123 })
                },
                {
                    let mut __tmp_124: () = 3_();
                    let mut __tmp_125: () = 3_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_124 }, { &mut __tmp_125 })
                },
            ];
            let num: () = (::std::mem::size_of::<[(); 9]>() as ())
                .wrapping_div((::std::mem::size_of::<()>() as ()));
            unimplemented!();
            let mut m1: () = unimplemented!();
            unimplemented!();
            let mut m2: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            let mut m3: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                let mut m4: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
    }
    {
        let mut a1: [(); 9] = [
            {
                let mut __tmp_126: () = 1_();
                let mut __tmp_127: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_126 }, { &mut __tmp_127 })
            },
            {
                let mut __tmp_128: () = 1_();
                let mut __tmp_129: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_128 }, { &mut __tmp_129 })
            },
            {
                let mut __tmp_130: () = 1_();
                let mut __tmp_131: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_130 }, { &mut __tmp_131 })
            },
            {
                let mut __tmp_132: () = 2_();
                let mut __tmp_133: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_132 }, { &mut __tmp_133 })
            },
            {
                let mut __tmp_134: () = 2_();
                let mut __tmp_135: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_134 }, { &mut __tmp_135 })
            },
            {
                let mut __tmp_136: () = 2_();
                let mut __tmp_137: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_136 }, { &mut __tmp_137 })
            },
            {
                let mut __tmp_138: () = 3_();
                let mut __tmp_139: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_138 }, { &mut __tmp_139 })
            },
            {
                let mut __tmp_140: () = 3_();
                let mut __tmp_141: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_140 }, { &mut __tmp_141 })
            },
            {
                let mut __tmp_142: () = 3_();
                let mut __tmp_143: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_142 }, { &mut __tmp_143 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [(); 9] = [
            {
                let mut __tmp_144: () = 1_();
                let mut __tmp_145: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_144 }, { &mut __tmp_145 })
            },
            {
                let mut __tmp_146: () = 1_();
                let mut __tmp_147: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_146 }, { &mut __tmp_147 })
            },
            {
                let mut __tmp_148: () = 1_();
                let mut __tmp_149: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_148 }, { &mut __tmp_149 })
            },
            {
                let mut __tmp_150: () = 2_();
                let mut __tmp_151: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_150 }, { &mut __tmp_151 })
            },
            {
                let mut __tmp_152: () = 2_();
                let mut __tmp_153: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_152 }, { &mut __tmp_153 })
            },
            {
                let mut __tmp_154: () = 2_();
                let mut __tmp_155: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_154 }, { &mut __tmp_155 })
            },
            {
                let mut __tmp_156: () = 3_();
                let mut __tmp_157: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_156 }, { &mut __tmp_157 })
            },
            {
                let mut __tmp_158: () = 3_();
                let mut __tmp_159: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_158 }, { &mut __tmp_159 })
            },
            {
                let mut __tmp_160: () = 3_();
                let mut __tmp_161: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_160 }, { &mut __tmp_161 })
            },
        ];
        let mut m2: () = unimplemented!();
        let mut m3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a1: [(); 9] = [
            {
                let mut __tmp_162: () = 1_();
                let mut __tmp_163: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_162 }, { &mut __tmp_163 })
            },
            {
                let mut __tmp_164: () = 1_();
                let mut __tmp_165: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_164 }, { &mut __tmp_165 })
            },
            {
                let mut __tmp_166: () = 1_();
                let mut __tmp_167: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_166 }, { &mut __tmp_167 })
            },
            {
                let mut __tmp_168: () = 2_();
                let mut __tmp_169: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_168 }, { &mut __tmp_169 })
            },
            {
                let mut __tmp_170: () = 2_();
                let mut __tmp_171: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_170 }, { &mut __tmp_171 })
            },
            {
                let mut __tmp_172: () = 2_();
                let mut __tmp_173: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_172 }, { &mut __tmp_173 })
            },
            {
                let mut __tmp_174: () = 3_();
                let mut __tmp_175: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_174 }, { &mut __tmp_175 })
            },
            {
                let mut __tmp_176: () = 3_();
                let mut __tmp_177: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_176 }, { &mut __tmp_177 })
            },
            {
                let mut __tmp_178: () = 3_();
                let mut __tmp_179: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_178 }, { &mut __tmp_179 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [(); 9] = [
            {
                let mut __tmp_180: () = 1_();
                let mut __tmp_181: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_180 }, { &mut __tmp_181 })
            },
            {
                let mut __tmp_182: () = 1_();
                let mut __tmp_183: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_182 }, { &mut __tmp_183 })
            },
            {
                let mut __tmp_184: () = 1_();
                let mut __tmp_185: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_184 }, { &mut __tmp_185 })
            },
            {
                let mut __tmp_186: () = 2_();
                let mut __tmp_187: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_186 }, { &mut __tmp_187 })
            },
            {
                let mut __tmp_188: () = 2_();
                let mut __tmp_189: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_188 }, { &mut __tmp_189 })
            },
            {
                let mut __tmp_190: () = 2_();
                let mut __tmp_191: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_190 }, { &mut __tmp_191 })
            },
            {
                let mut __tmp_192: () = 3_();
                let mut __tmp_193: () = 1_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_192 }, { &mut __tmp_193 })
            },
            {
                let mut __tmp_194: () = 3_();
                let mut __tmp_195: () = 2_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_194 }, { &mut __tmp_195 })
            },
            {
                let mut __tmp_196: () = 3_();
                let mut __tmp_197: () = 3_();
                std_pair_MoveOnly__MoveOnly_::new({ &mut __tmp_196 }, { &mut __tmp_197 })
            },
        ];
        let mut m2: () = unimplemented!();
        let mut m3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_29() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_15);
    std::cell::LazyCell::force(&*&raw const moved_value_16);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_22);
    std::cell::LazyCell::force(&*&raw const alloc_count_23);
    std::cell::LazyCell::force(&*&raw const dealloc_count_24);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_25);
    std::cell::LazyCell::force(&*&raw const gConstructed_28);
}
