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
impl test_allocator_std_pair_const_int__double__ {
    pub unsafe fn new_1(
        a: *const test_allocator_std___tree_node_std___value_type_int__double___void_ptr__,
    ) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__double___rebind_std_pair_const_int__double__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std_pair_const_int__double___rebind_std___tree_node_std___value_type_int__double___void_ptr__
{}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std_pair_const_int__double__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std_pair_const_int__double__ {
    pub unsafe fn new_2(mut data: (), mut id: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_std_pair_const_int__double__) -> Self {
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
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
}
impl Clone for test_allocator_std_pair_const_int__double__ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_std_pair_const_int__double__::copy_from(
                self as *const test_allocator_std_pair_const_int__double__,
            )
        }
    }
}
impl Default for test_allocator_std_pair_const_int__double__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std_pair_const_int__double__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl test_allocator_std___tree_node_std___value_type_int__double___void_ptr__ {
    pub unsafe fn new_1(a: *const test_allocator_std_pair_const_int__double__) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_std___tree_node_std___value_type_int__double___void_ptr___rebind_std___tree_node_base_void_ptr__
{}
#[repr(C)]
#[derive(VaArg, FnPtrArg, DestructorUnsafe)]
pub struct test_allocator_std___tree_node_std___value_type_int__double___void_ptr__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___tree_node_std___value_type_int__double___void_ptr__ {
    pub unsafe fn copy_from(
        a: *const test_allocator_std___tree_node_std___value_type_int__double___void_ptr__,
    ) -> Self {
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
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
        mut _a1: Option<()>,
    ) -> *mut std___tree_node_std___value_type_int__double___void_ptr_ {
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
        mut p: *mut std___tree_node_std___value_type_int__double___void_ptr_,
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
impl Clone for test_allocator_std___tree_node_std___value_type_int__double___void_ptr__ {
    fn clone(&self) -> Self {
        unsafe {
            test_allocator_std___tree_node_std___value_type_int__double___void_ptr__::copy_from(self as *const test_allocator_std___tree_node_std___value_type_int__double___void_ptr__)
        }
    }
}
impl Default for test_allocator_std___tree_node_std___value_type_int__double___void_ptr__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std___tree_node_std___value_type_int__double___void_ptr__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct test_allocator_std___tree_node_base_void_ptr__ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_std___tree_node_base_void_ptr__ {}
impl Default for test_allocator_std___tree_node_base_void_ptr__ {
    fn default() -> Self {
        unsafe {
            test_allocator_std___tree_node_base_void_ptr__ {
                data_: unimplemented!(),
                id_: unimplemented!(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
impl other_allocator_std_pair_const_int__double__ {
    pub unsafe fn new_1(
        a: *const other_allocator_std___tree_node_std___value_type_int__double___void_ptr__,
    ) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
pub unsafe fn operator_eq_4(
    x: *const other_allocator_std_pair_const_int__double__,
    y: *const other_allocator_std_pair_const_int__double__,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_std_pair_const_int__double__ {
    data_: (),
}
impl other_allocator_std_pair_const_int__double__ {
    pub unsafe fn new_2(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        this
    }
}
impl std::cmp::PartialEq for other_allocator_std_pair_const_int__double__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_4(
                self as *const other_allocator_std_pair_const_int__double__,
                other as *const other_allocator_std_pair_const_int__double__,
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_std_pair_const_int__double__ {}
impl Default for other_allocator_std_pair_const_int__double__ {
    fn default() -> Self {
        unsafe {
            other_allocator_std_pair_const_int__double__ {
                data_: unimplemented!(),
            }
        }
    }
}
impl other_allocator_std___tree_node_std___value_type_int__double___void_ptr__ {
    pub unsafe fn new_1(a: *const other_allocator_std_pair_const_int__double__) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_std___tree_node_std___value_type_int__double___void_ptr__ {
    data_: (),
}
impl other_allocator_std___tree_node_std___value_type_int__double___void_ptr__ {
    pub unsafe fn new_2(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        this
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> *mut std___tree_node_std___value_type_int__double___void_ptr_ {
        return unimplemented!();
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: *mut std___tree_node_std___value_type_int__double___void_ptr_,
        mut s: (),
    ) {
        unimplemented!();
    }
    pub unsafe fn select_on_container_copy_construction(
        &self,
    ) -> other_allocator_std___tree_node_std___value_type_int__double___void_ptr__ {
        return other_allocator_std___tree_node_std___value_type_int__double___void_ptr__::new_2({
            -2_()
        });
    }
}
impl Default for other_allocator_std___tree_node_std___value_type_int__double___void_ptr__ {
    fn default() -> Self {
        unsafe {
            other_allocator_std___tree_node_std___value_type_int__double___void_ptr__ {
                data_: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_std___tree_node_base_void_ptr__ {
    data_: (),
}
impl other_allocator_std___tree_node_base_void_ptr__ {}
impl Default for other_allocator_std___tree_node_base_void_ptr__ {
    fn default() -> Self {
        unsafe {
            other_allocator_std___tree_node_base_void_ptr__ {
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
pub unsafe fn operator_eq_5(a: *const test_less_int_, b: *const test_less_int_) -> () {
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
            operator_eq_5(
                self as *const test_less_int_,
                other as *const test_less_int_,
            )
        }
    }
}
impl std::cmp::Eq for test_less_int_ {}
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
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_10(
    mut x : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_11(
    mut x : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_10(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_10(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_12(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_13(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_12(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___,
}
impl min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_12(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_14(
    mut x : min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_14(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
impl min_allocator_std_pair_const_int__double__ {
    pub unsafe fn new_1(
        mut _a0 : min_allocator_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0____,
    ) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn operator_eq_15(
    mut _a0: min_allocator_std_pair_const_int__double__,
    mut _a1: min_allocator_std_pair_const_int__double__,
) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__double__ {}
impl std::cmp::PartialEq for min_allocator_std_pair_const_int__double__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_15(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_std_pair_const_int__double__ {}
impl min_allocator_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0____ { pub unsafe fn new_1 ( mut _a0 : min_allocator_std_pair_const_int__double__ , ) -> Self { let mut this = Self { } ;
 this } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl min_allocator_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0____ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl min_allocator_std_pair_const_int__double__ {}
impl min_allocator_std___tree_node_std___value_type_int__double___min_pointer_void__std_integral_constant_unsigned_long__0____ { }
pub unsafe fn test_16() -> () {
    {
        {
            let mut m1: () = unimplemented!();
            let mut m2: () = unimplemented!();
            let mut m1_save: () = unimplemented!();
            let mut m2_save: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut ar2: [(); 8] = [
                {
                    let mut __tmp_0: () = 5_();
                    let mut __tmp_1: () = 5_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_0 }, { &mut __tmp_1 })
                },
                {
                    let mut __tmp_2: () = 6_();
                    let mut __tmp_3: () = 6_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_2 }, { &mut __tmp_3 })
                },
                {
                    let mut __tmp_4: () = 7_();
                    let mut __tmp_5: () = 7_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_4 }, { &mut __tmp_5 })
                },
                {
                    let mut __tmp_6: () = 8_();
                    let mut __tmp_7: () = 8_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_6 }, { &mut __tmp_7 })
                },
                {
                    let mut __tmp_8: () = 9_();
                    let mut __tmp_9: () = 9_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_8 }, { &mut __tmp_9 })
                },
                {
                    let mut __tmp_10: () = 10_();
                    let mut __tmp_11: () = 10_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_10 }, { &mut __tmp_11 })
                },
                {
                    let mut __tmp_12: () = 11_();
                    let mut __tmp_13: () = 11_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_12 }, { &mut __tmp_13 })
                },
                {
                    let mut __tmp_14: () = 12_();
                    let mut __tmp_15: () = 12_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_14 }, { &mut __tmp_15 })
                },
            ];
            let mut m1: () = unimplemented!();
            let mut m2: () = unimplemented!();
            let mut m1_save: () = unimplemented!();
            let mut m2_save: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut ar1: [(); 4] = [
                {
                    let mut __tmp_16: () = 1_();
                    let mut __tmp_17: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_16 }, { &mut __tmp_17 })
                },
                {
                    let mut __tmp_18: () = 2_();
                    let mut __tmp_19: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_18 }, { &mut __tmp_19 })
                },
                {
                    let mut __tmp_20: () = 3_();
                    let mut __tmp_21: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_20 }, { &mut __tmp_21 })
                },
                {
                    let mut __tmp_22: () = 4_();
                    let mut __tmp_23: () = 4_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_22 }, { &mut __tmp_23 })
                },
            ];
            let mut m1: () = unimplemented!();
            let mut m2: () = unimplemented!();
            let mut m1_save: () = unimplemented!();
            let mut m2_save: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut ar1: [(); 4] = [
                {
                    let mut __tmp_24: () = 1_();
                    let mut __tmp_25: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_24 }, { &mut __tmp_25 })
                },
                {
                    let mut __tmp_26: () = 2_();
                    let mut __tmp_27: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_26 }, { &mut __tmp_27 })
                },
                {
                    let mut __tmp_28: () = 3_();
                    let mut __tmp_29: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_28 }, { &mut __tmp_29 })
                },
                {
                    let mut __tmp_30: () = 4_();
                    let mut __tmp_31: () = 4_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_30 }, { &mut __tmp_31 })
                },
            ];
            let mut ar2: [(); 8] = [
                {
                    let mut __tmp_32: () = 5_();
                    let mut __tmp_33: () = 5_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_32 }, { &mut __tmp_33 })
                },
                {
                    let mut __tmp_34: () = 6_();
                    let mut __tmp_35: () = 6_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_34 }, { &mut __tmp_35 })
                },
                {
                    let mut __tmp_36: () = 7_();
                    let mut __tmp_37: () = 7_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_36 }, { &mut __tmp_37 })
                },
                {
                    let mut __tmp_38: () = 8_();
                    let mut __tmp_39: () = 8_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_38 }, { &mut __tmp_39 })
                },
                {
                    let mut __tmp_40: () = 9_();
                    let mut __tmp_41: () = 9_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_40 }, { &mut __tmp_41 })
                },
                {
                    let mut __tmp_42: () = 10_();
                    let mut __tmp_43: () = 10_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_42 }, { &mut __tmp_43 })
                },
                {
                    let mut __tmp_44: () = 11_();
                    let mut __tmp_45: () = 11_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_44 }, { &mut __tmp_45 })
                },
                {
                    let mut __tmp_46: () = 12_();
                    let mut __tmp_47: () = 12_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_46 }, { &mut __tmp_47 })
                },
            ];
            let mut m1: () = unimplemented!();
            let mut m2: () = unimplemented!();
            let mut m1_save: () = unimplemented!();
            let mut m2_save: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut ar1: [(); 4] = [
            {
                let mut __tmp_48: () = 1_();
                let mut __tmp_49: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_48 }, { &mut __tmp_49 })
            },
            {
                let mut __tmp_50: () = 2_();
                let mut __tmp_51: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_50 }, { &mut __tmp_51 })
            },
            {
                let mut __tmp_52: () = 3_();
                let mut __tmp_53: () = 3_();
                std_pair_const_int__double_::new_1({ &mut __tmp_52 }, { &mut __tmp_53 })
            },
            {
                let mut __tmp_54: () = 4_();
                let mut __tmp_55: () = 4_();
                std_pair_const_int__double_::new_1({ &mut __tmp_54 }, { &mut __tmp_55 })
            },
        ];
        let mut ar2: [(); 8] = [
            {
                let mut __tmp_56: () = 5_();
                let mut __tmp_57: () = 5_();
                std_pair_const_int__double_::new_1({ &mut __tmp_56 }, { &mut __tmp_57 })
            },
            {
                let mut __tmp_58: () = 6_();
                let mut __tmp_59: () = 6_();
                std_pair_const_int__double_::new_1({ &mut __tmp_58 }, { &mut __tmp_59 })
            },
            {
                let mut __tmp_60: () = 7_();
                let mut __tmp_61: () = 7_();
                std_pair_const_int__double_::new_1({ &mut __tmp_60 }, { &mut __tmp_61 })
            },
            {
                let mut __tmp_62: () = 8_();
                let mut __tmp_63: () = 8_();
                std_pair_const_int__double_::new_1({ &mut __tmp_62 }, { &mut __tmp_63 })
            },
            {
                let mut __tmp_64: () = 9_();
                let mut __tmp_65: () = 9_();
                std_pair_const_int__double_::new_1({ &mut __tmp_64 }, { &mut __tmp_65 })
            },
            {
                let mut __tmp_66: () = 10_();
                let mut __tmp_67: () = 10_();
                std_pair_const_int__double_::new_1({ &mut __tmp_66 }, { &mut __tmp_67 })
            },
            {
                let mut __tmp_68: () = 11_();
                let mut __tmp_69: () = 11_();
                std_pair_const_int__double_::new_1({ &mut __tmp_68 }, { &mut __tmp_69 })
            },
            {
                let mut __tmp_70: () = 12_();
                let mut __tmp_71: () = 12_();
                std_pair_const_int__double_::new_1({ &mut __tmp_70 }, { &mut __tmp_71 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut m2: () = unimplemented!();
        let mut m1_save: () = unimplemented!();
        let mut m2_save: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ar1: [(); 4] = [
            {
                let mut __tmp_72: () = 1_();
                let mut __tmp_73: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_72 }, { &mut __tmp_73 })
            },
            {
                let mut __tmp_74: () = 2_();
                let mut __tmp_75: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_74 }, { &mut __tmp_75 })
            },
            {
                let mut __tmp_76: () = 3_();
                let mut __tmp_77: () = 3_();
                std_pair_const_int__double_::new_1({ &mut __tmp_76 }, { &mut __tmp_77 })
            },
            {
                let mut __tmp_78: () = 4_();
                let mut __tmp_79: () = 4_();
                std_pair_const_int__double_::new_1({ &mut __tmp_78 }, { &mut __tmp_79 })
            },
        ];
        let mut ar2: [(); 8] = [
            {
                let mut __tmp_80: () = 5_();
                let mut __tmp_81: () = 5_();
                std_pair_const_int__double_::new_1({ &mut __tmp_80 }, { &mut __tmp_81 })
            },
            {
                let mut __tmp_82: () = 6_();
                let mut __tmp_83: () = 6_();
                std_pair_const_int__double_::new_1({ &mut __tmp_82 }, { &mut __tmp_83 })
            },
            {
                let mut __tmp_84: () = 7_();
                let mut __tmp_85: () = 7_();
                std_pair_const_int__double_::new_1({ &mut __tmp_84 }, { &mut __tmp_85 })
            },
            {
                let mut __tmp_86: () = 8_();
                let mut __tmp_87: () = 8_();
                std_pair_const_int__double_::new_1({ &mut __tmp_86 }, { &mut __tmp_87 })
            },
            {
                let mut __tmp_88: () = 9_();
                let mut __tmp_89: () = 9_();
                std_pair_const_int__double_::new_1({ &mut __tmp_88 }, { &mut __tmp_89 })
            },
            {
                let mut __tmp_90: () = 10_();
                let mut __tmp_91: () = 10_();
                std_pair_const_int__double_::new_1({ &mut __tmp_90 }, { &mut __tmp_91 })
            },
            {
                let mut __tmp_92: () = 11_();
                let mut __tmp_93: () = 11_();
                std_pair_const_int__double_::new_1({ &mut __tmp_92 }, { &mut __tmp_93 })
            },
            {
                let mut __tmp_94: () = 12_();
                let mut __tmp_95: () = 12_();
                std_pair_const_int__double_::new_1({ &mut __tmp_94 }, { &mut __tmp_95 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut m2: () = unimplemented!();
        let mut m1_save: () = unimplemented!();
        let mut m2_save: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut m1: () = unimplemented!();
            let mut m2: () = unimplemented!();
            let mut m1_save: () = unimplemented!();
            let mut m2_save: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut ar2: [(); 8] = [
                {
                    let mut __tmp_96: () = 5_();
                    let mut __tmp_97: () = 5_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_96 }, { &mut __tmp_97 })
                },
                {
                    let mut __tmp_98: () = 6_();
                    let mut __tmp_99: () = 6_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_98 }, { &mut __tmp_99 })
                },
                {
                    let mut __tmp_100: () = 7_();
                    let mut __tmp_101: () = 7_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_100 }, { &mut __tmp_101 })
                },
                {
                    let mut __tmp_102: () = 8_();
                    let mut __tmp_103: () = 8_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_102 }, { &mut __tmp_103 })
                },
                {
                    let mut __tmp_104: () = 9_();
                    let mut __tmp_105: () = 9_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_104 }, { &mut __tmp_105 })
                },
                {
                    let mut __tmp_106: () = 10_();
                    let mut __tmp_107: () = 10_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_106 }, { &mut __tmp_107 })
                },
                {
                    let mut __tmp_108: () = 11_();
                    let mut __tmp_109: () = 11_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_108 }, { &mut __tmp_109 })
                },
                {
                    let mut __tmp_110: () = 12_();
                    let mut __tmp_111: () = 12_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_110 }, { &mut __tmp_111 })
                },
            ];
            let mut m1: () = unimplemented!();
            let mut m2: () = unimplemented!();
            let mut m1_save: () = unimplemented!();
            let mut m2_save: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut ar1: [(); 4] = [
                {
                    let mut __tmp_112: () = 1_();
                    let mut __tmp_113: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_112 }, { &mut __tmp_113 })
                },
                {
                    let mut __tmp_114: () = 2_();
                    let mut __tmp_115: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_114 }, { &mut __tmp_115 })
                },
                {
                    let mut __tmp_116: () = 3_();
                    let mut __tmp_117: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_116 }, { &mut __tmp_117 })
                },
                {
                    let mut __tmp_118: () = 4_();
                    let mut __tmp_119: () = 4_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_118 }, { &mut __tmp_119 })
                },
            ];
            let mut m1: () = unimplemented!();
            let mut m2: () = unimplemented!();
            let mut m1_save: () = unimplemented!();
            let mut m2_save: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut ar1: [(); 4] = [
                {
                    let mut __tmp_120: () = 1_();
                    let mut __tmp_121: () = 1_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_120 }, { &mut __tmp_121 })
                },
                {
                    let mut __tmp_122: () = 2_();
                    let mut __tmp_123: () = 2_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_122 }, { &mut __tmp_123 })
                },
                {
                    let mut __tmp_124: () = 3_();
                    let mut __tmp_125: () = 3_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_124 }, { &mut __tmp_125 })
                },
                {
                    let mut __tmp_126: () = 4_();
                    let mut __tmp_127: () = 4_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_126 }, { &mut __tmp_127 })
                },
            ];
            let mut ar2: [(); 8] = [
                {
                    let mut __tmp_128: () = 5_();
                    let mut __tmp_129: () = 5_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_128 }, { &mut __tmp_129 })
                },
                {
                    let mut __tmp_130: () = 6_();
                    let mut __tmp_131: () = 6_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_130 }, { &mut __tmp_131 })
                },
                {
                    let mut __tmp_132: () = 7_();
                    let mut __tmp_133: () = 7_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_132 }, { &mut __tmp_133 })
                },
                {
                    let mut __tmp_134: () = 8_();
                    let mut __tmp_135: () = 8_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_134 }, { &mut __tmp_135 })
                },
                {
                    let mut __tmp_136: () = 9_();
                    let mut __tmp_137: () = 9_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_136 }, { &mut __tmp_137 })
                },
                {
                    let mut __tmp_138: () = 10_();
                    let mut __tmp_139: () = 10_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_138 }, { &mut __tmp_139 })
                },
                {
                    let mut __tmp_140: () = 11_();
                    let mut __tmp_141: () = 11_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_140 }, { &mut __tmp_141 })
                },
                {
                    let mut __tmp_142: () = 12_();
                    let mut __tmp_143: () = 12_();
                    std_pair_const_int__double_::new_1({ &mut __tmp_142 }, { &mut __tmp_143 })
                },
            ];
            let mut m1: () = unimplemented!();
            let mut m2: () = unimplemented!();
            let mut m1_save: () = unimplemented!();
            let mut m2_save: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut ar1: [(); 4] = [
            {
                let mut __tmp_144: () = 1_();
                let mut __tmp_145: () = 1_();
                std_pair_const_int__double_::new_1({ &mut __tmp_144 }, { &mut __tmp_145 })
            },
            {
                let mut __tmp_146: () = 2_();
                let mut __tmp_147: () = 2_();
                std_pair_const_int__double_::new_1({ &mut __tmp_146 }, { &mut __tmp_147 })
            },
            {
                let mut __tmp_148: () = 3_();
                let mut __tmp_149: () = 3_();
                std_pair_const_int__double_::new_1({ &mut __tmp_148 }, { &mut __tmp_149 })
            },
            {
                let mut __tmp_150: () = 4_();
                let mut __tmp_151: () = 4_();
                std_pair_const_int__double_::new_1({ &mut __tmp_150 }, { &mut __tmp_151 })
            },
        ];
        let mut ar2: [(); 8] = [
            {
                let mut __tmp_152: () = 5_();
                let mut __tmp_153: () = 5_();
                std_pair_const_int__double_::new_1({ &mut __tmp_152 }, { &mut __tmp_153 })
            },
            {
                let mut __tmp_154: () = 6_();
                let mut __tmp_155: () = 6_();
                std_pair_const_int__double_::new_1({ &mut __tmp_154 }, { &mut __tmp_155 })
            },
            {
                let mut __tmp_156: () = 7_();
                let mut __tmp_157: () = 7_();
                std_pair_const_int__double_::new_1({ &mut __tmp_156 }, { &mut __tmp_157 })
            },
            {
                let mut __tmp_158: () = 8_();
                let mut __tmp_159: () = 8_();
                std_pair_const_int__double_::new_1({ &mut __tmp_158 }, { &mut __tmp_159 })
            },
            {
                let mut __tmp_160: () = 9_();
                let mut __tmp_161: () = 9_();
                std_pair_const_int__double_::new_1({ &mut __tmp_160 }, { &mut __tmp_161 })
            },
            {
                let mut __tmp_162: () = 10_();
                let mut __tmp_163: () = 10_();
                std_pair_const_int__double_::new_1({ &mut __tmp_162 }, { &mut __tmp_163 })
            },
            {
                let mut __tmp_164: () = 11_();
                let mut __tmp_165: () = 11_();
                std_pair_const_int__double_::new_1({ &mut __tmp_164 }, { &mut __tmp_165 })
            },
            {
                let mut __tmp_166: () = 12_();
                let mut __tmp_167: () = 12_();
                std_pair_const_int__double_::new_1({ &mut __tmp_166 }, { &mut __tmp_167 })
            },
        ];
        let mut m1: () = unimplemented!();
        let mut m2: () = unimplemented!();
        let mut m1_save: () = unimplemented!();
        let mut m2_save: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_16() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_0);
    std::cell::LazyCell::force(&*&raw const moved_value_1);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_6);
    std::cell::LazyCell::force(&*&raw const alloc_count_7);
    std::cell::LazyCell::force(&*&raw const dealloc_count_8);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_9);
}
