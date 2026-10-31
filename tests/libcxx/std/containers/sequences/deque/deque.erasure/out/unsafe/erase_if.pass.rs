extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub unsafe fn DoNotOptimize_1(value: *const *mut ::libc::c_void) -> *const *mut ::libc::c_void {
    c"";
    return value;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: i32,
    pub throw_after: i32,
    pub count: i32,
    pub alloc_count: i32,
    pub allocated_size: i32,
    pub construct_count: i32,
    pub destroy_count: i32,
    pub copied: i32,
    pub moved: i32,
    pub converted: i32,
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        assert!(
            ((self.count) == (0)) && (!(c"clearing leaking allocator data?".as_ptr()).is_null())
        );
        self.count = 0;
        self.time_to_throw = 0;
        self.alloc_count = 0;
        self.allocated_size = 0;
        self.construct_count = 0;
        self.destroy_count = 0;
        self.throw_after = 2147483647;
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0;
        self.moved = 0;
        self.converted = 0;
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0,
                throw_after: 2147483647,
                count: 0,
                alloc_count: 0,
                allocated_size: 0,
                construct_count: 0,
                destroy_count: 0,
                copied: 0,
                moved: 0,
                converted: 0,
            }
        }
    }
}
pub static mut destructed_value_2: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { -1_i32 });
pub static mut moved_value_3: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2147483647 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_4(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_5(
    x: *const test_allocator_void_,
    y: *const test_allocator_void_,
) -> bool {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_4(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: i32, mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0,
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: i32, mut id: i32) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: i32,
        mut id: i32,
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
    }
    pub unsafe fn get_id(&self) -> i32 {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> i32 {
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
            operator_eq_4(
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
                data_: 0,
                id_: 0,
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
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_int_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ {
    pub unsafe fn copy_from(a: *const test_allocator_int_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        assert!(
            ((((*a).data_)
                != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2)))
                && (((*a).id_)
                    != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2))))
                && (!(c"copying from destroyed allocator".as_ptr()).is_null())
        );
        if !((this.stats_).is_null()) {
            (*this.stats_).count.prefix_inc();
            (*this.stats_).copied.prefix_inc();
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u32,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2)))
        );
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                std::process::abort();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_add(n)) as i32;
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u32 {
        return ((((((2147483647_u32).wrapping_mul((2_u32 as u32)) as u32)
            .wrapping_add((1_u32 as u32))) as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    }
    pub unsafe fn destroy(&mut self, mut p: *mut i32) {
        if !((self.stats_).is_null()) {
            (*self.stats_).destroy_count.prefix_inc();
        };
    }
    pub unsafe fn construct_16_const_int_ref(&mut self, mut p: *mut i32, val: *const i32) {
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
                data_: 0,
                id_: 0,
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
#[derive(FnPtrArg, CppFromUnsafe, DestructorUnsafe)]
#[cpp_from(test_allocator_int_, new_1, ptr)]
pub struct test_allocator_int_ptr_ {
    data_: i32,
    id_: i32,
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ptr_ {
    pub unsafe fn destructor(&mut self) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2)))
        );
        assert!(
            ((self.id_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u32,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut *mut i32 {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2)))
        );
        if !((self.stats_).is_null()) {
            if (((*self.stats_).time_to_throw) >= ((*self.stats_).throw_after)) {
                std::process::abort();
            }
            (*self.stats_).time_to_throw.prefix_inc();
            (*self.stats_).alloc_count.prefix_inc();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_add(n)) as i32;
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut i32, mut s: u32) {
        assert!(
            ((self.data_) != (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2)))
        );
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size =
                (((*self.stats_).allocated_size as u32).wrapping_sub(s)) as i32;
        }
        unimplemented!();
    }
}
impl Default for test_allocator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            test_allocator_int_ptr_ {
                data_: 0_i32,
                id_: 0_i32,
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(Ctor_Tag, new_1, value)]
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
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_9)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_6)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8)) = 0_usize;
    }
}
pub static mut outstanding_bytes_6: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_7: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_8: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_9: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_10(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_11(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_12(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_11(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut i32) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
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
        mut n: isize,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
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
        unsafe { operator_eq_11(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_10();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const i32,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_13(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<min_pointer_int__std_integral_constant_unsigned_long__0__>())
        as i64 as isize);
}
pub unsafe fn operator_lt_14(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) < (y.ptr_));
}
pub unsafe fn operator_eq_15(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_16(
    mut x : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_15(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_int__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_int__std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn to_bool ( &self , ) -> bool { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_10 ( &mut self , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : isize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } pub unsafe fn operator_sub ( &self , mut n : isize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_sub_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::Ord for  min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn cmp(&self, other: &Self) -> std::cmp::Ordering { unsafe { if operator_lt_14(self.clone(), other.clone()) { std::cmp::Ordering::Less } else if operator_lt_14(other.clone(), self.clone()) { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Equal } }}}
impl std::cmp::PartialOrd for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }}
impl std::cmp::PartialEq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_15(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn cpp_inc(&mut self) { self.operator_inc_8(); } unsafe fn cpp_dec(&mut self) { self.operator_dec_10(); } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn new_1(mut _a0: min_allocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(min_allocator_int_, new_1, value)]
pub struct min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_int__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
impl min_allocator_min_pointer_int__std_integral_constant_unsigned_long__0___ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        let mut memory: *mut i32 = unimplemented!();
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in
                    0..((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in
                        0..((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_1(&mut _value)
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
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut *mut i32 {
        let mut memory: *mut *mut i32 = unimplemented!();
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in 0..((::std::mem::size_of::<*mut i32>() as u64)
                    .wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut i32, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in 0..((::std::mem::size_of::<*mut i32>() as u64)
                        .wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_1(&mut _value)
            });
        }
        unimplemented!();
    }
}
pub unsafe fn test0_17(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_18(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_19(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_20(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_21(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_22(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_23(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_24(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_25(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_26(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_27(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_28(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_29(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_30(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_31(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_32(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_33(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_34(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_35(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_36(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_37(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_38(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_39(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_40(
    mut s: Vec<i32>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i32>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_41(
    mut s: Vec<i64>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i64>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_42(
    mut s: Vec<i64>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i64>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_43(
    mut s: Vec<i64>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i64>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_44(
    mut s: Vec<i64>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i64>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_45(
    mut s: Vec<i64>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i64>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_46(
    mut s: Vec<i64>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<i64>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_47(
    mut s: Vec<f64>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<f64>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_48(
    mut s: Vec<f64>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<f64>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_49(
    mut s: Vec<f64>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<f64>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_50(
    mut s: Vec<f64>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<f64>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_51(
    mut s: Vec<f64>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<f64>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test0_52(
    mut s: Vec<f64>,
    mut p: FnPtr<Generic>,
    mut expected: Vec<f64>,
    mut expected_erased_count: usize,
) {
    const _: () = assert!(true, "");;
    assert!(((expected_erased_count) == (unimplemented!())));
    assert!(unimplemented!());
    const _: () = assert!(true, "");;
}
pub unsafe fn test_53() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (1));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (2));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (3));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (4));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return false;
    });
    (unsafe { test0_17(Vec::new(), is1.copy_from(), Vec::new(), 0_usize) });
    (unsafe { test0_17(unimplemented!(), is1.copy_from(), Vec::new(), 1_usize) });
    (unsafe { test0_18(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_17(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_18(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_19(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_17(unimplemented!(), is1.copy_from(), Vec::new(), 2_usize) });
    (unsafe { test0_19(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_17(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_18(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_19(unimplemented!(), is3.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_20(unimplemented!(), is4.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_17(unimplemented!(), is1.copy_from(), Vec::new(), 3_usize) });
    (unsafe { test0_18(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_17(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_18(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_19(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_17(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_18(unimplemented!(), is2.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_19(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_21(unimplemented!(), True.copy_from(), Vec::new(), 3_usize) });
    (unsafe {
        test0_22(
            unimplemented!(),
            False.copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
}
pub unsafe fn test_54() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (1));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (2));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (3));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (4));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return false;
    });
    (unsafe { test0_23(Vec::new(), is1.copy_from(), Vec::new(), 0_usize) });
    (unsafe { test0_23(unimplemented!(), is1.copy_from(), Vec::new(), 1_usize) });
    (unsafe { test0_24(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_23(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_24(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_25(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_23(unimplemented!(), is1.copy_from(), Vec::new(), 2_usize) });
    (unsafe { test0_25(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_23(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_24(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_25(unimplemented!(), is3.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_26(unimplemented!(), is4.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_23(unimplemented!(), is1.copy_from(), Vec::new(), 3_usize) });
    (unsafe { test0_24(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_23(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_24(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_25(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_23(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_24(unimplemented!(), is2.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_25(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_27(unimplemented!(), True.copy_from(), Vec::new(), 3_usize) });
    (unsafe {
        test0_28(
            unimplemented!(),
            False.copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
}
pub unsafe fn test_55() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (1));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (2));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (3));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (4));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return false;
    });
    (unsafe { test0_29(Vec::new(), is1.copy_from(), Vec::new(), 0_usize) });
    (unsafe { test0_29(unimplemented!(), is1.copy_from(), Vec::new(), 1_usize) });
    (unsafe { test0_30(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_29(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_30(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_31(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_29(unimplemented!(), is1.copy_from(), Vec::new(), 2_usize) });
    (unsafe { test0_31(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_29(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_30(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_31(unimplemented!(), is3.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_32(unimplemented!(), is4.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_29(unimplemented!(), is1.copy_from(), Vec::new(), 3_usize) });
    (unsafe { test0_30(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_29(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_30(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_31(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_29(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_30(unimplemented!(), is2.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_31(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_33(unimplemented!(), True.copy_from(), Vec::new(), 3_usize) });
    (unsafe {
        test0_34(
            unimplemented!(),
            False.copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
}
pub unsafe fn test_56() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (1));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (2));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (3));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i32| -> bool {
        return ((v) == (4));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i32| -> bool {
        return false;
    });
    (unsafe { test0_35(Vec::new(), is1.copy_from(), Vec::new(), 0_usize) });
    (unsafe { test0_35(unimplemented!(), is1.copy_from(), Vec::new(), 1_usize) });
    (unsafe { test0_36(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_35(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_36(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_37(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_35(unimplemented!(), is1.copy_from(), Vec::new(), 2_usize) });
    (unsafe { test0_37(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_35(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_36(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_37(unimplemented!(), is3.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_38(unimplemented!(), is4.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_35(unimplemented!(), is1.copy_from(), Vec::new(), 3_usize) });
    (unsafe { test0_36(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_35(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_36(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_37(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_35(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_36(unimplemented!(), is2.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_37(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_39(unimplemented!(), True.copy_from(), Vec::new(), 3_usize) });
    (unsafe {
        test0_40(
            unimplemented!(),
            False.copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
}
pub unsafe fn test_57() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i64| -> bool {
        return ((v) == (1_i64));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i64| -> bool {
        return ((v) == (2_i64));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i64| -> bool {
        return ((v) == (3_i64));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: i64| -> bool {
        return ((v) == (4_i64));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i64| -> bool {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: i64| -> bool {
        return false;
    });
    (unsafe { test0_41(Vec::new(), is1.copy_from(), Vec::new(), 0_usize) });
    (unsafe { test0_41(unimplemented!(), is1.copy_from(), Vec::new(), 1_usize) });
    (unsafe { test0_42(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_41(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_42(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_43(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_41(unimplemented!(), is1.copy_from(), Vec::new(), 2_usize) });
    (unsafe { test0_43(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_41(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_42(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_43(unimplemented!(), is3.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_44(unimplemented!(), is4.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_41(unimplemented!(), is1.copy_from(), Vec::new(), 3_usize) });
    (unsafe { test0_42(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_41(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_42(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_43(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_41(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_42(unimplemented!(), is2.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_43(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_45(unimplemented!(), True.copy_from(), Vec::new(), 3_usize) });
    (unsafe {
        test0_46(
            unimplemented!(),
            False.copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
}
pub unsafe fn test_58() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: f64| -> bool {
        return ((v) == (1_f64));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: f64| -> bool {
        return ((v) == (2_f64));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: f64| -> bool {
        return ((v) == (3_f64));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: f64| -> bool {
        return ((v) == (4_f64));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: f64| -> bool {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: f64| -> bool {
        return false;
    });
    (unsafe { test0_47(Vec::new(), is1.copy_from(), Vec::new(), 0_usize) });
    (unsafe { test0_47(unimplemented!(), is1.copy_from(), Vec::new(), 1_usize) });
    (unsafe { test0_48(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_47(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_48(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_49(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_47(unimplemented!(), is1.copy_from(), Vec::new(), 2_usize) });
    (unsafe { test0_49(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_47(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_48(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_49(unimplemented!(), is3.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_50(unimplemented!(), is4.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_47(unimplemented!(), is1.copy_from(), Vec::new(), 3_usize) });
    (unsafe { test0_48(unimplemented!(), is2.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_47(unimplemented!(), is1.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_48(unimplemented!(), is2.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_49(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_47(unimplemented!(), is1.copy_from(), unimplemented!(), 1_usize) });
    (unsafe { test0_48(unimplemented!(), is2.copy_from(), unimplemented!(), 2_usize) });
    (unsafe { test0_49(unimplemented!(), is3.copy_from(), unimplemented!(), 0_usize) });
    (unsafe { test0_51(unimplemented!(), True.copy_from(), Vec::new(), 3_usize) });
    (unsafe {
        test0_52(
            unimplemented!(),
            False.copy_from(),
            unimplemented!(),
            0_usize,
        )
    });
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_53() });
    (unsafe { test_54() });
    (unsafe { test_55() });
    (unsafe { test_56() });
    (unsafe { test_57() });
    (unsafe { test_58() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_2);
    std::cell::LazyCell::force(&*&raw const moved_value_3);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_6);
    std::cell::LazyCell::force(&*&raw const alloc_count_7);
    std::cell::LazyCell::force(&*&raw const dealloc_count_8);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_9);
}
