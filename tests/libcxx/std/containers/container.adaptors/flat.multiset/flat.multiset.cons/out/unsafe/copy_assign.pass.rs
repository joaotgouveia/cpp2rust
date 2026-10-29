extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub unsafe fn operator_eq_0(a: *const test_less_int_, b: *const test_less_int_) -> () {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_less_int_ {
    pub data_: (),
}
impl test_less_int_ {
    pub unsafe fn new_1(mut data: ()) -> Self {
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
pub static mut destructed_value_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_3(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_4(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_3(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
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
            operator_eq_3(
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
pub unsafe fn operator_eq_5(x: *const test_allocator_int_, y: *const test_allocator_int_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_6(x: *const test_allocator_int_, y: *const test_allocator_int_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_int_ = x;
        let _y: *const test_allocator_int_ = y;
        operator_eq_5(_x, _y)
    });
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
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
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
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_2));
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
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
    pub unsafe fn max_size(&self) -> () {
        return (((2147483647_()).wrapping_mul((2_() as ())) as ()).wrapping_add((1_() as ())))
            .wrapping_div((::std::mem::size_of::<()>() as ()));
    }
    pub unsafe fn destroy(&mut self, mut p: *mut ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        };
    }
    pub unsafe fn construct_16_const_int_ref(&mut self, mut p: *mut (), val: *const ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        unimplemented!();
    }
    pub unsafe fn construct_16_int_ref(&mut self, mut p: *mut (), val: *mut ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        unimplemented!();
    }
    pub unsafe fn construct_16_int(&mut self, mut p: *mut (), val: *mut ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        unimplemented!();
    }
}
impl std::cmp::PartialEq for test_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
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
                data_: 0_(),
                id_: 0_(),
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_1));
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
pub unsafe fn operator_eq_7(x: *const other_allocator_int_, y: *const other_allocator_int_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(x: *const other_allocator_int_, y: *const other_allocator_int_) -> () {
    return !(unsafe {
        let _x: *const other_allocator_int_ = x;
        let _y: *const other_allocator_int_ = y;
        operator_eq_7(_x, _y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_int_ {
    data_: (),
}
impl other_allocator_int_ {
    pub unsafe fn new_1(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for other_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_7(
                self as *const other_allocator_int_,
                other as *const other_allocator_int_,
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_int_ {}
impl Default for other_allocator_int_ {
    fn default() -> Self {
        unsafe {
            other_allocator_int_ {
                data_: unimplemented!(),
            }
        }
    }
}
impl other_allocator_int_ptr_ {
    pub unsafe fn new_1(a: *const other_allocator_int_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_int_ptr_ {
    data_: (),
}
impl other_allocator_int_ptr_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut *mut () {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut (), mut s: ()) {
        unimplemented!();
    }
}
impl Default for other_allocator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            other_allocator_int_ptr_ {
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
pub unsafe fn test_9() {
    {
        let mut ks: () = unimplemented!();
        let mut mo: () = {
            let mut __tmp_0: test_less_int_ = test_less_int_::new_1({ 5_() });
            std_flat_multiset_int__test_less_int___std_vector_int__test_allocator_int___::new_1(
                { unimplemented!() },
                { &mut __tmp_0 },
            )
        };
        let mut m: () = {
            let mut __tmp_1: () = unimplemented!();
            let mut __tmp_2: test_less_int_ = test_less_int_::new_1({ 3_() });
            let mut __tmp_3: test_allocator_int_ = test_allocator_int_::new_1({ 2_() });
            std_flat_multiset_int__test_less_int___std_vector_int__test_allocator_int___::new_2(
                { &mut __tmp_1 },
                { &mut __tmp_2 },
                { &mut __tmp_3 },
            )
        };
        m = (mo).clone();
        unimplemented!();
        unimplemented!();
        let mut keys: () = (unsafe {
            ( m as std__Flat_set_impl_int__test_less_int___std_vector_int__test_allocator_int____true_ ) . extract   ( )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut keys2: () = (unsafe {
            ( mo as std__Flat_set_impl_int__test_less_int___std_vector_int__test_allocator_int____true_ ) . extract   ( )
        });
        unimplemented!();
    }
    {
        let mut ks: () = unimplemented!();
        let expected: [(); 4] = [1_(), 3_(), 3_(), 5_()];
        let mut mo: () = {
            let mut __tmp_4: test_less_int_ = test_less_int_::new_1({ 5_() });
            std_flat_multiset_int__test_less_int___std_vector_int__other_allocator_int___::new_1(
                { unimplemented!() },
                { &mut __tmp_4 },
            )
        };
        let mut m: () = {
            let mut __tmp_5: test_less_int_ = test_less_int_::new_1({ 3_() });
            let mut __tmp_6: other_allocator_int_ = other_allocator_int_::new_1({ 2_() });
            std_flat_multiset_int__test_less_int___std_vector_int__other_allocator_int___::new_2(
                { unimplemented!() },
                { &mut __tmp_5 },
                { &mut __tmp_6 },
            )
        };
        m = (mo).clone();
        unimplemented!();
        unimplemented!();
        let mut keys: () = (unsafe {
            ( m as std__Flat_set_impl_int__test_less_int___std_vector_int__other_allocator_int____true_ ) . extract   ( )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut keys2: () = (unsafe {
            ( mo as std__Flat_set_impl_int__test_less_int___std_vector_int__other_allocator_int____true_ ) . extract   ( )
        });
        unimplemented!();
    }
    {
        let mut mo: () = {
            let mut __tmp_7: () = unimplemented!();
            std_flat_multiset_int__std_function_bool__int__int____std_vector_int__std_allocator_int___ :: new_1 ( { unimplemented!() } , { & mut __tmp_7 } , )
        };
        let mut m: () = {
            let mut __tmp_8: () = unimplemented!();
            std_flat_multiset_int__std_function_bool__int__int____std_vector_int__std_allocator_int___ :: new_1 ( { unimplemented!() } , { & mut __tmp_8 } , )
        };
        unimplemented!();
        unimplemented!();
        m = (mo).clone();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () =
            std_flat_multiset_int__std_less_int___std_vector_int__std_allocator_int___::new_1(
                { unimplemented!() },
                None,
            );
        m = (m).clone();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        unimplemented!();
        m = std_flat_multiset_int__std_less_int___std_vector_int__std_allocator_int___::new_1(
            { unimplemented!() },
            None,
        );
        let mut expected: [(); 10] = [1_(), 2_(), 2_(), 3_(), 3_(), 3_(), 4_(), 5_(), 5_(), 6_()];
        unimplemented!();
    }
    {
        let mut so: () = unimplemented!();
        let mut s: () = unimplemented!();
        s = (so).clone();
        s = (so).clone();
        (unsafe { swap_10(&mut s, &mut so) });
    }
}
pub unsafe fn test_11() {
    {
        let mut ks: () = unimplemented!();
        let mut mo: () = {
            let mut __tmp_9: test_less_int_ = test_less_int_::new_1({ 5_() });
            std_flat_multiset_int__test_less_int___std_deque_int__test_allocator_int___::new_1(
                { unimplemented!() },
                { &mut __tmp_9 },
            )
        };
        let mut m: () = {
            let mut __tmp_10: () = unimplemented!();
            let mut __tmp_11: test_less_int_ = test_less_int_::new_1({ 3_() });
            let mut __tmp_12: test_allocator_int_ = test_allocator_int_::new_1({ 2_() });
            std_flat_multiset_int__test_less_int___std_deque_int__test_allocator_int___::new_2(
                { &mut __tmp_10 },
                { &mut __tmp_11 },
                { &mut __tmp_12 },
            )
        };
        m = (mo).clone();
        unimplemented!();
        unimplemented!();
        let mut keys: () = (unsafe {
            ( m as std__Flat_set_impl_int__test_less_int___std_deque_int__test_allocator_int____true_ ) . extract   ( )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut keys2: () = (unsafe {
            ( mo as std__Flat_set_impl_int__test_less_int___std_deque_int__test_allocator_int____true_ ) . extract   ( )
        });
        unimplemented!();
    }
    {
        let mut ks: () = unimplemented!();
        let expected: [(); 4] = [1_(), 3_(), 3_(), 5_()];
        let mut mo: () = {
            let mut __tmp_13: test_less_int_ = test_less_int_::new_1({ 5_() });
            std_flat_multiset_int__test_less_int___std_deque_int__other_allocator_int___::new_1(
                { unimplemented!() },
                { &mut __tmp_13 },
            )
        };
        let mut m: () = {
            let mut __tmp_14: test_less_int_ = test_less_int_::new_1({ 3_() });
            let mut __tmp_15: other_allocator_int_ = other_allocator_int_::new_1({ 2_() });
            std_flat_multiset_int__test_less_int___std_deque_int__other_allocator_int___::new_2(
                { unimplemented!() },
                { &mut __tmp_14 },
                { &mut __tmp_15 },
            )
        };
        m = (mo).clone();
        unimplemented!();
        unimplemented!();
        let mut keys: () = (unsafe {
            ( m as std__Flat_set_impl_int__test_less_int___std_deque_int__other_allocator_int____true_ ) . extract   ( )
        });
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut keys2: () = (unsafe {
            ( mo as std__Flat_set_impl_int__test_less_int___std_deque_int__other_allocator_int____true_ ) . extract   ( )
        });
        unimplemented!();
    }
    {
        let mut mo: () = {
            let mut __tmp_16: () = unimplemented!();
            std_flat_multiset_int__std_function_bool__int__int____std_vector_int__std_allocator_int___ :: new_1 ( { unimplemented!() } , { & mut __tmp_16 } , )
        };
        let mut m: () = {
            let mut __tmp_17: () = unimplemented!();
            std_flat_multiset_int__std_function_bool__int__int____std_vector_int__std_allocator_int___ :: new_1 ( { unimplemented!() } , { & mut __tmp_17 } , )
        };
        unimplemented!();
        unimplemented!();
        m = (mo).clone();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () =
            std_flat_multiset_int__std_less_int___std_vector_int__std_allocator_int___::new_1(
                { unimplemented!() },
                None,
            );
        m = (m).clone();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        unimplemented!();
        m = std_flat_multiset_int__std_less_int___std_vector_int__std_allocator_int___::new_1(
            { unimplemented!() },
            None,
        );
        let mut expected: [(); 10] = [1_(), 2_(), 2_(), 3_(), 3_(), 3_(), 4_(), 5_(), 5_(), 6_()];
        unimplemented!();
    }
    {
        let mut so: () = unimplemented!();
        let mut s: () = unimplemented!();
        s = (so).clone();
        s = (so).clone();
        (unsafe { swap_10(&mut s, &mut so) });
    }
}
pub unsafe fn test_12() -> () {
    (unsafe { test_9() });
    {
        (unsafe { test_11() });
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
    (unsafe { test_12() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_1);
    std::cell::LazyCell::force(&*&raw const moved_value_2);
}
