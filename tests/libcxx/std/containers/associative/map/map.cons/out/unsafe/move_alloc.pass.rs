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
pub unsafe fn operator_eq_14(x: *const CopyConstructible, y: *const CopyConstructible) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_15(x: *const CopyConstructible, y: *const CopyConstructible) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_16(x: *const CopyConstructible, y: *const CopyConstructible) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_17(x: *const CopyConstructible, y: *const CopyConstructible) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_18(x: *const CopyConstructible, y: *const CopyConstructible) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_19(x: *const CopyConstructible, y: *const CopyConstructible) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_20(_a0: *const CopyConstructible, _a1: *const CopyConstructible) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct CopyConstructible {
    data_: (),
}
impl CopyConstructible {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut CopyConstructible) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut CopyConstructible) -> *mut CopyConstructible {
        self.data_ = (*x).data_;
        (*x).data_ = 0_();
        return &mut (*(self as *mut CopyConstructible));
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const CopyConstructible) -> CopyConstructible {
        return CopyConstructible::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const CopyConstructible) -> CopyConstructible {
        return CopyConstructible::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for CopyConstructible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_20(
                self as *const CopyConstructible,
                other as *const CopyConstructible,
            )
        }
    }
}
impl std::cmp::PartialOrd for CopyConstructible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for CopyConstructible {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_14(
                self as *const CopyConstructible,
                other as *const CopyConstructible,
            )
        }
    }
}
impl std::cmp::Eq for CopyConstructible {}
impl Default for CopyConstructible {
    fn default() -> Self {
        unsafe { CopyConstructible::new(None) }
    }
}
const _: () = assert!(true, "std::is_copy_constructible<CopyConstructible>::value");
pub unsafe fn operator_eq_21(a: *const test_less_MoveOnly_, b: *const test_less_MoveOnly_) -> () {
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
            operator_eq_21(
                self as *const test_less_MoveOnly_,
                other as *const test_less_MoveOnly_,
            )
        }
    }
}
impl std::cmp::Eq for test_less_MoveOnly_ {}
pub unsafe fn operator_eq_22(
    a: *const test_less_CopyConstructible_,
    b: *const test_less_CopyConstructible_,
) -> () {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_less_CopyConstructible_ {
    pub data_: (),
}
impl test_less_CopyConstructible_ {
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(
        &self,
        a: *const CopyConstructible,
        b: *const CopyConstructible,
    ) -> () {
        return (unsafe {
            let _x: *const CopyConstructible = a;
            let _y: *const CopyConstructible = b;
            operator_lt_16(_x, _y)
        });
    }
}
impl std::cmp::PartialEq for test_less_CopyConstructible_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_22(
                self as *const test_less_CopyConstructible_,
                other as *const test_less_CopyConstructible_,
            )
        }
    }
}
impl std::cmp::Eq for test_less_CopyConstructible_ {}
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
pub static mut destructed_value_23: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_24: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_25(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_26(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_25(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
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
            operator_eq_25(
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
impl test_allocator_std_pair_const_int__Counter_int___ {
    pub unsafe fn new_1(
        a : *const test_allocator_std___tree_node_std___value_type_const_int__Counter_int____void_ptr__,
    ) -> Self {
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
pub unsafe fn operator_eq_27(
    x: *const test_allocator_std_pair_const_int__Counter_int___,
    y: *const test_allocator_std_pair_const_int__Counter_int___,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__Counter_int____rebind_std___tree_node_std___value_type_const_int__Counter_int____void_ptr__
{}
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
    pub unsafe fn new_2(mut data: ()) -> Self {
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
    }
}
impl std::cmp::PartialEq for test_allocator_std_pair_const_int__Counter_int___ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_27(
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
impl test_allocator_std___tree_node_std___value_type_const_int__Counter_int____void_ptr__ {
    pub unsafe fn new_1(a: *const test_allocator_std_pair_const_int__Counter_int___) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std___tree_node_std___value_type_const_int__Counter_int____void_ptr___rebind_std___tree_node_base_void_ptr__
{}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std___tree_node_std___value_type_const_int__Counter_int____void_ptr__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___tree_node_std___value_type_const_int__Counter_int____void_ptr__ {
    pub unsafe fn copy_from(
        a : *const test_allocator_std___tree_node_std___value_type_const_int__Counter_int____void_ptr__,
    ) -> Self {
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
        mut _a1: Option<()>,
    ) -> *mut std___tree_node_std___value_type_const_int__Counter_int____void_ptr_ {
        let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
        unimplemented!();
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                unimplemented!();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_add(n);
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: *mut std___tree_node_std___value_type_const_int__Counter_int____void_ptr_,
        mut s: (),
    ) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
}
impl Clone
    for test_allocator_std___tree_node_std___value_type_const_int__Counter_int____void_ptr__
{
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_std___tree_node_std___value_type_const_int__Counter_int____void_ptr__::copy_from(self as *const test_allocator_std___tree_node_std___value_type_const_int__Counter_int____void_ptr__)
        }
    }
}
impl Default
    for test_allocator_std___tree_node_std___value_type_const_int__Counter_int____void_ptr__
{
    fn default() -> Self {
        unsafe {
            test_allocator_std___tree_node_std___value_type_const_int__Counter_int____void_ptr__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_std___tree_node_base_void_ptr__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___tree_node_base_void_ptr__ {}
impl Default for test_allocator_std___tree_node_base_void_ptr__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std___tree_node_base_void_ptr__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    pub unsafe fn new_1(
        a: *const test_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__,
    ) -> Self {
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
pub unsafe fn operator_eq_28(
    x: *const test_allocator_std_pair_const_MoveOnly__MoveOnly__,
    y: *const test_allocator_std_pair_const_MoveOnly__MoveOnly__,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_MoveOnly__MoveOnly___rebind_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__
{}
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
    pub unsafe fn new_2(mut data: ()) -> Self {
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
    }
}
impl std::cmp::PartialEq for test_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_28(
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
impl test_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {
    pub unsafe fn new_1(a: *const test_allocator_std_pair_const_MoveOnly__MoveOnly__) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr___rebind_std___tree_node_base_void_ptr__
{}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
        mut _a1: Option<()>,
    ) -> *mut std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr_ {
        let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
        unimplemented!();
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                unimplemented!();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_add(n);
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: *mut std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr_,
        mut s: (),
    ) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
}
impl Default for test_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    pub unsafe fn new_1(
        a : *const test_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__,
    ) -> Self {
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
pub unsafe fn operator_eq_29(
    x: *const test_allocator_std_pair_const_CopyConstructible__MoveOnly__,
    y: *const test_allocator_std_pair_const_CopyConstructible__MoveOnly__,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_CopyConstructible__MoveOnly___rebind_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_CopyConstructible__MoveOnly___rebind_std_pair_const_CopyConstructible__MoveOnly__
{}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(
        a: *const test_allocator_std_pair_const_CopyConstructible__MoveOnly__,
    ) -> Self {
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
    }
}
impl std::cmp::PartialEq for test_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_29(
                self as *const test_allocator_std_pair_const_CopyConstructible__MoveOnly__,
                other as *const test_allocator_std_pair_const_CopyConstructible__MoveOnly__,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_std_pair_const_CopyConstructible__MoveOnly__ {}
impl Clone for test_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_std_pair_const_CopyConstructible__MoveOnly__::copy_from(
                self as *const test_allocator_std_pair_const_CopyConstructible__MoveOnly__,
            )
        }
    }
}
impl Default for test_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
                data_: unimplemented!(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__ {
    pub unsafe fn new_1(
        a: *const test_allocator_std_pair_const_CopyConstructible__MoveOnly__,
    ) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr___rebind_std___tree_node_base_void_ptr__
{}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__
{
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__ {
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_23));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
        mut _a1: Option<()>,
    ) -> *mut std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr_ {
        let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
        unimplemented!();
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                unimplemented!();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_add(n);
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: *mut std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr_,
        mut s: (),
    ) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
}
impl Default
    for test_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__
{
    fn default() -> Self {
        unsafe {
            test_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__ { data_ : unimplemented!() , id_ : unimplemented!() , stats_ : std::ptr::null_mut() , }
        }
    }
}
impl other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    pub unsafe fn new_1(
        a: *const other_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__,
    ) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
pub unsafe fn operator_eq_30(
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
    pub unsafe fn new_2(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        this
    }
}
impl std::cmp::PartialEq for other_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_30(
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
impl other_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {
    pub unsafe fn new_1(a: *const other_allocator_std_pair_const_MoveOnly__MoveOnly__) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {
    data_: (),
}
impl other_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> *mut std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr_ {
        return unimplemented!();
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: *mut std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr_,
        mut s: (),
    ) {
        unimplemented!();
    }
}
impl Default for other_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {
    fn default() -> Self {
        unsafe {
            other_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {
                data_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_std___tree_node_base_void_ptr__ {
    data_: (),
}
impl other_allocator_std___tree_node_base_void_ptr__ {}
impl Default for other_allocator_std___tree_node_base_void_ptr__ {
    fn default() -> Self {
        unsafe {
            other_allocator_std___tree_node_base_void_ptr__ {
                data_: unimplemented!(),
            }
        }
    }
}
impl other_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    pub unsafe fn new_1(
        a : *const other_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__,
    ) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
pub unsafe fn operator_eq_31(
    x: *const other_allocator_std_pair_const_CopyConstructible__MoveOnly__,
    y: *const other_allocator_std_pair_const_CopyConstructible__MoveOnly__,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    data_: (),
}
impl other_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    pub unsafe fn new_2(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        this
    }
}
impl std::cmp::PartialEq for other_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_31(
                self as *const other_allocator_std_pair_const_CopyConstructible__MoveOnly__,
                other as *const other_allocator_std_pair_const_CopyConstructible__MoveOnly__,
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_std_pair_const_CopyConstructible__MoveOnly__ {}
impl Default for other_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    fn default() -> Self {
        unsafe {
            other_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
                data_: unimplemented!(),
            }
        }
    }
}
impl other_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__ {
    pub unsafe fn new_1(
        a: *const other_allocator_std_pair_const_CopyConstructible__MoveOnly__,
    ) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__
{
    data_: (),
}
impl other_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> *mut std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr_ {
        return unimplemented!();
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: *mut std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr_,
        mut s: (),
    ) {
        unimplemented!();
    }
}
impl Default
    for other_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__
{
    fn default() -> Self {
        unsafe {
            other_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__ { data_ : unimplemented!() , }
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
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_33))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_34))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_35)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_32)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_33)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_34)) = 0_();
    }
}
pub static mut outstanding_bytes_32: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_33: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_34: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_35: std::cell::LazyCell<()> =
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
pub unsafe fn operator_eq_36(
    mut x : min_pointer_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_36(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_37(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_38(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_37(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___,
}
impl min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_37(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_39(
    mut x : min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_39(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_pair_const_CopyConstructible__MoveOnly___std_integral_constant_unsigned_long__0__
{
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_CopyConstructible__MoveOnly___std_integral_constant_unsigned_long__0__ { }
pub unsafe fn operator_eq_40(
    mut x : min_pointer_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_40(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
impl min_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    pub unsafe fn new_1(
        mut _a0 : min_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0____,
    ) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn operator_eq_41(
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
        unsafe { operator_eq_41(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl min_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0____ { pub unsafe fn new_1 ( mut _a0 : min_allocator_std_pair_const_MoveOnly__MoveOnly__ , ) -> Self { let mut this = Self { } ;
 this } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl min_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0____ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl min_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    pub unsafe fn new_1(
        mut _a0 : min_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0____,
    ) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn operator_eq_42(
    mut _a0: min_allocator_std_pair_const_CopyConstructible__MoveOnly__,
    mut _a1: min_allocator_std_pair_const_CopyConstructible__MoveOnly__,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_CopyConstructible__MoveOnly__ {}
impl std::cmp::PartialEq for min_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_42(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_std_pair_const_CopyConstructible__MoveOnly__ {}
impl min_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0____ { pub unsafe fn new_1 ( mut _a0 : min_allocator_std_pair_const_CopyConstructible__MoveOnly__ , ) -> Self { let mut this = Self { } ;
 this } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl min_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0____ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
impl min_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl min_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0____ { }
impl min_allocator_std_pair_const_CopyConstructible__MoveOnly__ {}
impl min_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___min_pointer_void__std_integral_constant_unsigned_long__0____ { }
impl explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    pub unsafe fn new_1(
        mut _a0 : explicit_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__,
    ) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn operator_eq_43(
    mut _a0: explicit_allocator_std_pair_const_MoveOnly__MoveOnly__,
    mut _a1: explicit_allocator_std_pair_const_MoveOnly__MoveOnly__,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    pub unsafe fn new_2() -> Self {
        let mut this = Self {};
        this
    }
}
impl std::cmp::PartialEq for explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_43(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {}
impl Default for explicit_allocator_std_pair_const_MoveOnly__MoveOnly__ {
    fn default() -> Self {
        unsafe { explicit_allocator_std_pair_const_MoveOnly__MoveOnly__::new_2() }
    }
}
impl explicit_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {
    pub unsafe fn new_1(mut _a0: explicit_allocator_std_pair_const_MoveOnly__MoveOnly__) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct explicit_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {}
impl explicit_allocator_std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr__ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> *mut std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr_ {
        return (unimplemented!()
            as *mut std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr_);
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: *mut std___tree_node_std___value_type_MoveOnly__MoveOnly___void_ptr_,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct explicit_allocator_std___tree_node_base_void_ptr__ {}
impl explicit_allocator_std___tree_node_base_void_ptr__ {}
impl explicit_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    pub unsafe fn new_1(
        mut _a0 : explicit_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__,
    ) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn operator_eq_44(
    mut _a0: explicit_allocator_std_pair_const_CopyConstructible__MoveOnly__,
    mut _a1: explicit_allocator_std_pair_const_CopyConstructible__MoveOnly__,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct explicit_allocator_std_pair_const_CopyConstructible__MoveOnly__ {}
impl explicit_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    pub unsafe fn new_2() -> Self {
        let mut this = Self {};
        this
    }
}
impl std::cmp::PartialEq for explicit_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_44(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for explicit_allocator_std_pair_const_CopyConstructible__MoveOnly__ {}
impl Default for explicit_allocator_std_pair_const_CopyConstructible__MoveOnly__ {
    fn default() -> Self {
        unsafe { explicit_allocator_std_pair_const_CopyConstructible__MoveOnly__::new_2() }
    }
}
impl explicit_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__ {
    pub unsafe fn new_1(
        mut _a0: explicit_allocator_std_pair_const_CopyConstructible__MoveOnly__,
    ) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct explicit_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__
{}
impl explicit_allocator_std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr__ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> *mut std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr_ {
        return (unimplemented!()
            as *mut std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr_);
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: *mut std___tree_node_std___value_type_CopyConstructible__MoveOnly___void_ptr_,
        mut n: (),
    ) {
        unimplemented!();
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_45)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(rhs: *const Counter_int_) -> Self {
        let mut this = Self {
            data_: (*rhs).data_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_45)).prefix_inc();
        this
    }
    pub unsafe fn move_from(rhs: *mut Counter_int_) -> Self {
        let mut this = Self {
            data_: ((*rhs).data_ as ()),
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_45)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_45)).prefix_dec();
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
pub static mut gConstructed_45: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn test_move_alloc_46() -> () {
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_45)) = 0_();
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
pub unsafe fn test_move_alloc_47() -> () {
    {
        let mut a1: [(); 9] = [
            {
                let mut __tmp_198: () = 1_();
                let mut __tmp_199: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_198 }, { &mut __tmp_199 })
            },
            {
                let mut __tmp_200: () = 1_();
                let mut __tmp_201: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_200 }, { &mut __tmp_201 })
            },
            {
                let mut __tmp_202: () = 1_();
                let mut __tmp_203: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_202 }, { &mut __tmp_203 })
            },
            {
                let mut __tmp_204: () = 2_();
                let mut __tmp_205: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_204 }, { &mut __tmp_205 })
            },
            {
                let mut __tmp_206: () = 2_();
                let mut __tmp_207: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_206 }, { &mut __tmp_207 })
            },
            {
                let mut __tmp_208: () = 2_();
                let mut __tmp_209: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_208 }, { &mut __tmp_209 })
            },
            {
                let mut __tmp_210: () = 3_();
                let mut __tmp_211: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_210 }, { &mut __tmp_211 })
            },
            {
                let mut __tmp_212: () = 3_();
                let mut __tmp_213: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_212 }, { &mut __tmp_213 })
            },
            {
                let mut __tmp_214: () = 3_();
                let mut __tmp_215: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_214 }, { &mut __tmp_215 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [(); 9] = [
            {
                let mut __tmp_216: () = 1_();
                let mut __tmp_217: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_216 }, { &mut __tmp_217 })
            },
            {
                let mut __tmp_218: () = 1_();
                let mut __tmp_219: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_218 }, { &mut __tmp_219 })
            },
            {
                let mut __tmp_220: () = 1_();
                let mut __tmp_221: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_220 }, { &mut __tmp_221 })
            },
            {
                let mut __tmp_222: () = 2_();
                let mut __tmp_223: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_222 }, { &mut __tmp_223 })
            },
            {
                let mut __tmp_224: () = 2_();
                let mut __tmp_225: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_224 }, { &mut __tmp_225 })
            },
            {
                let mut __tmp_226: () = 2_();
                let mut __tmp_227: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_226 }, { &mut __tmp_227 })
            },
            {
                let mut __tmp_228: () = 3_();
                let mut __tmp_229: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_228 }, { &mut __tmp_229 })
            },
            {
                let mut __tmp_230: () = 3_();
                let mut __tmp_231: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_230 }, { &mut __tmp_231 })
            },
            {
                let mut __tmp_232: () = 3_();
                let mut __tmp_233: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_232 }, { &mut __tmp_233 })
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
                let mut __tmp_234: () = 1_();
                let mut __tmp_235: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_234 }, { &mut __tmp_235 })
            },
            {
                let mut __tmp_236: () = 1_();
                let mut __tmp_237: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_236 }, { &mut __tmp_237 })
            },
            {
                let mut __tmp_238: () = 1_();
                let mut __tmp_239: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_238 }, { &mut __tmp_239 })
            },
            {
                let mut __tmp_240: () = 2_();
                let mut __tmp_241: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_240 }, { &mut __tmp_241 })
            },
            {
                let mut __tmp_242: () = 2_();
                let mut __tmp_243: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_242 }, { &mut __tmp_243 })
            },
            {
                let mut __tmp_244: () = 2_();
                let mut __tmp_245: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_244 }, { &mut __tmp_245 })
            },
            {
                let mut __tmp_246: () = 3_();
                let mut __tmp_247: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_246 }, { &mut __tmp_247 })
            },
            {
                let mut __tmp_248: () = 3_();
                let mut __tmp_249: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_248 }, { &mut __tmp_249 })
            },
            {
                let mut __tmp_250: () = 3_();
                let mut __tmp_251: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_250 }, { &mut __tmp_251 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [(); 9] = [
            {
                let mut __tmp_252: () = 1_();
                let mut __tmp_253: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_252 }, { &mut __tmp_253 })
            },
            {
                let mut __tmp_254: () = 1_();
                let mut __tmp_255: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_254 }, { &mut __tmp_255 })
            },
            {
                let mut __tmp_256: () = 1_();
                let mut __tmp_257: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_256 }, { &mut __tmp_257 })
            },
            {
                let mut __tmp_258: () = 2_();
                let mut __tmp_259: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_258 }, { &mut __tmp_259 })
            },
            {
                let mut __tmp_260: () = 2_();
                let mut __tmp_261: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_260 }, { &mut __tmp_261 })
            },
            {
                let mut __tmp_262: () = 2_();
                let mut __tmp_263: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_262 }, { &mut __tmp_263 })
            },
            {
                let mut __tmp_264: () = 3_();
                let mut __tmp_265: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_264 }, { &mut __tmp_265 })
            },
            {
                let mut __tmp_266: () = 3_();
                let mut __tmp_267: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_266 }, { &mut __tmp_267 })
            },
            {
                let mut __tmp_268: () = 3_();
                let mut __tmp_269: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_268 }, { &mut __tmp_269 })
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
                let mut __tmp_270: () = 1_();
                let mut __tmp_271: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_270 }, { &mut __tmp_271 })
            },
            {
                let mut __tmp_272: () = 1_();
                let mut __tmp_273: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_272 }, { &mut __tmp_273 })
            },
            {
                let mut __tmp_274: () = 1_();
                let mut __tmp_275: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_274 }, { &mut __tmp_275 })
            },
            {
                let mut __tmp_276: () = 2_();
                let mut __tmp_277: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_276 }, { &mut __tmp_277 })
            },
            {
                let mut __tmp_278: () = 2_();
                let mut __tmp_279: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_278 }, { &mut __tmp_279 })
            },
            {
                let mut __tmp_280: () = 2_();
                let mut __tmp_281: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_280 }, { &mut __tmp_281 })
            },
            {
                let mut __tmp_282: () = 3_();
                let mut __tmp_283: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_282 }, { &mut __tmp_283 })
            },
            {
                let mut __tmp_284: () = 3_();
                let mut __tmp_285: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_284 }, { &mut __tmp_285 })
            },
            {
                let mut __tmp_286: () = 3_();
                let mut __tmp_287: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_286 }, { &mut __tmp_287 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [(); 9] = [
            {
                let mut __tmp_288: () = 1_();
                let mut __tmp_289: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_288 }, { &mut __tmp_289 })
            },
            {
                let mut __tmp_290: () = 1_();
                let mut __tmp_291: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_290 }, { &mut __tmp_291 })
            },
            {
                let mut __tmp_292: () = 1_();
                let mut __tmp_293: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_292 }, { &mut __tmp_293 })
            },
            {
                let mut __tmp_294: () = 2_();
                let mut __tmp_295: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_294 }, { &mut __tmp_295 })
            },
            {
                let mut __tmp_296: () = 2_();
                let mut __tmp_297: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_296 }, { &mut __tmp_297 })
            },
            {
                let mut __tmp_298: () = 2_();
                let mut __tmp_299: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_298 }, { &mut __tmp_299 })
            },
            {
                let mut __tmp_300: () = 3_();
                let mut __tmp_301: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_300 }, { &mut __tmp_301 })
            },
            {
                let mut __tmp_302: () = 3_();
                let mut __tmp_303: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_302 }, { &mut __tmp_303 })
            },
            {
                let mut __tmp_304: () = 3_();
                let mut __tmp_305: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_304 }, { &mut __tmp_305 })
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
        (*std::cell::LazyCell::force_mut(&mut *&raw mut gConstructed_45)) = 0_();
        {
            let mut a1: [(); 9] = [
                {
                    let mut __tmp_306: () = 1_();
                    let mut __tmp_307: () = 1_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_306 }, { &mut __tmp_307 })
                },
                {
                    let mut __tmp_308: () = 1_();
                    let mut __tmp_309: () = 2_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_308 }, { &mut __tmp_309 })
                },
                {
                    let mut __tmp_310: () = 1_();
                    let mut __tmp_311: () = 3_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_310 }, { &mut __tmp_311 })
                },
                {
                    let mut __tmp_312: () = 2_();
                    let mut __tmp_313: () = 1_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_312 }, { &mut __tmp_313 })
                },
                {
                    let mut __tmp_314: () = 2_();
                    let mut __tmp_315: () = 2_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_314 }, { &mut __tmp_315 })
                },
                {
                    let mut __tmp_316: () = 2_();
                    let mut __tmp_317: () = 3_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_316 }, { &mut __tmp_317 })
                },
                {
                    let mut __tmp_318: () = 3_();
                    let mut __tmp_319: () = 1_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_318 }, { &mut __tmp_319 })
                },
                {
                    let mut __tmp_320: () = 3_();
                    let mut __tmp_321: () = 2_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_320 }, { &mut __tmp_321 })
                },
                {
                    let mut __tmp_322: () = 3_();
                    let mut __tmp_323: () = 3_();
                    std_pair_int__Counter_int__::new({ &mut __tmp_322 }, { &mut __tmp_323 })
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
                let mut __tmp_324: () = 1_();
                let mut __tmp_325: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_324 }, { &mut __tmp_325 })
            },
            {
                let mut __tmp_326: () = 1_();
                let mut __tmp_327: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_326 }, { &mut __tmp_327 })
            },
            {
                let mut __tmp_328: () = 1_();
                let mut __tmp_329: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_328 }, { &mut __tmp_329 })
            },
            {
                let mut __tmp_330: () = 2_();
                let mut __tmp_331: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_330 }, { &mut __tmp_331 })
            },
            {
                let mut __tmp_332: () = 2_();
                let mut __tmp_333: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_332 }, { &mut __tmp_333 })
            },
            {
                let mut __tmp_334: () = 2_();
                let mut __tmp_335: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_334 }, { &mut __tmp_335 })
            },
            {
                let mut __tmp_336: () = 3_();
                let mut __tmp_337: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_336 }, { &mut __tmp_337 })
            },
            {
                let mut __tmp_338: () = 3_();
                let mut __tmp_339: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_338 }, { &mut __tmp_339 })
            },
            {
                let mut __tmp_340: () = 3_();
                let mut __tmp_341: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_340 }, { &mut __tmp_341 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [(); 9] = [
            {
                let mut __tmp_342: () = 1_();
                let mut __tmp_343: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_342 }, { &mut __tmp_343 })
            },
            {
                let mut __tmp_344: () = 1_();
                let mut __tmp_345: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_344 }, { &mut __tmp_345 })
            },
            {
                let mut __tmp_346: () = 1_();
                let mut __tmp_347: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_346 }, { &mut __tmp_347 })
            },
            {
                let mut __tmp_348: () = 2_();
                let mut __tmp_349: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_348 }, { &mut __tmp_349 })
            },
            {
                let mut __tmp_350: () = 2_();
                let mut __tmp_351: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_350 }, { &mut __tmp_351 })
            },
            {
                let mut __tmp_352: () = 2_();
                let mut __tmp_353: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_352 }, { &mut __tmp_353 })
            },
            {
                let mut __tmp_354: () = 3_();
                let mut __tmp_355: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_354 }, { &mut __tmp_355 })
            },
            {
                let mut __tmp_356: () = 3_();
                let mut __tmp_357: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_356 }, { &mut __tmp_357 })
            },
            {
                let mut __tmp_358: () = 3_();
                let mut __tmp_359: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_358 }, { &mut __tmp_359 })
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
                let mut __tmp_360: () = 1_();
                let mut __tmp_361: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_360 }, { &mut __tmp_361 })
            },
            {
                let mut __tmp_362: () = 1_();
                let mut __tmp_363: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_362 }, { &mut __tmp_363 })
            },
            {
                let mut __tmp_364: () = 1_();
                let mut __tmp_365: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_364 }, { &mut __tmp_365 })
            },
            {
                let mut __tmp_366: () = 2_();
                let mut __tmp_367: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_366 }, { &mut __tmp_367 })
            },
            {
                let mut __tmp_368: () = 2_();
                let mut __tmp_369: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_368 }, { &mut __tmp_369 })
            },
            {
                let mut __tmp_370: () = 2_();
                let mut __tmp_371: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_370 }, { &mut __tmp_371 })
            },
            {
                let mut __tmp_372: () = 3_();
                let mut __tmp_373: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_372 }, { &mut __tmp_373 })
            },
            {
                let mut __tmp_374: () = 3_();
                let mut __tmp_375: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_374 }, { &mut __tmp_375 })
            },
            {
                let mut __tmp_376: () = 3_();
                let mut __tmp_377: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_376 }, { &mut __tmp_377 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut a2: [(); 9] = [
            {
                let mut __tmp_378: () = 1_();
                let mut __tmp_379: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_378 }, { &mut __tmp_379 })
            },
            {
                let mut __tmp_380: () = 1_();
                let mut __tmp_381: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_380 }, { &mut __tmp_381 })
            },
            {
                let mut __tmp_382: () = 1_();
                let mut __tmp_383: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_382 }, { &mut __tmp_383 })
            },
            {
                let mut __tmp_384: () = 2_();
                let mut __tmp_385: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_384 }, { &mut __tmp_385 })
            },
            {
                let mut __tmp_386: () = 2_();
                let mut __tmp_387: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_386 }, { &mut __tmp_387 })
            },
            {
                let mut __tmp_388: () = 2_();
                let mut __tmp_389: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_388 }, { &mut __tmp_389 })
            },
            {
                let mut __tmp_390: () = 3_();
                let mut __tmp_391: () = 1_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_390 }, { &mut __tmp_391 })
            },
            {
                let mut __tmp_392: () = 3_();
                let mut __tmp_393: () = 2_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_392 }, { &mut __tmp_393 })
            },
            {
                let mut __tmp_394: () = 3_();
                let mut __tmp_395: () = 3_();
                std_pair_CopyConstructible__MoveOnly_::new({ &mut __tmp_394 }, { &mut __tmp_395 })
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
    (unsafe { test_move_alloc_46() });
    (unsafe { test_move_alloc_47() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_23);
    std::cell::LazyCell::force(&*&raw const moved_value_24);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_32);
    std::cell::LazyCell::force(&*&raw const alloc_count_33);
    std::cell::LazyCell::force(&*&raw const dealloc_count_34);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_35);
    std::cell::LazyCell::force(&*&raw const gConstructed_45);
}
