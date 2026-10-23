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
pub struct bare_allocator_void_ {}
impl bare_allocator_void_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for bare_allocator_void_ {
    fn default() -> Self {
        unsafe { bare_allocator_void_::new() }
    }
}
impl bare_allocator_Zero_ {
    pub unsafe fn new_1(mut _a0: bare_allocator_void_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bare_allocator_Zero_ {}
impl bare_allocator_Zero_ {}
impl bare_allocator_One_ {
    pub unsafe fn new_1(mut _a0: bare_allocator_void_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bare_allocator_One_ {}
impl bare_allocator_One_ {}
impl bare_allocator_Two_ {
    pub unsafe fn new_1(mut _a0: bare_allocator_void_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bare_allocator_Two_ {}
impl bare_allocator_Two_ {}
impl bare_allocator_Three_ {
    pub unsafe fn new_1(mut _a0: bare_allocator_void_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bare_allocator_Three_ {}
impl bare_allocator_Three_ {}
impl bare_allocator_A_ {
    pub unsafe fn new_1(mut _a0: bare_allocator_void_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bare_allocator_A_ {}
impl bare_allocator_A_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut operator_hijacker,
}
impl min_pointer_operator_hijacker__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_A__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut A,
}
impl min_pointer_A__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_void_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_operator_hijacker_ {}
impl min_allocator_A_ {
    pub unsafe fn new_1(mut _a0: min_allocator_void_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_A_ {}
impl min_allocator_A_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct complete_type_allocator_int_ {}
impl complete_type_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for complete_type_allocator_int_ {
    fn default() -> Self {
        unsafe { complete_type_allocator_int_::new() }
    }
}
impl complete_type_allocator_Zero_ {
    pub unsafe fn new_1(mut _a0: complete_type_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct complete_type_allocator_Zero_ {}
impl complete_type_allocator_Zero_ {}
impl complete_type_allocator_One_ {
    pub unsafe fn new_1(mut _a0: complete_type_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct complete_type_allocator_One_ {}
impl complete_type_allocator_One_ {}
impl complete_type_allocator_Two_ {
    pub unsafe fn new_1(mut _a0: complete_type_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct complete_type_allocator_Two_ {}
impl complete_type_allocator_Two_ {}
impl complete_type_allocator_Three_ {
    pub unsafe fn new_1(mut _a0: complete_type_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct complete_type_allocator_Three_ {}
impl complete_type_allocator_Three_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct operator_hijacker {}
impl operator_hijacker {
    pub unsafe fn operator_lt(&self, _a0: *const operator_hijacker) -> () {
        return true;
    }
    pub unsafe fn operator_eq(&self, _a0: *const operator_hijacker) -> () {
        return true;
    }
    pub unsafe fn operator_call(&self) -> () {
        return 42_();
    }
}
impl std::cmp::Ord for operator_hijacker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_hijacker::operator_lt(self, other as *const operator_hijacker) {
                std::cmp::Ordering::Less
            } else if operator_hijacker::operator_lt(other, self as *const operator_hijacker) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for operator_hijacker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for operator_hijacker {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_hijacker::operator_eq(self, other as *const operator_hijacker) }
    }
}
impl std::cmp::Eq for operator_hijacker {}
const _: () = assert!(
    true,
    "std::is_trivially_copyable<operator_hijacker>::value &&     //\n                  std::is_copy_constructible<operator_hijacker>::value && //\n                  std::is_move_constructible<operator_hijacker>::value && //\n                  std::is_copy_assignable<operator_hijacker>::value &&    //\n                  std::is_move_assignable<operator_hijacker>::value"
);
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
pub static mut destructed_value_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_6(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_7(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_6(_x, _y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_void__rebind_Zero_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_void__rebind_One_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_void__rebind_Two_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_void__rebind_Three_ {}
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
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
            operator_eq_6(
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
pub struct test_allocator_A__rebind_A_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_A_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_A_ {
    pub unsafe fn new_1(mut data: (), mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: stats,
        };
        if !((stats).is_null()) {
            (*this.stats_).count.prefix_inc();
        }
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_A_) -> Self {
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
    pub unsafe fn move_from(a: *mut test_allocator_A_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).moved.prefix_inc();
        }
        unimplemented!();
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_5));
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
    }
    pub unsafe fn destroy(&mut self, mut p: *mut A) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        }
        (unsafe { A::destructor(&mut (*p)) });
    }
}
impl Clone for test_allocator_A_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_A_::copy_from(self as *const test_allocator_A_) }
    }
}
impl Default for test_allocator_A_ {
    fn default() -> Self {
        unsafe {
            test_allocator_A_ {
                data_: unimplemented!(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_Zero_ {
    pub unsafe fn new_1(a: *const test_allocator_void_) -> Self {
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
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_Zero_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_Zero_ {
    pub unsafe fn copy_from(a: *const test_allocator_Zero_) -> Self {
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
    pub unsafe fn move_from(a: *mut test_allocator_Zero_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).moved.prefix_inc();
        }
        unimplemented!();
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_5));
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
    }
    pub unsafe fn destroy(&mut self, mut p: *mut Zero) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        }
        (unsafe { Zero::destructor(&mut (*p)) });
    }
}
impl Clone for test_allocator_Zero_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_Zero_::copy_from(self as *const test_allocator_Zero_) }
    }
}
impl Default for test_allocator_Zero_ {
    fn default() -> Self {
        unsafe {
            test_allocator_Zero_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_One_ {
    pub unsafe fn new_1(a: *const test_allocator_void_) -> Self {
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
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_One_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_One_ {
    pub unsafe fn copy_from(a: *const test_allocator_One_) -> Self {
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
    pub unsafe fn move_from(a: *mut test_allocator_One_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).moved.prefix_inc();
        }
        unimplemented!();
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_5));
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
    }
    pub unsafe fn destroy(&mut self, mut p: *mut One) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        }
        (unsafe { One::destructor(&mut (*p)) });
    }
    pub unsafe fn construct_16_const_int_ref(&mut self, mut p: *mut One, val: *const ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        unimplemented!();
    }
}
impl Clone for test_allocator_One_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_One_::copy_from(self as *const test_allocator_One_) }
    }
}
impl Default for test_allocator_One_ {
    fn default() -> Self {
        unsafe {
            test_allocator_One_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_Two_ {
    pub unsafe fn new_1(a: *const test_allocator_void_) -> Self {
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
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_Two_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_Two_ {
    pub unsafe fn copy_from(a: *const test_allocator_Two_) -> Self {
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
    pub unsafe fn move_from(a: *mut test_allocator_Two_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).moved.prefix_inc();
        }
        unimplemented!();
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_5));
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
    }
    pub unsafe fn destroy(&mut self, mut p: *mut Two) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        }
        (unsafe { Two::destructor(&mut (*p)) });
    }
}
impl Clone for test_allocator_Two_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_Two_::copy_from(self as *const test_allocator_Two_) }
    }
}
impl Default for test_allocator_Two_ {
    fn default() -> Self {
        unsafe {
            test_allocator_Two_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_Three_ {
    pub unsafe fn new_1(a: *const test_allocator_void_) -> Self {
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
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_Three_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_Three_ {
    pub unsafe fn copy_from(a: *const test_allocator_Three_) -> Self {
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
    pub unsafe fn move_from(a: *mut test_allocator_Three_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).moved.prefix_inc();
        }
        unimplemented!();
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_5));
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_4));
    }
    pub unsafe fn destroy(&mut self, mut p: *mut Three) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        }
        (unsafe { Three::destructor(&mut (*p)) });
    }
}
impl Clone for test_allocator_Three_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_Three_::copy_from(self as *const test_allocator_Three_) }
    }
}
impl Default for test_allocator_Three_ {
    fn default() -> Self {
        unsafe {
            test_allocator_Three_ {
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
pub static mut new_count_8: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct A {
    int_: (),
    char_: (),
}
impl A {
    pub unsafe fn new(mut i: (), mut c: ()) -> Self {
        let mut this = Self { int_: i, char_: c };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(a: *const A) -> Self {
        let mut this = Self {
            int_: (*a).int_,
            char_: (*a).char_,
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_9)).prefix_dec();
    }
    pub unsafe fn get_int(&self) -> () {
        return self.int_;
    }
    pub unsafe fn get_char(&self) -> () {
        return self.char_;
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const A) -> *mut A {
        self.int_ = (*_a0).int_;
        self.char_ = (*_a0).char_;
        return &mut (*(self as *mut A));
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        unsafe { A::copy_from(self as *const A) }
    }
}
pub static mut count_9: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct Zero {}
impl Zero {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(_a0: *const Zero) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_10)).prefix_dec();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Zero) -> *mut Zero {
        return &mut (*(self as *mut Zero));
    }
}
impl Clone for Zero {
    fn clone(&self) -> Self {
        unsafe { Zero::copy_from(self as *const Zero) }
    }
}
impl Default for Zero {
    fn default() -> Self {
        unsafe { Zero::new() }
    }
}
pub static mut count_10: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct One {
    pub value: (),
}
impl One {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { value: v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_11)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(o: *const One) -> Self {
        let mut this = Self { value: (*o).value };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_11)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_11)).prefix_dec();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const One) -> *mut One {
        self.value = (*_a0).value;
        return &mut (*(self as *mut One));
    }
}
impl Clone for One {
    fn clone(&self) -> Self {
        unsafe { One::copy_from(self as *const One) }
    }
}
pub static mut count_11: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct Two {
    pub value: (),
}
impl Two {
    pub unsafe fn new(mut v: (), mut _a1: ()) -> Self {
        let mut this = Self { value: v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(o: *const Two) -> Self {
        let mut this = Self { value: (*o).value };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_12)).prefix_dec();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Two) -> *mut Two {
        self.value = (*_a0).value;
        return &mut (*(self as *mut Two));
    }
}
impl Clone for Two {
    fn clone(&self) -> Self {
        unsafe { Two::copy_from(self as *const Two) }
    }
}
pub static mut count_12: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct Three {
    pub value: (),
}
impl Three {
    pub unsafe fn new(mut v: (), mut _a1: (), mut _a2: ()) -> Self {
        let mut this = Self { value: v };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)).prefix_inc();
        this
    }
    pub unsafe fn copy_from(o: *const Three) -> Self {
        let mut this = Self { value: (*o).value };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)).prefix_dec();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Three) -> *mut Three {
        self.value = (*_a0).value;
        return &mut (*(self as *mut Three));
    }
}
impl Clone for Three {
    fn clone(&self) -> Self {
        unsafe { Three::copy_from(self as *const Three) }
    }
}
pub static mut count_13: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn test_14() {
    let bad: () = -1_();
    {
        let mut p: () = (unsafe {
            let mut ___a: bare_allocator_void_ = bare_allocator_void_::new();
            allocate_shared_15(&mut ___a)
        });
        unimplemented!();
    }
    unimplemented!();
    {
        let i: () = 42_();
        let mut p: () = (unsafe {
            let mut ___a: bare_allocator_void_ = bare_allocator_void_::new();
            allocate_shared_16(&mut ___a, &i)
        });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let i: () = 42_();
        let mut p: () = (unsafe {
            let mut ___a: bare_allocator_void_ = bare_allocator_void_::new();
            allocate_shared_17(&mut ___a, &i, &bad)
        });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let i: () = 42_();
        let mut p: () = (unsafe {
            let mut ___a: bare_allocator_void_ = bare_allocator_void_::new();
            let ___args_2: *const () = &bad;
            let ___args_3: *const () = &bad;
            allocate_shared_18(&mut ___a, &i, ___args_2, ___args_3)
        });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_19() {
    let bad: () = -1_();
    {
        let mut p: () = (unsafe {
            let mut ___a: test_allocator_void_ = <test_allocator_void_>::default();
            allocate_shared_20(&mut ___a)
        });
        unimplemented!();
    }
    unimplemented!();
    {
        let i: () = 42_();
        let mut p: () = (unsafe {
            let mut ___a: test_allocator_void_ = <test_allocator_void_>::default();
            allocate_shared_21(&mut ___a, &i)
        });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let i: () = 42_();
        let mut p: () = (unsafe {
            let mut ___a: test_allocator_void_ = <test_allocator_void_>::default();
            allocate_shared_22(&mut ___a, &i, &bad)
        });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let i: () = 42_();
        let mut p: () = (unsafe {
            let mut ___a: test_allocator_void_ = <test_allocator_void_>::default();
            let ___args_2: *const () = &bad;
            let ___args_3: *const () = &bad;
            allocate_shared_23(&mut ___a, &i, ___args_2, ___args_3)
        });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_24() {
    let bad: () = -1_();
    {
        let mut p: () = (unsafe {
            let mut ___a: complete_type_allocator_int_ = complete_type_allocator_int_::new();
            allocate_shared_25(&mut ___a)
        });
        unimplemented!();
    }
    unimplemented!();
    {
        let i: () = 42_();
        let mut p: () = (unsafe {
            let mut ___a: complete_type_allocator_int_ = complete_type_allocator_int_::new();
            allocate_shared_26(&mut ___a, &i)
        });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let i: () = 42_();
        let mut p: () = (unsafe {
            let mut ___a: complete_type_allocator_int_ = complete_type_allocator_int_::new();
            allocate_shared_27(&mut ___a, &i, &bad)
        });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let i: () = 42_();
        let mut p: () = (unsafe {
            let mut ___a: complete_type_allocator_int_ = complete_type_allocator_int_::new();
            let ___args_2: *const () = &bad;
            let ___args_3: *const () = &bad;
            allocate_shared_28(&mut ___a, &i, ___args_2, ___args_3)
        });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
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
    (unsafe { test_14() });
    (unsafe { test_19() });
    (unsafe { test_24() });
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        let mut i: () = 67_();
        let mut c: () = ('e' as ());
        let mut p: () = (unsafe {
            let mut ___a: test_allocator_A_ = test_allocator_A_::new_1({ 54_() }, {
                (&mut alloc_stats as *mut test_allocator_statistics)
            });
            allocate_shared_29(&mut ___a, &mut i, &mut c)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    {
        let mut i: () = 67_();
        let mut c: () = ('e' as ());
        let mut p: () = (unsafe {
            let mut ___a: min_allocator_void_ = <min_allocator_void_>::default();
            allocate_shared_30(&mut ___a, &mut i, &mut c)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let mut i: () = 68_();
        let mut c: () = ('f' as ());
        let mut p: () = (unsafe {
            let mut ___a: bare_allocator_void_ = bare_allocator_void_::new();
            allocate_shared_31(&mut ___a, &mut i, &mut c)
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    {
        let mut p1: () = (unsafe {
            let mut ___a: min_allocator_operator_hijacker_ =
                <min_allocator_operator_hijacker_>::default();
            allocate_shared_32(&mut ___a)
        });
        let mut p2: () = (unsafe {
            let mut ___a: min_allocator_operator_hijacker_ =
                <min_allocator_operator_hijacker_>::default();
            let mut ___args: operator_hijacker = <operator_hijacker>::default();
            allocate_shared_33(&mut ___a, &mut ___args)
        });
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const destructed_value_4);
    std::cell::LazyCell::force(&*&raw const moved_value_5);
    std::cell::LazyCell::force(&*&raw const new_count_8);
    std::cell::LazyCell::force(&*&raw const count_9);
    std::cell::LazyCell::force(&*&raw const count_10);
    std::cell::LazyCell::force(&*&raw const count_11);
    std::cell::LazyCell::force(&*&raw const count_12);
    std::cell::LazyCell::force(&*&raw const count_13);
}
