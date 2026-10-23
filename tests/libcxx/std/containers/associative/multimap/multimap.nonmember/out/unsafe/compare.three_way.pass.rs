extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub static mut destructed_value_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_2(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_3(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_2(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
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
            operator_eq_2(
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
pub struct test_allocator_std_pair_const_int__int___rebind_std_pair_const_int__int__ {}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_std_pair_const_int__int__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__int__ {}
impl Default for test_allocator_std_pair_const_int__int__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_int__int__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__StrongOrder___rebind_std_pair_const_int__StrongOrder__
{}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_std_pair_const_int__StrongOrder__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__StrongOrder__ {}
impl Default for test_allocator_std_pair_const_int__StrongOrder__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_int__StrongOrder__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__WeakOrder___rebind_std_pair_const_int__WeakOrder__ {
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_std_pair_const_int__WeakOrder__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__WeakOrder__ {}
impl Default for test_allocator_std_pair_const_int__WeakOrder__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_int__WeakOrder__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__PartialOrder___rebind_std_pair_const_int__PartialOrder__
{}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_std_pair_const_int__PartialOrder__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__PartialOrder__ {}
impl Default for test_allocator_std_pair_const_int__PartialOrder__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_int__PartialOrder__ {
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
pub unsafe fn testComparisonsComplete_4(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_5(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_6(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_7(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_8(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_9(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_10(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_11(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testComparisonsComplete_12(
    t1: *const (),
    t2: *const (),
    mut isEqual: (),
    mut isLess: (),
    mut isGreater: (),
) -> () {
    unimplemented!();
    if isEqual {
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isLess {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
    } else if isGreater {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    } else {
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if !(!(unimplemented!())) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
        if (unimplemented!()) {
            return false;
        }
    }
    return true;
}
pub unsafe fn testOrder_13(t1: *const (), t2: *const (), mut order: ()) -> () {
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const () = t1;
            let _t2: *const () = t2;
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_4(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_14(t1: *const (), t2: *const (), mut order: ()) -> () {
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const () = t1;
            let _t2: *const () = t2;
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_5(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_15(t1: *const (), t2: *const (), mut order: ()) -> () {
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const () = t1;
            let _t2: *const () = t2;
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_6(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_16(t1: *const (), t2: *const (), mut order: ()) -> () {
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const () = t1;
            let _t2: *const () = t2;
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_7(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_17(t1: *const (), t2: *const (), mut order: ()) -> () {
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const () = t1;
            let _t2: *const () = t2;
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_8(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_18(t1: *const (), t2: *const (), mut order: ()) -> () {
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const () = t1;
            let _t2: *const () = t2;
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_9(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_19(t1: *const (), t2: *const (), mut order: ()) -> () {
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const () = t1;
            let _t2: *const () = t2;
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_10(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_20(t1: *const (), t2: *const (), mut order: ()) -> () {
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const () = t1;
            let _t2: *const () = t2;
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_11(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_21(t1: *const (), t2: *const (), mut order: ()) -> () {
    let mut equal: () = unimplemented!();
    let mut less: () = unimplemented!();
    let mut greater: () = unimplemented!();
    return (unimplemented!())
        && (unsafe {
            let _t1: *const () = t1;
            let _t2: *const () = t2;
            let _isEqual: () = equal;
            let _isLess: () = less;
            let _isGreater: () = greater;
            testComparisonsComplete_12(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn operator_lt_22(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
    return (((*lhs).value) < ((*rhs).value));
}
pub unsafe fn operator_eq_23(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
    return (((*lhs).value) == ((*rhs).value));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct LessAndEqComp {
    pub value: (),
}
impl LessAndEqComp {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for LessAndEqComp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_22(self as *const LessAndEqComp, other as *const LessAndEqComp) {
                std::cmp::Ordering::Less
            } else if operator_lt_22(other as *const LessAndEqComp, self as *const LessAndEqComp) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for LessAndEqComp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for LessAndEqComp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_23(self as *const LessAndEqComp, other as *const LessAndEqComp) }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub unsafe fn operator_cmp_24(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&(_a0.value), &(_a1.value));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StrongOrder {
    pub value: (),
}
impl StrongOrder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for StrongOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_24(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_24(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub unsafe fn operator_cmp_25(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let mut cmp: () = unimplemented!();
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WeakOrder {
    pub value: (),
}
impl WeakOrder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for WeakOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_25(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_25(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub unsafe fn operator_cmp_26(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if ((lhs.value) == (unsafe { std_numeric_limits_int_::min() }))
        || ((rhs.value) == (unsafe { std_numeric_limits_int_::min() }))
    {
        return unimplemented!();
    }
    if ((lhs.value) == (unsafe { std_numeric_limits_int_::max() }))
        || ((rhs.value) == (unsafe { std_numeric_limits_int_::max() }))
    {
        return unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn operator_eq_27(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PartialOrder {
    pub value: (),
}
impl PartialOrder {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for PartialOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_26(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_27(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for PartialOrder {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonComparable {}
const _: () = assert!(!(false), "!std::equality_comparable<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorNotEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorSpaceship<NonComparable>");
pub unsafe fn operator_eq_28(_a0: *const EqualityComparable, _a1: *const EqualityComparable) -> () {
    return (((*_a0).value_) == ((*_a1).value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct EqualityComparable {
    value_: (),
}
impl EqualityComparable {
    pub unsafe fn new(mut value: ()) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::PartialEq for EqualityComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_28(
                self as *const EqualityComparable,
                other as *const EqualityComparable,
            )
        }
    }
}
impl std::cmp::Eq for EqualityComparable {}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub unsafe fn operator_eq_29(_a0: *const ThreeWayComparable, _a1: *const ThreeWayComparable) -> () {
    return (((*_a0).value_) == ((*_a1).value_));
}
pub unsafe fn operator_cmp_30(
    _a0: *const ThreeWayComparable,
    _a1: *const ThreeWayComparable,
) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).value_), &((*_a1).value_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ThreeWayComparable {
    value_: (),
}
impl ThreeWayComparable {
    pub unsafe fn new(mut value: ()) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::Ord for ThreeWayComparable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_30(
                self as *const ThreeWayComparable,
                other as *const ThreeWayComparable,
            )
        }
    }
}
impl std::cmp::PartialOrd for ThreeWayComparable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for ThreeWayComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_29(
                self as *const ThreeWayComparable,
                other as *const ThreeWayComparable,
            )
        }
    }
}
impl std::cmp::Eq for ThreeWayComparable {}
const _: () = assert!(true, "std::equality_comparable<ThreeWayComparable>");
const _: () = assert!(true, "std::three_way_comparable<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorSpaceship<ThreeWayComparable>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E1 {
    pub x: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E2 {
    pub y: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool {
    pub val: (),
}
impl ImplicitBool {
    pub unsafe fn to__1(&self) -> () {
        return self.val;
    }
}
pub unsafe fn operator_eq_31(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: ((lhs.x) == (rhs.y)),
    };
}
pub unsafe fn test_ordered_map_container_spaceship_with_type_32(mut comp: ()) {
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {}
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_ordered_map_container_spaceship_with_type_33(mut comp: ()) {
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {}
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_ordered_map_container_spaceship_with_type_34(mut comp: ()) {
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {}
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_ordered_map_container_spaceship_with_type_35(mut comp: ()) {
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {}
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_ordered_map_container_spaceship_with_type_36(mut comp: ()) {
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {}
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_ordered_map_container_spaceship_with_type_37(mut comp: ()) {
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {}
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_ordered_map_container_spaceship_with_type_38(mut comp: ()) {
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_ordered_map_container_spaceship_with_type_39(mut comp: ()) {
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_ordered_map_container_spaceship_with_type_40(mut comp: ()) {
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
    }
    {}
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        unimplemented!();
        let mut l3: () = unimplemented!();
        let mut l4: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_ordered_map_container_spaceship_41() -> () {
    const _: () = assert!(
        !(false),
        "!std::three_way_comparable<Container<int, NonComparable>>"
    );;
    const _: () = assert!(true, "std::three_way_comparable<Container<int, int>>");;
    (unsafe { test_ordered_map_container_spaceship_with_type_32(unimplemented!()) });
    (unsafe { test_ordered_map_container_spaceship_with_type_33(unimplemented!()) });
    (unsafe { test_ordered_map_container_spaceship_with_type_34(unimplemented!()) });
    (unsafe { test_ordered_map_container_spaceship_with_type_35(unimplemented!()) });
    (unsafe { test_ordered_map_container_spaceship_with_type_36(unimplemented!()) });
    (unsafe { test_ordered_map_container_spaceship_with_type_37(unimplemented!()) });
    (unsafe { test_ordered_map_container_spaceship_with_type_38(unimplemented!()) });
    (unsafe { test_ordered_map_container_spaceship_with_type_39(unimplemented!()) });
    (unsafe { test_ordered_map_container_spaceship_with_type_40(unimplemented!()) });
    return true;
}
pub unsafe fn test_42() -> () {
    unimplemented!();
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
    (unsafe { test_42() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
}
