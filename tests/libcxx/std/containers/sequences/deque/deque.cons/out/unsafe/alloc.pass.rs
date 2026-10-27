extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn DoNotOptimize_0(value: *const ()) -> *const () {
    c"";
    return value;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_1(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_2(_a0: *const ()) -> () {
    return true;
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
pub unsafe fn operator_eq_13(
    x: *const test_allocator_NotConstructible_,
    y: *const test_allocator_NotConstructible_,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_NotConstructible__rebind_NotConstructible_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_NotConstructible__rebind_NotConstructible_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_NotConstructible__rebind_const_NotConstructible_ptr_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_NotConstructible_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_NotConstructible_ {
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_NotConstructible_) -> Self {
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
    pub unsafe fn deallocate(&mut self, mut p: *mut NotConstructible, mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
    pub unsafe fn destroy(&mut self, mut p: *mut NotConstructible) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        };
    }
}
impl std::cmp::PartialEq for test_allocator_NotConstructible_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_13(
                self as *const test_allocator_NotConstructible_,
                other as *const test_allocator_NotConstructible_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_NotConstructible_ {}
impl Clone for test_allocator_NotConstructible_ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_NotConstructible_::copy_from(
                self as *const test_allocator_NotConstructible_,
            )
        }
    }
}
impl Default for test_allocator_NotConstructible_ {
    fn default() -> Self {
        unsafe {
            test_allocator_NotConstructible_ {
                data_: unimplemented!(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_NotConstructible_ptr_ {
    pub unsafe fn new_1(a: *const test_allocator_NotConstructible_) -> Self {
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
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_NotConstructible_ptr_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_NotConstructible_ptr_ {
    pub unsafe fn copy_from(a: *const test_allocator_NotConstructible_ptr_) -> Self {
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
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut NotConstructible, mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
    pub unsafe fn destroy(&mut self, mut p: *mut *mut NotConstructible) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        };
    }
}
impl Clone for test_allocator_NotConstructible_ptr_ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_NotConstructible_ptr_::copy_from(
                self as *const test_allocator_NotConstructible_ptr_,
            )
        }
    }
}
impl Default for test_allocator_NotConstructible_ptr_ {
    fn default() -> Self {
        unsafe {
            test_allocator_NotConstructible_ptr_ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_const_NotConstructible_ptr_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_const_NotConstructible_ptr_ {}
impl Default for test_allocator_const_NotConstructible_ptr_ {
    fn default() -> Self {
        unsafe {
            test_allocator_const_NotConstructible_ptr_ {
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
#[derive(FnPtrArg, Default)]
pub struct NotConstructible {}
impl NotConstructible {}
impl std::cmp::PartialEq for NotConstructible {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_14(
                self as *const NotConstructible,
                other as *const NotConstructible,
            )
        }
    }
}
impl std::cmp::Eq for NotConstructible {}
pub unsafe fn operator_eq_14(_a0: *const NotConstructible, _a1: *const NotConstructible) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_16))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_17))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_18)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_15)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_16)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_17)) = 0_();
    }
}
pub static mut outstanding_bytes_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_19(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_20(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(mut _a0: ()) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
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
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_20(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_21(
    mut x: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<NotConstructible>()) as ()
        as ());
}
pub unsafe fn operator_eq_22(
    mut x: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut NotConstructible,
}
impl min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(mut _a0: ()) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut NotConstructible {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self
            as *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ =
            (*(self
                as *const min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq
    for min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_22(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
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
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { pub unsafe fn new_1 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ { return self . ptr_   ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_10 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : () , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : () , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::PartialEq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_24(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *const min_pointer_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { pub unsafe fn new_1 ( mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : ( p . ptr_  ) .cast_const() , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *const min_pointer_int__std_integral_constant_unsigned_long__0__ { return & ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_9 ( &mut self , ) -> *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_const_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
pub unsafe fn operator_sub_26(
    mut x : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<
            min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
        >()) as () as ());
}
pub unsafe fn operator_eq_27(
    mut x : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_28(
    mut x : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_27(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { pub unsafe fn new_1 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ { return self . ptr_   ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_10 ( &mut self , ) -> *mut min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : () , ) -> *mut min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : () , ) -> min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::PartialEq for min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_27(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *const min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_const_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { pub unsafe fn new_1 ( mut p : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : ( p . ptr_  ) .cast_const() , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *const min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__ { return & ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_9 ( &mut self , ) -> *mut min_pointer_const_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_const_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_const_NotConstructible__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_const_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
pub unsafe fn operator_eq_29(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for min_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_29(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_int_ {}
pub unsafe fn operator_eq_30(
    mut _a0: min_allocator_NotConstructible_,
    mut _a1: min_allocator_NotConstructible_,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_NotConstructible_ {}
impl min_allocator_NotConstructible_ {
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_NotConstructible__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for min_allocator_NotConstructible_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_30(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_NotConstructible_ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
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
impl min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_NotConstructible_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {
}
impl min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_const_NotConstructible__std_integral_constant_unsigned_long__0___
{}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_NotConstructible__std_integral_constant_unsigned_long__0___ {}
pub unsafe fn operator_eq_31(
    mut _a0: explicit_allocator_int_,
    mut _a1: explicit_allocator_int_,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct explicit_allocator_int_ {}
impl explicit_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for explicit_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_31(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for explicit_allocator_int_ {}
impl Default for explicit_allocator_int_ {
    fn default() -> Self {
        unsafe { explicit_allocator_int_::new() }
    }
}
pub unsafe fn operator_eq_32(
    mut _a0: explicit_allocator_NotConstructible_,
    mut _a1: explicit_allocator_NotConstructible_,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct explicit_allocator_NotConstructible_ {}
impl explicit_allocator_NotConstructible_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut NotConstructible, mut n: ()) {
        unimplemented!();
    }
}
impl std::cmp::PartialEq for explicit_allocator_NotConstructible_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_32(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for explicit_allocator_NotConstructible_ {}
impl Default for explicit_allocator_NotConstructible_ {
    fn default() -> Self {
        unsafe { explicit_allocator_NotConstructible_::new() }
    }
}
impl explicit_allocator_int_ptr_ {
    pub unsafe fn new_1(mut _a0: explicit_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct explicit_allocator_int_ptr_ {}
impl explicit_allocator_int_ptr_ {
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut (), mut n: ()) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct explicit_allocator_const_int_ptr_ {}
impl explicit_allocator_const_int_ptr_ {}
impl explicit_allocator_NotConstructible_ptr_ {
    pub unsafe fn new_1(mut _a0: explicit_allocator_NotConstructible_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct explicit_allocator_NotConstructible_ptr_ {}
impl explicit_allocator_NotConstructible_ptr_ {
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut NotConstructible, mut n: ()) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct explicit_allocator_const_NotConstructible_ptr_ {}
impl explicit_allocator_const_NotConstructible_ptr_ {}
pub unsafe fn operator_eq_33(mut _a0: safe_allocator_int_, mut _a1: safe_allocator_int_) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    let ___s: () = (p as ());
                    memset_34(
                        ___s,
                        0_(),
                        (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl std::cmp::PartialEq for safe_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_33(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for safe_allocator_int_ {}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        unsafe { safe_allocator_int_::new() }
    }
}
pub unsafe fn operator_eq_35(
    mut _a0: safe_allocator_NotConstructible_,
    mut _a1: safe_allocator_NotConstructible_,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_NotConstructible_ {}
impl safe_allocator_NotConstructible_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut NotConstructible, mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    memset_34(
                        (p as ()),
                        0_(),
                        (::std::mem::size_of::<NotConstructible>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl std::cmp::PartialEq for safe_allocator_NotConstructible_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_35(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for safe_allocator_NotConstructible_ {}
impl Default for safe_allocator_NotConstructible_ {
    fn default() -> Self {
        unsafe { safe_allocator_NotConstructible_::new() }
    }
}
impl safe_allocator_int_ptr_ {
    pub unsafe fn new_1(mut _a0: safe_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct safe_allocator_int_ptr_ {}
impl safe_allocator_int_ptr_ {
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut (), mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    memset_34(
                        (p as ()),
                        0_(),
                        (::std::mem::size_of::<*mut ()>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct safe_allocator_const_int_ptr_ {}
impl safe_allocator_const_int_ptr_ {}
impl safe_allocator_NotConstructible_ptr_ {
    pub unsafe fn new_1(mut _a0: safe_allocator_NotConstructible_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct safe_allocator_NotConstructible_ptr_ {}
impl safe_allocator_NotConstructible_ptr_ {
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut NotConstructible, mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    memset_34(
                        (p as ()),
                        0_(),
                        (::std::mem::size_of::<*mut NotConstructible>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct safe_allocator_const_NotConstructible_ptr_ {}
impl safe_allocator_const_NotConstructible_ptr_ {}
pub unsafe fn test_36(a: *const ()) {
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_37(a: *const test_allocator_NotConstructible_) {
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_38(a: *const min_allocator_int_) {
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_39(a: *const safe_allocator_int_) {
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_40(a: *const min_allocator_NotConstructible_) {
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_41(a: *const safe_allocator_NotConstructible_) {
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_42(a: *const explicit_allocator_int_) {
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_43(a: *const explicit_allocator_NotConstructible_) {
    let mut d: () = unimplemented!();
    unimplemented!();
    unimplemented!();
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
    (unsafe {
        let mut _a: () = unimplemented!();
        test_36(&mut _a)
    });
    (unsafe {
        let mut _a: test_allocator_NotConstructible_ =
            test_allocator_NotConstructible_::new_1({ 3_() });
        test_37(&mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_int_ = <min_allocator_int_>::default();
        test_38(&mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_int_ = safe_allocator_int_::new();
        test_39(&mut _a)
    });
    (unsafe {
        let mut _a: min_allocator_NotConstructible_ = <min_allocator_NotConstructible_>::default();
        test_40(&mut _a)
    });
    (unsafe {
        let mut _a: safe_allocator_NotConstructible_ = safe_allocator_NotConstructible_::new();
        test_41(&mut _a)
    });
    (unsafe {
        let mut _a: explicit_allocator_int_ = explicit_allocator_int_::new();
        test_42(&mut _a)
    });
    (unsafe {
        let mut _a: explicit_allocator_NotConstructible_ =
            explicit_allocator_NotConstructible_::new();
        test_43(&mut _a)
    });
    return 0_();
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_const_NotConstructible__std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_9);
    std::cell::LazyCell::force(&*&raw const moved_value_10);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_15);
    std::cell::LazyCell::force(&*&raw const alloc_count_16);
    std::cell::LazyCell::force(&*&raw const dealloc_count_17);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_18);
}
