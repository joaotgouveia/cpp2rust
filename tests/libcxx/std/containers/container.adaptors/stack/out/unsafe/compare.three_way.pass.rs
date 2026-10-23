extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct nasty_vector_int_ {
    pub v_: (),
}
impl nasty_vector_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            v_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut il: ()) -> Self {
        let mut this = Self {
            v_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl std::cmp::Ord for nasty_vector_int_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_0(
                self as *const nasty_vector_int_,
                other as *const nasty_vector_int_,
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_vector_int_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_vector_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_1(
                self as *const nasty_vector_int_,
                other as *const nasty_vector_int_,
            )
        }
    }
}
impl std::cmp::Eq for nasty_vector_int_ {}
impl Default for nasty_vector_int_ {
    fn default() -> Self {
        unsafe { nasty_vector_int_::new_1() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct nasty_vector_StrongOrder_ {
    pub v_: (),
}
impl nasty_vector_StrongOrder_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            v_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut il: ()) -> Self {
        let mut this = Self {
            v_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl std::cmp::Ord for nasty_vector_StrongOrder_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_2(
                self as *const nasty_vector_StrongOrder_,
                other as *const nasty_vector_StrongOrder_,
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_vector_StrongOrder_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_vector_StrongOrder_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_3(
                self as *const nasty_vector_StrongOrder_,
                other as *const nasty_vector_StrongOrder_,
            )
        }
    }
}
impl std::cmp::Eq for nasty_vector_StrongOrder_ {}
impl Default for nasty_vector_StrongOrder_ {
    fn default() -> Self {
        unsafe { nasty_vector_StrongOrder_::new_1() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct nasty_vector_WeakOrder_ {
    pub v_: (),
}
impl nasty_vector_WeakOrder_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            v_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut il: ()) -> Self {
        let mut this = Self {
            v_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl std::cmp::Ord for nasty_vector_WeakOrder_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_4(
                self as *const nasty_vector_WeakOrder_,
                other as *const nasty_vector_WeakOrder_,
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_vector_WeakOrder_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_vector_WeakOrder_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
                self as *const nasty_vector_WeakOrder_,
                other as *const nasty_vector_WeakOrder_,
            )
        }
    }
}
impl std::cmp::Eq for nasty_vector_WeakOrder_ {}
impl Default for nasty_vector_WeakOrder_ {
    fn default() -> Self {
        unsafe { nasty_vector_WeakOrder_::new_1() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct nasty_vector_PartialOrder_ {
    pub v_: (),
}
impl nasty_vector_PartialOrder_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            v_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut il: ()) -> Self {
        let mut this = Self {
            v_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl std::cmp::Ord for nasty_vector_PartialOrder_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_6(
                self as *const nasty_vector_PartialOrder_,
                other as *const nasty_vector_PartialOrder_,
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_vector_PartialOrder_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_vector_PartialOrder_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_7(
                self as *const nasty_vector_PartialOrder_,
                other as *const nasty_vector_PartialOrder_,
            )
        }
    }
}
impl std::cmp::Eq for nasty_vector_PartialOrder_ {}
impl Default for nasty_vector_PartialOrder_ {
    fn default() -> Self {
        unsafe { nasty_vector_PartialOrder_::new_1() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct nasty_vector_LessAndEqComp_ {
    pub v_: (),
}
impl nasty_vector_LessAndEqComp_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            v_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut il: ()) -> Self {
        let mut this = Self {
            v_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl std::cmp::Ord for nasty_vector_LessAndEqComp_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_8(
                self as *const nasty_vector_LessAndEqComp_,
                other as *const nasty_vector_LessAndEqComp_,
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_vector_LessAndEqComp_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_vector_LessAndEqComp_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_9(
                self as *const nasty_vector_LessAndEqComp_,
                other as *const nasty_vector_LessAndEqComp_,
            )
        }
    }
}
impl std::cmp::Eq for nasty_vector_LessAndEqComp_ {}
impl Default for nasty_vector_LessAndEqComp_ {
    fn default() -> Self {
        unsafe { nasty_vector_LessAndEqComp_::new_1() }
    }
}
pub unsafe fn operator_eq_1(x: *const nasty_vector_int_, y: *const nasty_vector_int_) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_3(
    x: *const nasty_vector_StrongOrder_,
    y: *const nasty_vector_StrongOrder_,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_5(
    x: *const nasty_vector_WeakOrder_,
    y: *const nasty_vector_WeakOrder_,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_7(
    x: *const nasty_vector_PartialOrder_,
    y: *const nasty_vector_PartialOrder_,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_9(
    x: *const nasty_vector_LessAndEqComp_,
    y: *const nasty_vector_LessAndEqComp_,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_cmp_0(
    x: *const nasty_vector_int_,
    y: *const nasty_vector_int_,
) -> std_strong_ordering {
    return unimplemented!();
}
pub unsafe fn operator_cmp_2(
    x: *const nasty_vector_StrongOrder_,
    y: *const nasty_vector_StrongOrder_,
) -> std_strong_ordering {
    return unimplemented!();
}
pub unsafe fn operator_cmp_4(
    x: *const nasty_vector_WeakOrder_,
    y: *const nasty_vector_WeakOrder_,
) -> std_weak_ordering {
    return unimplemented!();
}
pub unsafe fn operator_cmp_6(
    x: *const nasty_vector_PartialOrder_,
    y: *const nasty_vector_PartialOrder_,
) -> std_partial_ordering {
    return unimplemented!();
}
pub unsafe fn operator_cmp_8(
    x: *const nasty_vector_LessAndEqComp_,
    y: *const nasty_vector_LessAndEqComp_,
) -> std_weak_ordering {
    return unimplemented!();
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct nasty_list_int_ {
    pub l_: (),
}
impl nasty_list_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            l_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut il: ()) -> Self {
        let mut this = Self {
            l_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl std::cmp::Ord for nasty_list_int_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_10(
                self as *const nasty_list_int_,
                other as *const nasty_list_int_,
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_list_int_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_list_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_11(
                self as *const nasty_list_int_,
                other as *const nasty_list_int_,
            )
        }
    }
}
impl std::cmp::Eq for nasty_list_int_ {}
impl Default for nasty_list_int_ {
    fn default() -> Self {
        unsafe { nasty_list_int_::new_1() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct nasty_list_StrongOrder_ {
    pub l_: (),
}
impl nasty_list_StrongOrder_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            l_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut il: ()) -> Self {
        let mut this = Self {
            l_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl std::cmp::Ord for nasty_list_StrongOrder_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_12(
                self as *const nasty_list_StrongOrder_,
                other as *const nasty_list_StrongOrder_,
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_list_StrongOrder_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_list_StrongOrder_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_13(
                self as *const nasty_list_StrongOrder_,
                other as *const nasty_list_StrongOrder_,
            )
        }
    }
}
impl std::cmp::Eq for nasty_list_StrongOrder_ {}
impl Default for nasty_list_StrongOrder_ {
    fn default() -> Self {
        unsafe { nasty_list_StrongOrder_::new_1() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct nasty_list_WeakOrder_ {
    pub l_: (),
}
impl nasty_list_WeakOrder_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            l_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut il: ()) -> Self {
        let mut this = Self {
            l_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl std::cmp::Ord for nasty_list_WeakOrder_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_14(
                self as *const nasty_list_WeakOrder_,
                other as *const nasty_list_WeakOrder_,
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_list_WeakOrder_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_list_WeakOrder_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_15(
                self as *const nasty_list_WeakOrder_,
                other as *const nasty_list_WeakOrder_,
            )
        }
    }
}
impl std::cmp::Eq for nasty_list_WeakOrder_ {}
impl Default for nasty_list_WeakOrder_ {
    fn default() -> Self {
        unsafe { nasty_list_WeakOrder_::new_1() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct nasty_list_PartialOrder_ {
    pub l_: (),
}
impl nasty_list_PartialOrder_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            l_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut il: ()) -> Self {
        let mut this = Self {
            l_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl std::cmp::Ord for nasty_list_PartialOrder_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_16(
                self as *const nasty_list_PartialOrder_,
                other as *const nasty_list_PartialOrder_,
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_list_PartialOrder_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_list_PartialOrder_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_17(
                self as *const nasty_list_PartialOrder_,
                other as *const nasty_list_PartialOrder_,
            )
        }
    }
}
impl std::cmp::Eq for nasty_list_PartialOrder_ {}
impl Default for nasty_list_PartialOrder_ {
    fn default() -> Self {
        unsafe { nasty_list_PartialOrder_::new_1() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct nasty_list_LessAndEqComp_ {
    pub l_: (),
}
impl nasty_list_LessAndEqComp_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            l_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(mut il: ()) -> Self {
        let mut this = Self {
            l_: unimplemented!(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
impl std::cmp::Ord for nasty_list_LessAndEqComp_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_18(
                self as *const nasty_list_LessAndEqComp_,
                other as *const nasty_list_LessAndEqComp_,
            )
        }
    }
}
impl std::cmp::PartialOrd for nasty_list_LessAndEqComp_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for nasty_list_LessAndEqComp_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_19(
                self as *const nasty_list_LessAndEqComp_,
                other as *const nasty_list_LessAndEqComp_,
            )
        }
    }
}
impl std::cmp::Eq for nasty_list_LessAndEqComp_ {}
impl Default for nasty_list_LessAndEqComp_ {
    fn default() -> Self {
        unsafe { nasty_list_LessAndEqComp_::new_1() }
    }
}
pub unsafe fn operator_eq_11(x: *const nasty_list_int_, y: *const nasty_list_int_) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_13(
    x: *const nasty_list_StrongOrder_,
    y: *const nasty_list_StrongOrder_,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_15(
    x: *const nasty_list_WeakOrder_,
    y: *const nasty_list_WeakOrder_,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_17(
    x: *const nasty_list_PartialOrder_,
    y: *const nasty_list_PartialOrder_,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_eq_19(
    x: *const nasty_list_LessAndEqComp_,
    y: *const nasty_list_LessAndEqComp_,
) -> () {
    return unimplemented!();
}
pub unsafe fn operator_cmp_10(
    x: *const nasty_list_int_,
    y: *const nasty_list_int_,
) -> std_strong_ordering {
    return unimplemented!();
}
pub unsafe fn operator_cmp_12(
    x: *const nasty_list_StrongOrder_,
    y: *const nasty_list_StrongOrder_,
) -> std_strong_ordering {
    return unimplemented!();
}
pub unsafe fn operator_cmp_14(
    x: *const nasty_list_WeakOrder_,
    y: *const nasty_list_WeakOrder_,
) -> std_weak_ordering {
    return unimplemented!();
}
pub unsafe fn operator_cmp_16(
    x: *const nasty_list_PartialOrder_,
    y: *const nasty_list_PartialOrder_,
) -> std_partial_ordering {
    return unimplemented!();
}
pub unsafe fn operator_cmp_18(
    x: *const nasty_list_LessAndEqComp_,
    y: *const nasty_list_LessAndEqComp_,
) -> std_weak_ordering {
    return unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct nasty_mutex {}
impl nasty_mutex {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn destructor(&mut self) {}
    pub unsafe fn operator_addr(&mut self) -> *mut nasty_mutex {
        unimplemented!();
        return std::ptr::null_mut();
    }
    unsafe fn copy_from(_a0: *const nasty_mutex) -> Self {
        let mut this = Self {};
        unimplemented!();
        this
    }
    unsafe fn copy_assign(&mut self, _a0: *const nasty_mutex) -> *mut nasty_mutex {
        unimplemented!();
        return &mut (*(self as *mut nasty_mutex));
    }
    pub unsafe fn lock(&mut self) {}
    pub unsafe fn try_lock(&mut self) -> () {
        return true;
    }
    pub unsafe fn unlock(&mut self) {}
    pub unsafe fn lock_shared(&mut self) {}
    pub unsafe fn try_lock_shared(&mut self) -> () {
        return true;
    }
    pub unsafe fn unlock_shared(&mut self) {}
}
impl Clone for nasty_mutex {
    fn clone(&self) -> Self {
        unsafe { nasty_mutex::copy_from(self as *const nasty_mutex) }
    }
}
impl Default for nasty_mutex {
    fn default() -> Self {
        unsafe { nasty_mutex::new() }
    }
}
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
pub static mut destructed_value_20: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_21: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_22(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_23(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_22(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_20));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_20));
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
            operator_eq_22(
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
pub unsafe fn testComparisonsComplete_24(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_25(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_26(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_27(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_28(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_29(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_30(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_31(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_32(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_33(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_34(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_35(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_36(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_37(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_38(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_39(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_40(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_41(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_42(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_43(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_44(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_45(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_46(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_47(
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
        if !(unimplemented!()) {
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
pub unsafe fn testComparisonsComplete_48(
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
        if !(unimplemented!()) {
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
pub unsafe fn testOrder_49(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_24(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_50(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_25(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_51(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_26(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_52(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_27(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_53(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_28(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_54(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_29(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_55(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_30(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_56(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_31(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_57(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_32(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_58(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_33(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_59(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_34(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_60(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_35(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_61(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_36(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_62(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_37(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_63(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_38(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_64(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_39(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_65(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_40(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_66(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_41(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_67(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_42(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_68(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_43(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_69(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_44(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_70(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_45(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_71(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_46(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_72(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_47(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn testOrder_73(t1: *const (), t2: *const (), mut order: ()) -> () {
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
            testComparisonsComplete_48(_t1, _t2, _isEqual, _isLess, _isGreater)
        });
}
pub unsafe fn operator_lt_74(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
    return (((*lhs).value) < ((*rhs).value));
}
pub unsafe fn operator_eq_75(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> () {
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
            if operator_lt_74(self as *const LessAndEqComp, other as *const LessAndEqComp) {
                std::cmp::Ordering::Less
            } else if operator_lt_74(other as *const LessAndEqComp, self as *const LessAndEqComp) {
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
        unsafe { operator_eq_75(self as *const LessAndEqComp, other as *const LessAndEqComp) }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub unsafe fn operator_cmp_76(mut _a0: StrongOrder, mut _a1: StrongOrder) -> () {
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
        unsafe { operator_cmp_76(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_76(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub unsafe fn operator_cmp_77(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
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
        unsafe { operator_cmp_77(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_77(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub unsafe fn operator_cmp_78(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if ((lhs.value) == (unimplemented!())) || ((rhs.value) == (unimplemented!())) {
        return unimplemented!();
    }
    if ((lhs.value) == (unimplemented!())) || ((rhs.value) == (unimplemented!())) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn operator_eq_79(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
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
        unsafe { operator_cmp_78(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_79(self.clone(), other.clone()) }
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
pub unsafe fn operator_eq_80(_a0: *const EqualityComparable, _a1: *const EqualityComparable) -> () {
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
            operator_eq_80(
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
pub unsafe fn operator_eq_81(_a0: *const ThreeWayComparable, _a1: *const ThreeWayComparable) -> () {
    return (((*_a0).value_) == ((*_a1).value_));
}
pub unsafe fn operator_cmp_82(
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
            operator_cmp_82(
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
            operator_eq_81(
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
pub unsafe fn operator_eq_83(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: ((lhs.x) == (rhs.y)),
    };
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_84() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_85() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_86() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_87() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_88() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_89() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_90() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_91() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_92() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_93() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_94() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_95() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_96() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_97() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_98() {
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: () = unimplemented!();
        let mut ca1: () = unimplemented!();
        let mut l2: () = unimplemented!();
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_99() {
    {
        let mut l1: nasty_list_int_ = nasty_list_int_::new_1();
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_int_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_int_ = nasty_list_int_::new_1();
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_int_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_int_ = nasty_list_int_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_int_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_int_ = nasty_list_int_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_int_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_int_ = nasty_list_int_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_int_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_int_ = nasty_list_int_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_int_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_int_ = nasty_list_int_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_int_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_int_ = nasty_list_int_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_int_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_int_ = nasty_list_int_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_int_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_int_ = nasty_list_int_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_int_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_int_ = nasty_list_int_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_int_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_int_ = nasty_list_int_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_int_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_100() {
    {
        let mut l1: nasty_list_StrongOrder_ = nasty_list_StrongOrder_::new_1();
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_StrongOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_StrongOrder_ = nasty_list_StrongOrder_::new_1();
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_StrongOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_StrongOrder_ = nasty_list_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_StrongOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_StrongOrder_ = nasty_list_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_StrongOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_StrongOrder_ = nasty_list_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_StrongOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_StrongOrder_ = nasty_list_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_StrongOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_StrongOrder_ = nasty_list_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_StrongOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_StrongOrder_ = nasty_list_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_StrongOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_StrongOrder_ = nasty_list_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_StrongOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_StrongOrder_ = nasty_list_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_StrongOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_StrongOrder_ = nasty_list_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_StrongOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_StrongOrder_ = nasty_list_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_StrongOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_101() {
    {
        let mut l1: nasty_list_WeakOrder_ = nasty_list_WeakOrder_::new_1();
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_WeakOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_WeakOrder_ = nasty_list_WeakOrder_::new_1();
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_WeakOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_WeakOrder_ = nasty_list_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_WeakOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_WeakOrder_ = nasty_list_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_WeakOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_WeakOrder_ = nasty_list_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_WeakOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_WeakOrder_ = nasty_list_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_WeakOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_WeakOrder_ = nasty_list_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_WeakOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_WeakOrder_ = nasty_list_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_WeakOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_WeakOrder_ = nasty_list_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_WeakOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_WeakOrder_ = nasty_list_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_WeakOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_WeakOrder_ = nasty_list_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_WeakOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_WeakOrder_ = nasty_list_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_WeakOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_102() {
    {
        let mut l1: nasty_list_PartialOrder_ = nasty_list_PartialOrder_::new_1();
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_PartialOrder_ = nasty_list_PartialOrder_::new_1();
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_PartialOrder_ =
            nasty_list_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_PartialOrder_ =
            nasty_list_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_PartialOrder_ =
            nasty_list_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_PartialOrder_ =
            nasty_list_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_PartialOrder_ =
            nasty_list_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_PartialOrder_ =
            nasty_list_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_PartialOrder_ =
            nasty_list_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_PartialOrder_ =
            nasty_list_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_PartialOrder_ =
            nasty_list_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_PartialOrder_ =
            nasty_list_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_PartialOrder_ =
            nasty_list_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_PartialOrder_ =
            nasty_list_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_103() {
    {
        let mut l1: nasty_list_LessAndEqComp_ = nasty_list_LessAndEqComp_::new_1();
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_LessAndEqComp_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_LessAndEqComp_ = nasty_list_LessAndEqComp_::new_1();
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_LessAndEqComp_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_LessAndEqComp_ =
            nasty_list_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_LessAndEqComp_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_LessAndEqComp_ =
            nasty_list_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_LessAndEqComp_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_LessAndEqComp_ =
            nasty_list_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_LessAndEqComp_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_LessAndEqComp_ =
            nasty_list_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_LessAndEqComp_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_LessAndEqComp_ =
            nasty_list_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_LessAndEqComp_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_LessAndEqComp_ =
            nasty_list_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_LessAndEqComp_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_LessAndEqComp_ =
            nasty_list_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_LessAndEqComp_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_LessAndEqComp_ =
            nasty_list_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_LessAndEqComp_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_list_LessAndEqComp_ =
            nasty_list_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_list_LessAndEqComp_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_list_LessAndEqComp_ =
            nasty_list_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_list_LessAndEqComp_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_104() {
    {
        let mut l1: nasty_vector_int_ = nasty_vector_int_::new_1();
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_int_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_int_ = nasty_vector_int_::new_1();
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_int_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_int_ = nasty_vector_int_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_int_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_int_ = nasty_vector_int_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_int_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_int_ = nasty_vector_int_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_int_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_int_ = nasty_vector_int_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_int_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_int_ = nasty_vector_int_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_int_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_int_ = nasty_vector_int_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_int_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_int_ = nasty_vector_int_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_int_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_int_ = nasty_vector_int_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_int_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_int_ = nasty_vector_int_::new_2({ unimplemented!() });
        let _dtor_l1 = ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_int_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_int_ = nasty_vector_int_::new_2({ unimplemented!() });
        let _dtor_l2 = ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_int_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_105() {
    {
        let mut l1: nasty_vector_StrongOrder_ = nasty_vector_StrongOrder_::new_1();
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_StrongOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_StrongOrder_ = nasty_vector_StrongOrder_::new_1();
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_StrongOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_StrongOrder_ =
            nasty_vector_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_StrongOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_StrongOrder_ =
            nasty_vector_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_StrongOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_StrongOrder_ =
            nasty_vector_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_StrongOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_StrongOrder_ =
            nasty_vector_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_StrongOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_StrongOrder_ =
            nasty_vector_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_StrongOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_StrongOrder_ =
            nasty_vector_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_StrongOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_StrongOrder_ =
            nasty_vector_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_StrongOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_StrongOrder_ =
            nasty_vector_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_StrongOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_StrongOrder_ =
            nasty_vector_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_StrongOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_StrongOrder_ =
            nasty_vector_StrongOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_StrongOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_106() {
    {
        let mut l1: nasty_vector_WeakOrder_ = nasty_vector_WeakOrder_::new_1();
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_WeakOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_WeakOrder_ = nasty_vector_WeakOrder_::new_1();
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_WeakOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_WeakOrder_ = nasty_vector_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_WeakOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_WeakOrder_ = nasty_vector_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_WeakOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_WeakOrder_ = nasty_vector_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_WeakOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_WeakOrder_ = nasty_vector_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_WeakOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_WeakOrder_ = nasty_vector_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_WeakOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_WeakOrder_ = nasty_vector_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_WeakOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_WeakOrder_ = nasty_vector_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_WeakOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_WeakOrder_ = nasty_vector_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_WeakOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_WeakOrder_ = nasty_vector_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_WeakOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_WeakOrder_ = nasty_vector_WeakOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_WeakOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_107() {
    {
        let mut l1: nasty_vector_PartialOrder_ = nasty_vector_PartialOrder_::new_1();
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_PartialOrder_ = nasty_vector_PartialOrder_::new_1();
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_PartialOrder_ =
            nasty_vector_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_PartialOrder_ =
            nasty_vector_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_PartialOrder_ =
            nasty_vector_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_PartialOrder_ =
            nasty_vector_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_PartialOrder_ =
            nasty_vector_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_PartialOrder_ =
            nasty_vector_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_PartialOrder_ =
            nasty_vector_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_PartialOrder_ =
            nasty_vector_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_PartialOrder_ =
            nasty_vector_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_PartialOrder_ =
            nasty_vector_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_PartialOrder_ =
            nasty_vector_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_PartialOrder_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_PartialOrder_ =
            nasty_vector_PartialOrder_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_PartialOrder_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_sequence_container_adaptor_spaceship_with_type_108() {
    {
        let mut l1: nasty_vector_LessAndEqComp_ = nasty_vector_LessAndEqComp_::new_1();
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_LessAndEqComp_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_LessAndEqComp_ = nasty_vector_LessAndEqComp_::new_1();
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_LessAndEqComp_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_LessAndEqComp_ =
            nasty_vector_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_LessAndEqComp_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_LessAndEqComp_ =
            nasty_vector_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_LessAndEqComp_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_LessAndEqComp_ =
            nasty_vector_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_LessAndEqComp_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_LessAndEqComp_ =
            nasty_vector_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_LessAndEqComp_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_LessAndEqComp_ =
            nasty_vector_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_LessAndEqComp_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_LessAndEqComp_ =
            nasty_vector_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_LessAndEqComp_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_LessAndEqComp_ =
            nasty_vector_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_LessAndEqComp_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_LessAndEqComp_ =
            nasty_vector_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_LessAndEqComp_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut l1: nasty_vector_LessAndEqComp_ =
            nasty_vector_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l1 =
            ScopedDestructorUnsafe::new(&raw mut l1, nasty_vector_LessAndEqComp_::destructor);
        let mut ca1: () = unimplemented!();
        let mut l2: nasty_vector_LessAndEqComp_ =
            nasty_vector_LessAndEqComp_::new_2({ unimplemented!() });
        let _dtor_l2 =
            ScopedDestructorUnsafe::new(&raw mut l2, nasty_vector_LessAndEqComp_::destructor);
        let mut ca2: () = unimplemented!();
        unimplemented!();
    }
    {}
}
pub unsafe fn test_sequence_container_adaptor_spaceship_109() -> () {
    const _: () = assert!(
        !(false),
        "!std::three_way_comparable<ContainerAdaptor<NonComparable>>"
    );;
    const _: () = assert!(
        true,
        "std::three_way_comparable<ContainerAdaptor<int, Container<int>>>"
    );;
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_84() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_85() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_86() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_87() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_88() });
    return true;
}
pub unsafe fn test_sequence_container_adaptor_spaceship_110() -> () {
    const _: () = assert!(
        !(false),
        "!std::three_way_comparable<ContainerAdaptor<NonComparable>>"
    );;
    const _: () = assert!(
        true,
        "std::three_way_comparable<ContainerAdaptor<int, Container<int>>>"
    );;
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_89() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_90() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_91() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_92() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_93() });
    return true;
}
pub unsafe fn test_sequence_container_adaptor_spaceship_111() -> () {
    const _: () = assert!(
        !(false),
        "!std::three_way_comparable<ContainerAdaptor<NonComparable>>"
    );;
    const _: () = assert!(
        true,
        "std::three_way_comparable<ContainerAdaptor<int, Container<int>>>"
    );;
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_94() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_95() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_96() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_97() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_98() });
    return true;
}
pub unsafe fn test_sequence_container_adaptor_spaceship_112() -> () {
    const _: () = assert!(
        !(false),
        "!std::three_way_comparable<ContainerAdaptor<NonComparable>>"
    );;
    const _: () = assert!(
        true,
        "std::three_way_comparable<ContainerAdaptor<int, Container<int>>>"
    );;
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_99() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_100() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_101() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_102() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_103() });
    return true;
}
pub unsafe fn test_sequence_container_adaptor_spaceship_113() -> () {
    const _: () = assert!(
        !(false),
        "!std::three_way_comparable<ContainerAdaptor<NonComparable>>"
    );;
    const _: () = assert!(
        true,
        "std::three_way_comparable<ContainerAdaptor<int, Container<int>>>"
    );;
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_104() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_105() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_106() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_107() });
    (unsafe { test_sequence_container_adaptor_spaceship_with_type_108() });
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_20);
    std::cell::LazyCell::force(&*&raw const moved_value_21);
}
