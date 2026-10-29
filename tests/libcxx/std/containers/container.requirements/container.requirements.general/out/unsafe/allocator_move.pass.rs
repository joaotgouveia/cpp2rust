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
pub unsafe fn operator_eq_4(x: *const test_allocator_int_, y: *const test_allocator_int_) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_int_ptr_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_int_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ {
    pub unsafe fn new_1(
        mut data: (),
        mut id: (),
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
        unimplemented!();
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
        unimplemented!();
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_1));
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn allocate(&mut self, mut n: (), mut _a1: Option<()>) -> *mut () {
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
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
    pub unsafe fn destroy(&mut self, mut p: *mut ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        };
    }
    pub unsafe fn get_data(&self) -> () {
        return self.data_;
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
}
impl std::cmp::PartialEq for test_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_4(
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
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_bool_ {
    pub unsafe fn new_1(a: *const test_allocator_unsigned_long_) -> Self {
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
pub unsafe fn operator_eq_5(x: *const test_allocator_bool_, y: *const test_allocator_bool_) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_bool__rebind_unsigned_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_bool__rebind_bool_ {}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_bool_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_bool_ {
    pub unsafe fn new_2(
        mut data: (),
        mut id: (),
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
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn get_data(&self) -> () {
        return self.data_;
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
}
impl std::cmp::PartialEq for test_allocator_bool_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
                self as *const test_allocator_bool_,
                other as *const test_allocator_bool_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_bool_ {}
impl Default for test_allocator_bool_ {
    fn default() -> Self {
        unsafe {
            test_allocator_bool_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_6(
    x: *const test_allocator_std_pair_const_int__int__,
    y: *const test_allocator_std_pair_const_int__int__,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__int___rebind_std_pair_const_int__int__ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_pair_const_int__int__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__int__ {
    pub unsafe fn new_1(
        mut data: (),
        mut id: (),
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
    pub unsafe fn copy_from(a: *const test_allocator_std_pair_const_int__int__) -> Self {
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn get_data(&self) -> () {
        return self.data_;
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
}
impl std::cmp::PartialEq for test_allocator_std_pair_const_int__int__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_6(
                self as *const test_allocator_std_pair_const_int__int__,
                other as *const test_allocator_std_pair_const_int__int__,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_std_pair_const_int__int__ {}
impl Clone for test_allocator_std_pair_const_int__int__ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_std_pair_const_int__int__::copy_from(
                self as *const test_allocator_std_pair_const_int__int__,
            )
        }
    }
}
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
impl test_allocator_unsigned_long_ {
    pub unsafe fn new_1(a: *const test_allocator_bool_) -> Self {
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
pub struct test_allocator_unsigned_long_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_unsigned_long_ {
    pub unsafe fn copy_from(a: *const test_allocator_unsigned_long_) -> Self {
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
    pub unsafe fn move_from(a: *mut test_allocator_unsigned_long_) -> Self {
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
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_1));
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
}
impl Clone for test_allocator_unsigned_long_ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_unsigned_long_::copy_from(self as *const test_allocator_unsigned_long_)
        }
    }
}
impl Default for test_allocator_unsigned_long_ {
    fn default() -> Self {
        unsafe {
            test_allocator_unsigned_long_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_int_ptr_ {
    pub unsafe fn new_1(a: *const test_allocator_int_) -> Self {
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
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_int_ptr_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ptr_ {
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_0));
    }
    pub unsafe fn allocate(&mut self, mut n: (), mut _a1: Option<()>) -> *mut *mut () {
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
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut (), mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
}
impl Default for test_allocator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            test_allocator_int_ptr_ {
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
pub unsafe fn test_7(mut expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| unsafe { 1_() });
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_0: test_allocator_int_ = unimplemented!();
            let a: *const test_allocator_int_ = &mut __tmp_0;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_1: test_allocator_int_ = unimplemented!();
            let a1: *const test_allocator_int_ = &mut __tmp_1;
            unimplemented!();
            unimplemented!();
            let mut __tmp_2: test_allocator_int_ = unimplemented!();
            let a2: *const test_allocator_int_ = &mut __tmp_2;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_8(mut expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| unsafe { 1_() });
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_3: test_allocator_bool_ = unimplemented!();
            let a: *const test_allocator_bool_ = &mut __tmp_3;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_4: test_allocator_bool_ = unimplemented!();
            let a1: *const test_allocator_bool_ = &mut __tmp_4;
            unimplemented!();
            unimplemented!();
            let mut __tmp_5: test_allocator_bool_ = unimplemented!();
            let a2: *const test_allocator_bool_ = &mut __tmp_5;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_9(mut expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| unsafe { 1_() });
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_6: test_allocator_int_ = unimplemented!();
            let a: *const test_allocator_int_ = &mut __tmp_6;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_7: test_allocator_int_ = unimplemented!();
            let a1: *const test_allocator_int_ = &mut __tmp_7;
            unimplemented!();
            unimplemented!();
            let mut __tmp_8: test_allocator_int_ = unimplemented!();
            let a2: *const test_allocator_int_ = &mut __tmp_8;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_10(mut expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| unsafe { 1_() });
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_9: test_allocator_int_ = unimplemented!();
            let a: *const test_allocator_int_ = &mut __tmp_9;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_10: test_allocator_int_ = unimplemented!();
            let a1: *const test_allocator_int_ = &mut __tmp_10;
            unimplemented!();
            unimplemented!();
            let mut __tmp_11: test_allocator_int_ = unimplemented!();
            let a2: *const test_allocator_int_ = &mut __tmp_11;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_11(mut expected_num_allocs: ()) {
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_12: test_allocator_int_ = unimplemented!();
            let a: *const test_allocator_int_ = &mut __tmp_12;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_13: test_allocator_int_ = unimplemented!();
            let a1: *const test_allocator_int_ = &mut __tmp_13;
            unimplemented!();
            unimplemented!();
            let mut __tmp_14: test_allocator_int_ = unimplemented!();
            let a2: *const test_allocator_int_ = &mut __tmp_14;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_12(mut expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| unsafe { 1_() });
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_15: test_allocator_int_ = unimplemented!();
            let a: *const test_allocator_int_ = &mut __tmp_15;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_16: test_allocator_int_ = unimplemented!();
            let a1: *const test_allocator_int_ = &mut __tmp_16;
            unimplemented!();
            unimplemented!();
            let mut __tmp_17: test_allocator_int_ = unimplemented!();
            let a2: *const test_allocator_int_ = &mut __tmp_17;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_13(mut expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| unsafe { 1_() });
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_18: test_allocator_int_ = unimplemented!();
            let a: *const test_allocator_int_ = &mut __tmp_18;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_19: test_allocator_int_ = unimplemented!();
            let a1: *const test_allocator_int_ = &mut __tmp_19;
            unimplemented!();
            unimplemented!();
            let mut __tmp_20: test_allocator_int_ = unimplemented!();
            let a2: *const test_allocator_int_ = &mut __tmp_20;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_14(mut expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| unsafe { 1_() });
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_21: test_allocator_std_pair_const_int__int__ = unimplemented!();
            let a: *const test_allocator_std_pair_const_int__int__ = &mut __tmp_21;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_22: test_allocator_std_pair_const_int__int__ = unimplemented!();
            let a1: *const test_allocator_std_pair_const_int__int__ = &mut __tmp_22;
            unimplemented!();
            unimplemented!();
            let mut __tmp_23: test_allocator_std_pair_const_int__int__ = unimplemented!();
            let a2: *const test_allocator_std_pair_const_int__int__ = &mut __tmp_23;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_15(mut expected_num_allocs: Option<()>) {
    let mut expected_num_allocs: () = expected_num_allocs.unwrap_or_else(|| unsafe { 1_() });
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_24: test_allocator_std_pair_const_int__int__ = unimplemented!();
            let a: *const test_allocator_std_pair_const_int__int__ = &mut __tmp_24;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_25: test_allocator_std_pair_const_int__int__ = unimplemented!();
            let a1: *const test_allocator_std_pair_const_int__int__ = &mut __tmp_25;
            unimplemented!();
            unimplemented!();
            let mut __tmp_26: test_allocator_std_pair_const_int__int__ = unimplemented!();
            let a2: *const test_allocator_std_pair_const_int__int__ = &mut __tmp_26;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_16(mut expected_num_allocs: ()) {
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_27: test_allocator_int_ = unimplemented!();
            let a: *const test_allocator_int_ = &mut __tmp_27;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_28: test_allocator_int_ = unimplemented!();
            let a1: *const test_allocator_int_ = &mut __tmp_28;
            unimplemented!();
            unimplemented!();
            let mut __tmp_29: test_allocator_int_ = unimplemented!();
            let a2: *const test_allocator_int_ = &mut __tmp_29;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_17(mut expected_num_allocs: ()) {
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_30: test_allocator_int_ = unimplemented!();
            let a: *const test_allocator_int_ = &mut __tmp_30;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_31: test_allocator_int_ = unimplemented!();
            let a1: *const test_allocator_int_ = &mut __tmp_31;
            unimplemented!();
            unimplemented!();
            let mut __tmp_32: test_allocator_int_ = unimplemented!();
            let a2: *const test_allocator_int_ = &mut __tmp_32;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_18(mut expected_num_allocs: ()) {
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_33: test_allocator_std_pair_const_int__int__ = unimplemented!();
            let a: *const test_allocator_std_pair_const_int__int__ = &mut __tmp_33;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_34: test_allocator_std_pair_const_int__int__ = unimplemented!();
            let a1: *const test_allocator_std_pair_const_int__int__ = &mut __tmp_34;
            unimplemented!();
            unimplemented!();
            let mut __tmp_35: test_allocator_std_pair_const_int__int__ = unimplemented!();
            let a2: *const test_allocator_std_pair_const_int__int__ = &mut __tmp_35;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_19(mut expected_num_allocs: ()) {
    let mut alloc_stats: test_allocator_statistics = <test_allocator_statistics>::default();
    {
        (unsafe { test_allocator_statistics::clear(&mut alloc_stats) });
        let mut v: () = unimplemented!();
        unimplemented!();
        let num_stored_allocs: () = alloc_stats.count;
        {
            let mut __tmp_36: test_allocator_std_pair_const_int__int__ = unimplemented!();
            let a: *const test_allocator_std_pair_const_int__int__ = &mut __tmp_36;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        unimplemented!();
        (unsafe { test_allocator_statistics::clear_ctor_counters(&mut alloc_stats) });
        let mut v2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            let mut __tmp_37: test_allocator_std_pair_const_int__int__ = unimplemented!();
            let a1: *const test_allocator_std_pair_const_int__int__ = &mut __tmp_37;
            unimplemented!();
            unimplemented!();
            let mut __tmp_38: test_allocator_std_pair_const_int__int__ = unimplemented!();
            let a2: *const test_allocator_std_pair_const_int__int__ = &mut __tmp_38;
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
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
    {
        (unsafe { test_7(None) });
        (unsafe { test_8(None) });
        (unsafe { test_9(None) });
        (unsafe { test_10(None) });
        let mut stored_allocators: () = 1_();
        (unsafe { test_11(stored_allocators) });
    }
    {
        (unsafe { test_12(None) });
        (unsafe { test_13(None) });
        (unsafe { test_14(None) });
        (unsafe { test_15(None) });
    }
    {
        let mut stored_allocators: () = 1_();
        (unsafe { test_16(stored_allocators) });
        (unsafe { test_17(stored_allocators) });
        (unsafe { test_18(stored_allocators) });
        (unsafe { test_19(stored_allocators) });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
}
