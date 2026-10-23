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
pub static mut destructed_value_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_4(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_5(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_4(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
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
pub struct test_allocator_int__rebind_std___hash_node_int__void_ptr__ {}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_int_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_int_ {}
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std___hash_node_int__void_ptr___rebind_std___hash_node_base_std___hash_node_int__void_ptr__ptr__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std___hash_node_int__void_ptr___rebind_std___hash_node_base_std___hash_node_int__void_ptr__ptr__ptr_
{}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_std___hash_node_int__void_ptr__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___hash_node_int__void_ptr__ {
    pub unsafe fn move_from(a: *mut test_allocator_std___hash_node_int__void_ptr__) -> Self {
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
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_3));
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
        mut _a1: Option<()>,
    ) -> *mut std___hash_node_int__void_ptr_ {
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
    pub unsafe fn deallocate(&mut self, mut p: *mut std___hash_node_int__void_ptr_, mut s: ()) {
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).alloc_count.prefix_dec();
            (*self.stats_).allocated_size = ((*self.stats_).allocated_size).wrapping_sub(s);
        }
        unimplemented!();
    }
}
impl Default for test_allocator_std___hash_node_int__void_ptr__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std___hash_node_int__void_ptr__ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_std___hash_node_base_std___hash_node_int__void_ptr__ptr__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___hash_node_base_std___hash_node_int__void_ptr__ptr__ {}
impl Default for test_allocator_std___hash_node_base_std___hash_node_int__void_ptr__ptr__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std___hash_node_base_std___hash_node_int__void_ptr__ptr__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, DestructorUnsafe)]
pub struct test_allocator_std___hash_node_base_std___hash_node_int__void_ptr__ptr__ptr_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___hash_node_base_std___hash_node_int__void_ptr__ptr__ptr_ {
    pub unsafe fn move_from(
        a: *mut test_allocator_std___hash_node_base_std___hash_node_int__void_ptr__ptr__ptr_,
    ) -> Self {
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
        (*a).id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut moved_value_3));
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_2));
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
        mut _a1: Option<()>,
    ) -> *mut *mut std___hash_node_base_std___hash_node_int__void_ptr__ptr_ {
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
        mut p: *mut *mut std___hash_node_base_std___hash_node_int__void_ptr__ptr_,
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
impl Default for test_allocator_std___hash_node_base_std___hash_node_int__void_ptr__ptr__ptr_ {
    fn default() -> Self {
        unsafe {
            test_allocator_std___hash_node_base_std___hash_node_int__void_ptr__ptr__ptr_ {
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_9)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_6)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_7)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_8)) = 0_();
    }
}
pub static mut outstanding_bytes_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_9: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(
        mut p : min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_ as ()),
        };
        this
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_2(
        mut p : min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_ as ()),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0___,
}
impl min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut p : min_pointer_void__std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : ( p . ptr_  as *mut std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0___ ) , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } }
pub unsafe fn operator_eq_10(
    mut x : min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_11(
    mut x : min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_10(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn new_3 ( mut p : min_pointer_void__std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : ( p . ptr_  as *mut std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ ) , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_10(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { ptr_ : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_index ( &self , mut n : () , ) -> *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_  . offset ( ( n ) as isize ) )  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl
    min_allocator_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0____
{
    pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__{
        return min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
{}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { return min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn make_12(mut vals: ()) -> () {
    let mut ret: () = unimplemented!();
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (*v);
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn make_13(mut vals: ()) -> () {
    let mut ret: () = unimplemented!();
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (*v);
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn make_14(mut vals: ()) -> () {
    let mut ret: () = unimplemented!();
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (*v);
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn make_15(mut vals: ()) -> () {
    let mut ret: () = unimplemented!();
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (*v);
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn make_16(mut vals: ()) -> () {
    let mut ret: () = unimplemented!();
    'loop_: for v in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut v: () = (*v);
        unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn test0_17(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_12(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_12(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_18(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_12(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_12(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_19(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_12(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_12(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_20(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_12(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_12(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_21(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_12(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_12(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_22(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_12(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_12(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_23(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_13(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_13(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_24(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_13(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_13(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_25(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_13(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_13(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_26(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_13(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_13(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_27(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_13(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_13(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_28(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_13(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_13(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_29(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_14(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_14(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_30(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_14(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_14(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_31(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_14(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_14(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_32(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_14(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_14(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_33(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_14(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_14(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_34(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_14(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_14(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_35(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_15(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_15(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_36(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_15(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_15(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_37(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_15(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_15(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_38(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_15(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_15(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_39(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_15(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_15(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_40(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_15(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_15(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_41(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_16(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_16(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_42(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_16(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_16(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_43(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_16(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_16(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_44(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_16(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_16(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_45(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_16(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_16(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test0_46(
    mut vals: (),
    mut p: FnPtr<Generic>,
    mut expected: (),
    mut expected_erased_count: (),
) {
    let mut s: () = (unsafe { make_16(vals.clone()) });
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut e: () = (unsafe { make_16(expected.clone()) });
    unimplemented!();
}
pub unsafe fn test_47() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (1_()));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (2_()));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (3_()));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (4_()));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: ()| -> () {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: ()| -> () {
        return false;
    });
    (unsafe { test0_17(unimplemented!(), is1.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_17(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_18(unimplemented!(), is2.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_17(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_18(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_19(unimplemented!(), is3.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_17(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_18(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_19(unimplemented!(), is3.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_20(unimplemented!(), is4.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_21(unimplemented!(), True.copy_from(), unimplemented!(), 3_()) });
    (unsafe { test0_22(unimplemented!(), False.copy_from(), unimplemented!(), 0_()) });
}
pub unsafe fn test_48() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (1_()));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (2_()));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (3_()));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (4_()));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: ()| -> () {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: ()| -> () {
        return false;
    });
    (unsafe { test0_23(unimplemented!(), is1.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_23(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_24(unimplemented!(), is2.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_23(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_24(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_25(unimplemented!(), is3.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_23(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_24(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_25(unimplemented!(), is3.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_26(unimplemented!(), is4.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_27(unimplemented!(), True.copy_from(), unimplemented!(), 3_()) });
    (unsafe { test0_28(unimplemented!(), False.copy_from(), unimplemented!(), 0_()) });
}
pub unsafe fn test_49() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (1_()));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (2_()));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (3_()));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (4_()));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: ()| -> () {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: ()| -> () {
        return false;
    });
    (unsafe { test0_29(unimplemented!(), is1.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_29(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_30(unimplemented!(), is2.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_29(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_30(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_31(unimplemented!(), is3.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_29(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_30(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_31(unimplemented!(), is3.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_32(unimplemented!(), is4.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_33(unimplemented!(), True.copy_from(), unimplemented!(), 3_()) });
    (unsafe { test0_34(unimplemented!(), False.copy_from(), unimplemented!(), 0_()) });
}
pub unsafe fn test_50() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (1_()));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (2_()));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (3_()));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (4_()));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: ()| -> () {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: ()| -> () {
        return false;
    });
    (unsafe { test0_35(unimplemented!(), is1.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_35(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_36(unimplemented!(), is2.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_35(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_36(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_37(unimplemented!(), is3.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_35(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_36(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_37(unimplemented!(), is3.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_38(unimplemented!(), is4.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_39(unimplemented!(), True.copy_from(), unimplemented!(), 3_()) });
    (unsafe { test0_40(unimplemented!(), False.copy_from(), unimplemented!(), 0_()) });
}
pub unsafe fn test_51() {
    let mut is1: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (1_()));
    });
    let mut is2: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (2_()));
    });
    let mut is3: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (3_()));
    });
    let mut is4: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut v: ()| -> () {
        return ((v) == (4_()));
    });
    let mut True: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: ()| -> () {
        return true;
    });
    let mut False: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut _a0: ()| -> () {
        return false;
    });
    (unsafe { test0_41(unimplemented!(), is1.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_41(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_42(unimplemented!(), is2.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_41(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_42(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_43(unimplemented!(), is3.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_41(unimplemented!(), is1.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_42(unimplemented!(), is2.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_43(unimplemented!(), is3.copy_from(), unimplemented!(), 1_()) });
    (unsafe { test0_44(unimplemented!(), is4.copy_from(), unimplemented!(), 0_()) });
    (unsafe { test0_45(unimplemented!(), True.copy_from(), unimplemented!(), 3_()) });
    (unsafe { test0_46(unimplemented!(), False.copy_from(), unimplemented!(), 0_()) });
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
    (unsafe { test_47() });
    (unsafe { test_48() });
    (unsafe { test_49() });
    (unsafe { test_50() });
    (unsafe { test_51() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_2);
    std::cell::LazyCell::force(&*&raw const moved_value_3);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_6);
    std::cell::LazyCell::force(&*&raw const alloc_count_7);
    std::cell::LazyCell::force(&*&raw const dealloc_count_8);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_9);
}
