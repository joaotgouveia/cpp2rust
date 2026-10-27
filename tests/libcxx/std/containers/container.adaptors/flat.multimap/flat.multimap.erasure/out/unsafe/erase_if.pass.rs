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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_int_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_const_int_ptr_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_int_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ {
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
    pub unsafe fn construct_16_int(&mut self, mut p: *mut (), val: *mut ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        unimplemented!();
    }
}
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
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_int_ptr_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ptr_ {
    pub unsafe fn copy_from(a: *const test_allocator_int_ptr_) -> Self {
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
    pub unsafe fn move_from(a: *mut test_allocator_int_ptr_) -> Self {
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
    pub unsafe fn destroy(&mut self, mut p: *mut *mut ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        };
    }
    pub unsafe fn construct_16_int_ptr(&mut self, mut p: *mut *mut (), val: *mut *mut ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        unimplemented!();
    }
    pub unsafe fn construct_16_int_ptrref(&mut self, mut p: *mut *mut (), val: *mut *mut ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        unimplemented!();
    }
}
impl Clone for test_allocator_int_ptr_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_int_ptr_::copy_from(self as *const test_allocator_int_ptr_) }
    }
}
impl Default for test_allocator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            test_allocator_int_ptr_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_const_int_ptr_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_const_int_ptr_ {}
impl Default for test_allocator_const_int_ptr_ {
    fn default() -> Self {
        unsafe {
            test_allocator_const_int_ptr_ {
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
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_6))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_7))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_8)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_5)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_6)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_7)) = 0_();
    }
}
pub static mut outstanding_bytes_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_9(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_lt_10(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) < (y.ptr_));
}
pub unsafe fn operator_eq_11(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_12(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_11(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
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
        mut n: (),
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
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
    pub unsafe fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_int__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_int__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_int__std_integral_constant_unsigned_long__0__::operator_sub_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::Ord for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_10(self.clone(), other.clone()) {
                std::cmp::Ordering::Less
            } else if operator_lt_10(other.clone(), self.clone()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_11(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_13(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_14(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_15(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self
            as *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__));
    }
}
impl std::cmp::PartialEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_14(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_16(
    mut x: min_pointer_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_lt_17(
    mut x: min_pointer_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) < (y.ptr_));
}
pub unsafe fn operator_eq_18(
    mut x: min_pointer_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_19(
    mut x: min_pointer_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_char__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_18(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut min_pointer_char__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_dec_10(
        &mut self,
    ) -> *mut min_pointer_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut min_pointer_char__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_char__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut min_pointer_char__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_char__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_char__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_char__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_char__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_char__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_char__std_integral_constant_unsigned_long__0__::operator_sub_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::Ord for min_pointer_char__std_integral_constant_unsigned_long__0__ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_17(self.clone(), other.clone()) {
                std::cmp::Ordering::Less
            } else if operator_lt_17(other.clone(), self.clone()) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for min_pointer_char__std_integral_constant_unsigned_long__0__ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for min_pointer_char__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_18(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_char__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_20(
    mut x: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_21(
    mut x: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_22(
    mut x: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self
            as *mut min_pointer_const_char__std_integral_constant_unsigned_long__0__));
    }
}
impl std::cmp::PartialEq for min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_21(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_char__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_23(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<min_pointer_int__std_integral_constant_unsigned_long__0__>())
        as () as ());
}
pub unsafe fn operator_eq_24(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_25(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_24(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_int__std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ { return self . ptr_   ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_10 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : () , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : () , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : () , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } pub unsafe fn operator_sub ( &self , mut n : () , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_sub_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::PartialEq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_24(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_26(
    mut x : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<min_pointer_int__std_integral_constant_unsigned_long__0__>())
        as () as ());
}
pub unsafe fn operator_eq_27(
    mut x : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_28(
    mut x : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *const min_pointer_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { pub unsafe fn new_1 ( mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : ( p . ptr_  ) .cast_const() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *const min_pointer_int__std_integral_constant_unsigned_long__0__ { return & ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_9 ( &mut self , ) -> *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_11 ( &mut self , ) -> *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : () , ) -> *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : () , ) -> *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : () , ) -> min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::PartialEq for min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_27(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *const min_pointer_const_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_const_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char_ {}
impl min_allocator_char_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_const_int__std_integral_constant_unsigned_long__0___ {}
const _: () = assert!(true, "HasStdErase<std::vector<int>>");
const _: () = assert!(!(false), "!HasStdErase<std::flat_multimap<int, int>>");
pub unsafe fn make_29(mut vals: ()) -> () {
    let mut ret: () = unimplemented!();
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (*v);
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn make_30(mut vals: ()) -> () {
    let mut ret: () = unimplemented!();
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (*v);
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn make_31(mut vals: ()) -> () {
    let mut ret: () = unimplemented!();
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (*v);
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn make_32(mut vals: ()) -> () {
    let mut ret: () = unimplemented!();
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (*v);
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn make_33(mut vals: ()) -> () {
    let mut ret: () = unimplemented!();
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (*v);
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn make_34(mut vals: ()) -> () {
    let mut ret: () = unimplemented!();
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (*v);
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn make_35(mut vals: ()) -> () {
    let mut ret: () = unimplemented!();
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (*v);
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn test0_36(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_29(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_37(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_29(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_38(
    mut vals: (),
    mut p: FnPtr<fn(*const ()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_29(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_39(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_29(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_40(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_29(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_41(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_29(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_42(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_30(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_43(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_30(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_44(
    mut vals: (),
    mut p: FnPtr<fn(*const ()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_30(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_45(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_30(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_46(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_30(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_47(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_30(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_48(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_31(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_49(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_31(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_50(
    mut vals: (),
    mut p: FnPtr<fn(*const ()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_31(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_51(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_31(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_52(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_31(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_53(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_31(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_54(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_32(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_55(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_32(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_56(
    mut vals: (),
    mut p: FnPtr<fn(*const ()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_32(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_57(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_32(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_58(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_32(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_59(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_32(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_60(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_33(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_61(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_33(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_62(
    mut vals: (),
    mut p: FnPtr<fn(*const ()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_33(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_63(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_33(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_64(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_33(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_65(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_33(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_66(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_34(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_67(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_34(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_68(
    mut vals: (),
    mut p: FnPtr<fn(*const ()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_34(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_69(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_34(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_70(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_34(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_71(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_34(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_72(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_35(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_73(
    mut vals: (),
    mut p: FnPtr<fn(()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_35(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_74(
    mut vals: (),
    mut p: FnPtr<fn(*const ()) -> ()>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_35(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_75(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_35(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_76(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_35(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_77(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_35(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_78() {
    let mut is1: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((*v.first) == (1_()));
        }
    });
    let mut is2: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((v.first) == (2_()));
        }
    });
    let mut is3: FnPtr<fn(*const ()) -> ()> =
        FnPtr::<fn(*const ()) -> ()>::new(|v: *const ()| -> () {
            unsafe {
                return (((*v).first) == (3_()));
            }
        });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((*v.first) == (4_()));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *const ()| -> () {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *mut ()| -> () {
        return false;
    });
    (unsafe { test0_36(unimplemented!(), is1.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_36(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_36(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_37(unimplemented!(), is2.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_36(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_37(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_37(unimplemented!(), is2.copy_from(), unimplemented!(), 3_()) });
    (unsafe { test0_38(unimplemented!(), is3.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_36(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_37(unimplemented!(), is2.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_38(unimplemented!(), is3.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_39(unimplemented!(), is4.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_40(unimplemented!(), True.copy_from(), unimplemented!(), 6_()) });
    (unsafe { test0_41(unimplemented!(), False.copy_from(), unimplemented!(), 0_()) });
}
pub unsafe fn test_79() {
    let mut is1: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((*v.first) == (1_()));
        }
    });
    let mut is2: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((v.first) == (2_()));
        }
    });
    let mut is3: FnPtr<fn(*const ()) -> ()> =
        FnPtr::<fn(*const ()) -> ()>::new(|v: *const ()| -> () {
            unsafe {
                return (((*v).first) == (3_()));
            }
        });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((*v.first) == (4_()));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *const ()| -> () {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *mut ()| -> () {
        return false;
    });
    (unsafe { test0_42(unimplemented!(), is1.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_42(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_42(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_43(unimplemented!(), is2.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_42(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_43(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_43(unimplemented!(), is2.copy_from(), unimplemented!(), 3_()) });
    (unsafe { test0_44(unimplemented!(), is3.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_42(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_43(unimplemented!(), is2.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_44(unimplemented!(), is3.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_45(unimplemented!(), is4.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_46(unimplemented!(), True.copy_from(), unimplemented!(), 6_()) });
    (unsafe { test0_47(unimplemented!(), False.copy_from(), unimplemented!(), 0_()) });
}
pub unsafe fn test_80() {
    let mut is1: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((*v.first) == (1_()));
        }
    });
    let mut is2: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((v.first) == (2_()));
        }
    });
    let mut is3: FnPtr<fn(*const ()) -> ()> =
        FnPtr::<fn(*const ()) -> ()>::new(|v: *const ()| -> () {
            unsafe {
                return (((*v).first) == (3_()));
            }
        });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((*v.first) == (4_()));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *const ()| -> () {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *mut ()| -> () {
        return false;
    });
    (unsafe { test0_48(unimplemented!(), is1.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_48(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_48(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_49(unimplemented!(), is2.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_48(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_49(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_49(unimplemented!(), is2.copy_from(), unimplemented!(), 3_()) });
    (unsafe { test0_50(unimplemented!(), is3.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_48(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_49(unimplemented!(), is2.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_50(unimplemented!(), is3.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_51(unimplemented!(), is4.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_52(unimplemented!(), True.copy_from(), unimplemented!(), 6_()) });
    (unsafe { test0_53(unimplemented!(), False.copy_from(), unimplemented!(), 0_()) });
}
pub unsafe fn test_81() {
    let mut is1: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((*v.first) == (1_()));
        }
    });
    let mut is2: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((v.first) == (2_()));
        }
    });
    let mut is3: FnPtr<fn(*const ()) -> ()> =
        FnPtr::<fn(*const ()) -> ()>::new(|v: *const ()| -> () {
            unsafe {
                return (((*v).first) == (3_()));
            }
        });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((*v.first) == (4_()));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *const ()| -> () {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *mut ()| -> () {
        return false;
    });
    (unsafe { test0_54(unimplemented!(), is1.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_54(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_54(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_55(unimplemented!(), is2.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_54(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_55(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_55(unimplemented!(), is2.copy_from(), unimplemented!(), 3_()) });
    (unsafe { test0_56(unimplemented!(), is3.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_54(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_55(unimplemented!(), is2.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_56(unimplemented!(), is3.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_57(unimplemented!(), is4.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_58(unimplemented!(), True.copy_from(), unimplemented!(), 6_()) });
    (unsafe { test0_59(unimplemented!(), False.copy_from(), unimplemented!(), 0_()) });
}
pub unsafe fn test_82() {
    let mut is1: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((*v.first) == (1_()));
        }
    });
    let mut is2: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((v.first) == (2_()));
        }
    });
    let mut is3: FnPtr<fn(*const ()) -> ()> =
        FnPtr::<fn(*const ()) -> ()>::new(|v: *const ()| -> () {
            unsafe {
                return (((*v).first) == (3_()));
            }
        });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((*v.first) == (4_()));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *const ()| -> () {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *mut ()| -> () {
        return false;
    });
    (unsafe { test0_60(unimplemented!(), is1.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_60(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_60(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_61(unimplemented!(), is2.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_60(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_61(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_61(unimplemented!(), is2.copy_from(), unimplemented!(), 3_()) });
    (unsafe { test0_62(unimplemented!(), is3.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_60(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_61(unimplemented!(), is2.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_62(unimplemented!(), is3.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_63(unimplemented!(), is4.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_64(unimplemented!(), True.copy_from(), unimplemented!(), 6_()) });
    (unsafe { test0_65(unimplemented!(), False.copy_from(), unimplemented!(), 0_()) });
}
pub unsafe fn test_83() {
    let mut is1: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((*v.first) == (1_()));
        }
    });
    let mut is2: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((v.first) == (2_()));
        }
    });
    let mut is3: FnPtr<fn(*const ()) -> ()> =
        FnPtr::<fn(*const ()) -> ()>::new(|v: *const ()| -> () {
            unsafe {
                return (((*v).first) == (3_()));
            }
        });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((*v.first) == (4_()));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *const ()| -> () {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *mut ()| -> () {
        return false;
    });
    (unsafe { test0_66(unimplemented!(), is1.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_66(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_66(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_67(unimplemented!(), is2.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_66(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_67(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_67(unimplemented!(), is2.copy_from(), unimplemented!(), 3_()) });
    (unsafe { test0_68(unimplemented!(), is3.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_66(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_67(unimplemented!(), is2.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_68(unimplemented!(), is3.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_69(unimplemented!(), is4.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_70(unimplemented!(), True.copy_from(), unimplemented!(), 6_()) });
    (unsafe { test0_71(unimplemented!(), False.copy_from(), unimplemented!(), 0_()) });
}
pub unsafe fn test_84() {
    let mut is1: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((*v.first) == (1_()));
        }
    });
    let mut is2: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut v: ()| -> () {
        unsafe {
            return ((v.first) == (2_()));
        }
    });
    let mut is3: FnPtr<fn(*const ()) -> ()> =
        FnPtr::<fn(*const ()) -> ()>::new(|v: *const ()| -> () {
            unsafe {
                return (((*v).first) == (3_()));
            }
        });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((*v.first) == (4_()));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *const ()| -> () {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |_a0: *mut ()| -> () {
        return false;
    });
    (unsafe { test0_72(unimplemented!(), is1.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_72(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_72(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_73(unimplemented!(), is2.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_72(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_73(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_73(unimplemented!(), is2.copy_from(), unimplemented!(), 3_()) });
    (unsafe { test0_74(unimplemented!(), is3.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_72(unimplemented!(), is1.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_73(unimplemented!(), is2.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_74(unimplemented!(), is3.copy_from(), unimplemented!(), 2_()) });
    (unsafe { test0_75(unimplemented!(), is4.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_76(unimplemented!(), True.copy_from(), unimplemented!(), 6_()) });
    (unsafe { test0_77(unimplemented!(), False.copy_from(), unimplemented!(), 0_()) });
}
pub unsafe fn test_85() -> () {
    (unsafe { test_78() });
    (unsafe { test_79() });
    (unsafe { test_80() });
    {
        (unsafe { test_81() });
        (unsafe { test_82() });
    }
    (unsafe { test_83() });
    (unsafe { test_84() });
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
    (unsafe { test_85() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_1);
    std::cell::LazyCell::force(&*&raw const moved_value_2);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_5);
    std::cell::LazyCell::force(&*&raw const alloc_count_6);
    std::cell::LazyCell::force(&*&raw const dealloc_count_7);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_8);
}
