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
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
pub unsafe fn DoNotOptimize_2(value: *const ()) -> *const () {
    c"";
    return value;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_3(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_4(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_5(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_6(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_7(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_8(_a0: *const ()) -> () {
    return true;
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
pub static mut destructed_value_9: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_11(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_12(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_11(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_9));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_9));
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
            operator_eq_11(
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
#[derive(FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_int_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ {
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_9));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_9));
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
    pub unsafe fn construct_16_const_int_ref(&mut self, mut p: *mut (), val: *const ()) {
        if !((self.stats_).is_null()) {
            (*self.stats_).construct_count.prefix_inc();
        }
        unimplemented!();
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
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
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
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_9));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_9));
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
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_14))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_15))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_16)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_13)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_14)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_15)) = 0_();
    }
}
pub static mut outstanding_bytes_13: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_14: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_17(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_18(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_19(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_18(_x, y)
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
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_18(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_20(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_21(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_22(
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
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_const_int__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_const_int__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_const_int__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_21(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        let mut memory: *mut () = unimplemented!();
        {
            (unsafe {
                let ___s: () = (memory as ());
                memset_29(
                    ___s,
                    0_(),
                    (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                )
            });
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    let ___s: () = (p as ());
                    memset_29(
                        ___s,
                        0_(),
                        (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_2(&mut _value)
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_int_ptr_ {}
impl safe_allocator_int_ptr_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut *mut () {
        let mut memory: *mut *mut () = unimplemented!();
        {
            (unsafe {
                memset_29(
                    (memory as ()),
                    0_(),
                    (::std::mem::size_of::<*mut ()>() as ()).wrapping_mul(n),
                )
            });
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut (), mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    memset_29(
                        (p as ()),
                        0_(),
                        (::std::mem::size_of::<*mut ()>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_2(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl Default for safe_allocator_int_ptr_ {
    fn default() -> Self {
        unsafe { safe_allocator_int_ptr_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct safe_allocator_const_int_ptr_ {}
impl safe_allocator_const_int_ptr_ {}
pub unsafe fn test0_30(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_31(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_32(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_33(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_34(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_35(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_36(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_37(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_38(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_39(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_40(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_41(mut s: (), mut val: (), mut expected: (), mut expected_erased_count: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_42() {
    (unsafe { test0_30(unimplemented!(), 1_(), unimplemented!(), 0_()) });
    (unsafe { test0_30(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_30(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_30(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_30(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_30(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_30(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_30(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_30(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_30(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_30(unimplemented!(), 3_(), unimplemented!(), 1_()) });
    (unsafe { test0_30(unimplemented!(), 4_(), unimplemented!(), 0_()) });
    (unsafe { test0_30(unimplemented!(), 1_(), unimplemented!(), 3_()) });
    (unsafe { test0_30(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_30(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_30(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_30(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_30(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_30(unimplemented!(), 2_(), unimplemented!(), 2_()) });
    (unsafe { test0_30(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_31(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
    (unsafe { test0_31(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_31(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_31(unimplemented!(), 3_(), unimplemented!(), 0_()) });
}
pub unsafe fn test_43() {
    (unsafe { test0_32(unimplemented!(), 1_(), unimplemented!(), 0_()) });
    (unsafe { test0_32(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_32(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_32(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_32(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_32(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_32(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_32(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_32(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_32(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_32(unimplemented!(), 3_(), unimplemented!(), 1_()) });
    (unsafe { test0_32(unimplemented!(), 4_(), unimplemented!(), 0_()) });
    (unsafe { test0_32(unimplemented!(), 1_(), unimplemented!(), 3_()) });
    (unsafe { test0_32(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_32(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_32(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_32(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_32(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_32(unimplemented!(), 2_(), unimplemented!(), 2_()) });
    (unsafe { test0_32(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_33(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
    (unsafe { test0_33(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_33(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_33(unimplemented!(), 3_(), unimplemented!(), 0_()) });
}
pub unsafe fn test_44() {
    (unsafe { test0_34(unimplemented!(), 1_(), unimplemented!(), 0_()) });
    (unsafe { test0_34(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_34(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_34(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_34(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_34(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_34(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_34(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_34(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_34(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_34(unimplemented!(), 3_(), unimplemented!(), 1_()) });
    (unsafe { test0_34(unimplemented!(), 4_(), unimplemented!(), 0_()) });
    (unsafe { test0_34(unimplemented!(), 1_(), unimplemented!(), 3_()) });
    (unsafe { test0_34(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_34(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_34(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_34(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_34(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_34(unimplemented!(), 2_(), unimplemented!(), 2_()) });
    (unsafe { test0_34(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_35(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
    (unsafe { test0_35(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_35(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_35(unimplemented!(), 3_(), unimplemented!(), 0_()) });
}
pub unsafe fn test_45() {
    (unsafe { test0_36(unimplemented!(), 1_(), unimplemented!(), 0_()) });
    (unsafe { test0_36(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_36(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_36(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_36(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_36(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_36(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_36(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_36(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_36(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_36(unimplemented!(), 3_(), unimplemented!(), 1_()) });
    (unsafe { test0_36(unimplemented!(), 4_(), unimplemented!(), 0_()) });
    (unsafe { test0_36(unimplemented!(), 1_(), unimplemented!(), 3_()) });
    (unsafe { test0_36(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_36(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_36(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_36(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_36(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_36(unimplemented!(), 2_(), unimplemented!(), 2_()) });
    (unsafe { test0_36(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_37(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
    (unsafe { test0_37(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_37(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_37(unimplemented!(), 3_(), unimplemented!(), 0_()) });
}
pub unsafe fn test_46() {
    (unsafe { test0_38(unimplemented!(), 1_(), unimplemented!(), 0_()) });
    (unsafe { test0_38(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_38(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_38(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_38(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_38(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_38(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_38(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_38(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_38(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_38(unimplemented!(), 3_(), unimplemented!(), 1_()) });
    (unsafe { test0_38(unimplemented!(), 4_(), unimplemented!(), 0_()) });
    (unsafe { test0_38(unimplemented!(), 1_(), unimplemented!(), 3_()) });
    (unsafe { test0_38(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_38(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_38(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_38(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_38(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_38(unimplemented!(), 2_(), unimplemented!(), 2_()) });
    (unsafe { test0_38(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_39(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
    (unsafe { test0_39(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_39(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_39(unimplemented!(), 3_(), unimplemented!(), 0_()) });
}
pub unsafe fn test_47() {
    (unsafe { test0_40(unimplemented!(), 1_(), unimplemented!(), 0_()) });
    (unsafe { test0_40(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_40(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_40(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_40(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_40(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_40(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_40(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_40(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_40(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_40(unimplemented!(), 3_(), unimplemented!(), 1_()) });
    (unsafe { test0_40(unimplemented!(), 4_(), unimplemented!(), 0_()) });
    (unsafe { test0_40(unimplemented!(), 1_(), unimplemented!(), 3_()) });
    (unsafe { test0_40(unimplemented!(), 2_(), unimplemented!(), 0_()) });
    (unsafe { test0_40(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_40(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_40(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_40(unimplemented!(), 1_(), unimplemented!(), 1_()) });
    (unsafe { test0_40(unimplemented!(), 2_(), unimplemented!(), 2_()) });
    (unsafe { test0_40(unimplemented!(), 3_(), unimplemented!(), 0_()) });
    (unsafe { test0_41(unimplemented!(), unimplemented!(), unimplemented!(), 0_()) });
    (unsafe { test0_41(unimplemented!(), 1_(), unimplemented!(), 2_()) });
    (unsafe { test0_41(unimplemented!(), 2_(), unimplemented!(), 1_()) });
    (unsafe { test0_41(unimplemented!(), 3_(), unimplemented!(), 0_()) });
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
    (unsafe { test_43() });
    (unsafe { test_44() });
    (unsafe { test_45() });
    (unsafe { test_46() });
    (unsafe { test_47() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_9);
    std::cell::LazyCell::force(&*&raw const moved_value_10);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_13);
    std::cell::LazyCell::force(&*&raw const alloc_count_14);
    std::cell::LazyCell::force(&*&raw const dealloc_count_15);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_16);
}
