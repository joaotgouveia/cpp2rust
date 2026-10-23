extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(a: *const test_less_int_, b: *const test_less_int_) -> () {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_less_int_ {
    pub data_: (),
}
impl test_less_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { data_: 0_() };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
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
impl Default for test_less_int_ {
    fn default() -> Self {
        unsafe { test_less_int_::new_1() }
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_9(
    mut x : min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_10(
    mut x : min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_9(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { ptr_ : *mut std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0___ , }
impl min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0___ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0___ { return self . ptr_   ;
 } pub unsafe fn pointer_to ( t : *mut std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0___ , ) -> min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } }
impl std::cmp::PartialEq for min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_9(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_11(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_12(
    mut x : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_11(_x, y)
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
impl std::cmp::PartialEq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_11(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_13(
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
impl std::cmp::PartialEq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_13(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std___tree_end_node_min_pointer_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__int__ {}
impl min_allocator_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0____ { pub unsafe fn new_1 ( mut _a0 : min_allocator_std_pair_const_int__int__ , ) -> Self { let mut this = Self { } ;
 this } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl min_allocator_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0____ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std___tree_node_base_min_pointer_void__std_integral_constant_unsigned_long__0____
{}
impl min_allocator_std___tree_node_std___value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0____ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct tracking_allocator_std_pair_const_int__int__ {
    allocs_: *mut (),
}
impl tracking_allocator_std_pair_const_int__int__ {
    pub unsafe fn new(allocs: *mut ()) -> Self {
        let mut this = Self { allocs_: (allocs) };
        this
    }
}
impl tracking_allocator_std___tree_node_std___value_type_int__int___void_ptr__ {
    pub unsafe fn new_1(other: *const tracking_allocator_std_pair_const_int__int__) -> Self {
        let mut this = Self {
            allocs_: (*other).allocs_,
        };
        this
    }
}
pub unsafe fn operator_ne_14(
    lhs: *const tracking_allocator_std___tree_node_std___value_type_int__int___void_ptr__,
    rhs: *const tracking_allocator_std___tree_node_std___value_type_int__int___void_ptr__,
) -> () {
    return (((*lhs).allocs_) != ((*rhs).allocs_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct tracking_allocator_std___tree_node_std___value_type_int__int___void_ptr__ {
    allocs_: *mut (),
}
impl tracking_allocator_std___tree_node_std___value_type_int__int___void_ptr__ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> *mut std___tree_node_std___value_type_int__int___void_ptr_ {
        let mut allocation: *mut std___tree_node_std___value_type_int__int___void_ptr_ =
            unimplemented!();
        unimplemented!();
        return allocation;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut ptr: *mut std___tree_node_std___value_type_int__int___void_ptr_,
        mut n: (),
    ) {
        let mut res: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct tracking_allocator_std___tree_node_base_void_ptr__ {
    allocs_: *mut (),
}
impl tracking_allocator_std___tree_node_base_void_ptr__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoOp {}
impl NoOp {
    pub unsafe fn operator_call(&mut self) {}
}
pub unsafe fn test_alloc_15(
    lhs_alloc: Option<*const ()>,
    rhs_alloc: Option<*const ()>,
    mut check_alloc_invariant: Option<NoOp>,
) {
    let mut __lhs_alloc_default: Option<()> = None;
    let mut lhs_alloc: *const () = lhs_alloc
        .unwrap_or_else(|| unsafe { __lhs_alloc_default.insert(unimplemented!()) as *const () });
    let mut __rhs_alloc_default: Option<()> = None;
    let mut rhs_alloc: *const () = rhs_alloc
        .unwrap_or_else(|| unsafe { __rhs_alloc_default.insert(unimplemented!()) as *const () });
    let mut check_alloc_invariant: NoOp =
        check_alloc_invariant.unwrap_or_else(|| unsafe { <NoOp>::default() });
    {
        {
            let mut arr: [(); 3] = [
                {
                    let mut __tmp_0: () = 1_();
                    let mut __tmp_1: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_0 }, { &mut __tmp_1 })
                },
                {
                    let mut __tmp_2: () = 2_();
                    let mut __tmp_3: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_2 }, { &mut __tmp_3 })
                },
                {
                    let mut __tmp_4: () = 2_();
                    let mut __tmp_5: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_4 }, { &mut __tmp_5 })
                },
            ];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(); 3] = [
                {
                    let mut __tmp_6: () = 1_();
                    let mut __tmp_7: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_6 }, { &mut __tmp_7 })
                },
                {
                    let mut __tmp_8: () = 2_();
                    let mut __tmp_9: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_8 }, { &mut __tmp_9 })
                },
                {
                    let mut __tmp_10: () = 2_();
                    let mut __tmp_11: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_10 }, { &mut __tmp_11 })
                },
            ];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [
                {
                    let mut __tmp_12: () = 1_();
                    let mut __tmp_13: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_12 }, { &mut __tmp_13 })
                },
                {
                    let mut __tmp_14: () = 2_();
                    let mut __tmp_15: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_14 }, { &mut __tmp_15 })
                },
                {
                    let mut __tmp_16: () = 2_();
                    let mut __tmp_17: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_16 }, { &mut __tmp_17 })
                },
            ];
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [
                {
                    let mut __tmp_18: () = 1_();
                    let mut __tmp_19: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_18 }, { &mut __tmp_19 })
                },
                {
                    let mut __tmp_20: () = 2_();
                    let mut __tmp_21: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_20 }, { &mut __tmp_21 })
                },
                {
                    let mut __tmp_22: () = 2_();
                    let mut __tmp_23: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_22 }, { &mut __tmp_23 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 2] = [
                {
                    let mut __tmp_24: () = 4_();
                    let mut __tmp_25: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_24 }, { &mut __tmp_25 })
                },
                {
                    let mut __tmp_26: () = 5_();
                    let mut __tmp_27: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_26 }, { &mut __tmp_27 })
                },
            ];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [
                {
                    let mut __tmp_28: () = 1_();
                    let mut __tmp_29: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_28 }, { &mut __tmp_29 })
                },
                {
                    let mut __tmp_30: () = 2_();
                    let mut __tmp_31: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_30 }, { &mut __tmp_31 })
                },
                {
                    let mut __tmp_32: () = 2_();
                    let mut __tmp_33: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_32 }, { &mut __tmp_33 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 3] = [
                {
                    let mut __tmp_34: () = 4_();
                    let mut __tmp_35: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_34 }, { &mut __tmp_35 })
                },
                {
                    let mut __tmp_36: () = 5_();
                    let mut __tmp_37: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_36 }, { &mut __tmp_37 })
                },
                {
                    let mut __tmp_38: () = 6_();
                    let mut __tmp_39: () = 0_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_38 }, { &mut __tmp_39 })
                },
            ];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [
                {
                    let mut __tmp_40: () = 1_();
                    let mut __tmp_41: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_40 }, { &mut __tmp_41 })
                },
                {
                    let mut __tmp_42: () = 2_();
                    let mut __tmp_43: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_42 }, { &mut __tmp_43 })
                },
                {
                    let mut __tmp_44: () = 2_();
                    let mut __tmp_45: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_44 }, { &mut __tmp_45 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 4] = [
                {
                    let mut __tmp_46: () = 4_();
                    let mut __tmp_47: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_46 }, { &mut __tmp_47 })
                },
                {
                    let mut __tmp_48: () = 5_();
                    let mut __tmp_49: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_48 }, { &mut __tmp_49 })
                },
                {
                    let mut __tmp_50: () = 6_();
                    let mut __tmp_51: () = 0_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_50 }, { &mut __tmp_51 })
                },
                {
                    let mut __tmp_52: () = 7_();
                    let mut __tmp_53: () = 9_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_52 }, { &mut __tmp_53 })
                },
            ];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
    {
        let mut orig: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (50_())) {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        let mut copy: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        let mut iter: () = unimplemented!();
        'loop_: while unimplemented!() {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
}
pub unsafe fn test_alloc_16(
    lhs_alloc: Option<*const min_allocator_std_pair_const_int__int__>,
    rhs_alloc: Option<*const min_allocator_std_pair_const_int__int__>,
    mut check_alloc_invariant: Option<NoOp>,
) {
    let mut __lhs_alloc_default: Option<min_allocator_std_pair_const_int__int__> = None;
    let mut lhs_alloc: *const min_allocator_std_pair_const_int__int__ =
        lhs_alloc.unwrap_or_else(|| unsafe {
            __lhs_alloc_default.insert(<min_allocator_std_pair_const_int__int__>::default())
                as *const min_allocator_std_pair_const_int__int__
        });
    let mut __rhs_alloc_default: Option<min_allocator_std_pair_const_int__int__> = None;
    let mut rhs_alloc: *const min_allocator_std_pair_const_int__int__ =
        rhs_alloc.unwrap_or_else(|| unsafe {
            __rhs_alloc_default.insert(<min_allocator_std_pair_const_int__int__>::default())
                as *const min_allocator_std_pair_const_int__int__
        });
    let mut check_alloc_invariant: NoOp =
        check_alloc_invariant.unwrap_or_else(|| unsafe { <NoOp>::default() });
    {
        {
            let mut arr: [(); 3] = [
                {
                    let mut __tmp_54: () = 1_();
                    let mut __tmp_55: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_54 }, { &mut __tmp_55 })
                },
                {
                    let mut __tmp_56: () = 2_();
                    let mut __tmp_57: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_56 }, { &mut __tmp_57 })
                },
                {
                    let mut __tmp_58: () = 2_();
                    let mut __tmp_59: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_58 }, { &mut __tmp_59 })
                },
            ];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(); 3] = [
                {
                    let mut __tmp_60: () = 1_();
                    let mut __tmp_61: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_60 }, { &mut __tmp_61 })
                },
                {
                    let mut __tmp_62: () = 2_();
                    let mut __tmp_63: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_62 }, { &mut __tmp_63 })
                },
                {
                    let mut __tmp_64: () = 2_();
                    let mut __tmp_65: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_64 }, { &mut __tmp_65 })
                },
            ];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [
                {
                    let mut __tmp_66: () = 1_();
                    let mut __tmp_67: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_66 }, { &mut __tmp_67 })
                },
                {
                    let mut __tmp_68: () = 2_();
                    let mut __tmp_69: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_68 }, { &mut __tmp_69 })
                },
                {
                    let mut __tmp_70: () = 2_();
                    let mut __tmp_71: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_70 }, { &mut __tmp_71 })
                },
            ];
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [
                {
                    let mut __tmp_72: () = 1_();
                    let mut __tmp_73: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_72 }, { &mut __tmp_73 })
                },
                {
                    let mut __tmp_74: () = 2_();
                    let mut __tmp_75: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_74 }, { &mut __tmp_75 })
                },
                {
                    let mut __tmp_76: () = 2_();
                    let mut __tmp_77: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_76 }, { &mut __tmp_77 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 2] = [
                {
                    let mut __tmp_78: () = 4_();
                    let mut __tmp_79: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_78 }, { &mut __tmp_79 })
                },
                {
                    let mut __tmp_80: () = 5_();
                    let mut __tmp_81: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_80 }, { &mut __tmp_81 })
                },
            ];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [
                {
                    let mut __tmp_82: () = 1_();
                    let mut __tmp_83: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_82 }, { &mut __tmp_83 })
                },
                {
                    let mut __tmp_84: () = 2_();
                    let mut __tmp_85: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_84 }, { &mut __tmp_85 })
                },
                {
                    let mut __tmp_86: () = 2_();
                    let mut __tmp_87: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_86 }, { &mut __tmp_87 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 3] = [
                {
                    let mut __tmp_88: () = 4_();
                    let mut __tmp_89: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_88 }, { &mut __tmp_89 })
                },
                {
                    let mut __tmp_90: () = 5_();
                    let mut __tmp_91: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_90 }, { &mut __tmp_91 })
                },
                {
                    let mut __tmp_92: () = 6_();
                    let mut __tmp_93: () = 0_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_92 }, { &mut __tmp_93 })
                },
            ];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [
                {
                    let mut __tmp_94: () = 1_();
                    let mut __tmp_95: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_94 }, { &mut __tmp_95 })
                },
                {
                    let mut __tmp_96: () = 2_();
                    let mut __tmp_97: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_96 }, { &mut __tmp_97 })
                },
                {
                    let mut __tmp_98: () = 2_();
                    let mut __tmp_99: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_98 }, { &mut __tmp_99 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 4] = [
                {
                    let mut __tmp_100: () = 4_();
                    let mut __tmp_101: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_100 }, { &mut __tmp_101 })
                },
                {
                    let mut __tmp_102: () = 5_();
                    let mut __tmp_103: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_102 }, { &mut __tmp_103 })
                },
                {
                    let mut __tmp_104: () = 6_();
                    let mut __tmp_105: () = 0_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_104 }, { &mut __tmp_105 })
                },
                {
                    let mut __tmp_106: () = 7_();
                    let mut __tmp_107: () = 9_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_106 }, { &mut __tmp_107 })
                },
            ];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
    {
        let mut orig: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (50_())) {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        let mut copy: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        let mut iter: () = unimplemented!();
        'loop_: while unimplemented!() {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
}
pub unsafe fn test_alloc_17(
    lhs_alloc: *const tracking_allocator_std_pair_const_int__int__,
    rhs_alloc: *const tracking_allocator_std_pair_const_int__int__,
    mut check_alloc_invariant: AssertEmpty_18,
) {
    {
        {
            let mut arr: [(); 3] = [
                {
                    let mut __tmp_108: () = 1_();
                    let mut __tmp_109: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_108 }, { &mut __tmp_109 })
                },
                {
                    let mut __tmp_110: () = 2_();
                    let mut __tmp_111: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_110 }, { &mut __tmp_111 })
                },
                {
                    let mut __tmp_112: () = 2_();
                    let mut __tmp_113: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_112 }, { &mut __tmp_113 })
                },
            ];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
        {
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(); 3] = [
                {
                    let mut __tmp_114: () = 1_();
                    let mut __tmp_115: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_114 }, { &mut __tmp_115 })
                },
                {
                    let mut __tmp_116: () = 2_();
                    let mut __tmp_117: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_116 }, { &mut __tmp_117 })
                },
                {
                    let mut __tmp_118: () = 2_();
                    let mut __tmp_119: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_118 }, { &mut __tmp_119 })
                },
            ];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [
                {
                    let mut __tmp_120: () = 1_();
                    let mut __tmp_121: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_120 }, { &mut __tmp_121 })
                },
                {
                    let mut __tmp_122: () = 2_();
                    let mut __tmp_123: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_122 }, { &mut __tmp_123 })
                },
                {
                    let mut __tmp_124: () = 2_();
                    let mut __tmp_125: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_124 }, { &mut __tmp_125 })
                },
            ];
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [
                {
                    let mut __tmp_126: () = 1_();
                    let mut __tmp_127: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_126 }, { &mut __tmp_127 })
                },
                {
                    let mut __tmp_128: () = 2_();
                    let mut __tmp_129: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_128 }, { &mut __tmp_129 })
                },
                {
                    let mut __tmp_130: () = 2_();
                    let mut __tmp_131: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_130 }, { &mut __tmp_131 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 2] = [
                {
                    let mut __tmp_132: () = 4_();
                    let mut __tmp_133: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_132 }, { &mut __tmp_133 })
                },
                {
                    let mut __tmp_134: () = 5_();
                    let mut __tmp_135: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_134 }, { &mut __tmp_135 })
                },
            ];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [
                {
                    let mut __tmp_136: () = 1_();
                    let mut __tmp_137: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_136 }, { &mut __tmp_137 })
                },
                {
                    let mut __tmp_138: () = 2_();
                    let mut __tmp_139: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_138 }, { &mut __tmp_139 })
                },
                {
                    let mut __tmp_140: () = 2_();
                    let mut __tmp_141: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_140 }, { &mut __tmp_141 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 3] = [
                {
                    let mut __tmp_142: () = 4_();
                    let mut __tmp_143: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_142 }, { &mut __tmp_143 })
                },
                {
                    let mut __tmp_144: () = 5_();
                    let mut __tmp_145: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_144 }, { &mut __tmp_145 })
                },
                {
                    let mut __tmp_146: () = 6_();
                    let mut __tmp_147: () = 0_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_146 }, { &mut __tmp_147 })
                },
            ];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 3] = [
                {
                    let mut __tmp_148: () = 1_();
                    let mut __tmp_149: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_148 }, { &mut __tmp_149 })
                },
                {
                    let mut __tmp_150: () = 2_();
                    let mut __tmp_151: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_150 }, { &mut __tmp_151 })
                },
                {
                    let mut __tmp_152: () = 2_();
                    let mut __tmp_153: () = 6_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_152 }, { &mut __tmp_153 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 4] = [
                {
                    let mut __tmp_154: () = 4_();
                    let mut __tmp_155: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_154 }, { &mut __tmp_155 })
                },
                {
                    let mut __tmp_156: () = 5_();
                    let mut __tmp_157: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_156 }, { &mut __tmp_157 })
                },
                {
                    let mut __tmp_158: () = 6_();
                    let mut __tmp_159: () = 0_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_158 }, { &mut __tmp_159 })
                },
                {
                    let mut __tmp_160: () = 7_();
                    let mut __tmp_161: () = 9_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_160 }, { &mut __tmp_161 })
                },
            ];
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
    }
    {
        let mut orig: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (50_())) {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        let mut copy: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        let mut iter: () = unimplemented!();
        'loop_: while unimplemented!() {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
}
pub unsafe fn test_19() -> () {
    (unsafe { test_alloc_15(None, None, None) });
    (unsafe { test_alloc_16(None, None, None) });
    {
        let mut lhs_allocs: () = unimplemented!();
        let mut rhs_allocs: () = unimplemented!();
        (unsafe {
            let mut _lhs_alloc: tracking_allocator_std_pair_const_int__int__ =
                tracking_allocator_std_pair_const_int__int__::new({ &mut lhs_allocs });
            let mut _rhs_alloc: tracking_allocator_std_pair_const_int__int__ =
                tracking_allocator_std_pair_const_int__int__::new({ &mut rhs_allocs });
            let _check_alloc_invariant: AssertEmpty_18 =
                AssertEmpty_18::new({ &mut lhs_allocs }, { &mut rhs_allocs });
            test_alloc_17(&mut _lhs_alloc, &mut _rhs_alloc, _check_alloc_invariant)
        });
    }
    {
        let mut arr: [(); 3] = [
            {
                let mut __tmp_162: () = 1_();
                let mut __tmp_163: () = 1_();
                std_pair_const_int__int_::new_1({ &mut __tmp_162 }, { &mut __tmp_163 })
            },
            {
                let mut __tmp_164: () = 2_();
                let mut __tmp_165: () = 3_();
                std_pair_const_int__int_::new_1({ &mut __tmp_164 }, { &mut __tmp_165 })
            },
            {
                let mut __tmp_166: () = 2_();
                let mut __tmp_167: () = 6_();
                std_pair_const_int__int_::new_1({ &mut __tmp_166 }, { &mut __tmp_167 })
            },
        ];
        let orig: () = unimplemented!();
        let mut copy: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct AssertEmpty_18 {
    lhs_allocs_: *mut (),
    rhs_allocs_: *mut (),
}
impl AssertEmpty_18 {
    pub unsafe fn new(lhs_allocs: *mut (), rhs_allocs: *mut ()) -> Self {
        let mut this = Self {
            lhs_allocs_: (lhs_allocs),
            rhs_allocs_: (rhs_allocs),
        };
        this
    }
    pub unsafe fn operator_call(&mut self) {
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_19() });
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
