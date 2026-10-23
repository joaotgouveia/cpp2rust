extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(a: *const test_equal_to_int_, b: *const test_equal_to_int_) -> () {
    return (((*a).data_) == ((*b).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_equal_to_int_ {
    pub data_: (),
}
impl test_equal_to_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { data_: 0_() };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, a: *const (), b: *const ()) -> () {
        return ((*a) == (*b));
    }
}
impl std::cmp::PartialEq for test_equal_to_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_0(
                self as *const test_equal_to_int_,
                other as *const test_equal_to_int_,
            )
        }
    }
}
impl std::cmp::Eq for test_equal_to_int_ {}
impl Default for test_equal_to_int_ {
    fn default() -> Self {
        unsafe { test_equal_to_int_::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_hash_int_ {
    data_: (),
}
impl test_hash_int_ {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn operator_call(&self, x: *const ()) -> () {
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, c: *const test_hash_int_) -> () {
        return ((self.data_) == ((*c).data_));
    }
}
impl std::cmp::PartialEq for test_hash_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { test_hash_int_::operator_eq(self, other as *const test_hash_int_) }
    }
}
impl std::cmp::Eq for test_hash_int_ {}
impl Default for test_hash_int_ {
    fn default() -> Self {
        unsafe { test_hash_int_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_4)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3)) = 0_();
    }
}
pub static mut outstanding_bytes_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { ptr_ : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_index ( &self , mut n : () , ) -> *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_  . offset ( ( n ) as isize ) )  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { pub unsafe fn new_1 ( mut _a0 : min_allocator_int_ , ) -> Self { let mut this = Self { } ;
 this } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
{}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { return min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { }
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
pub static mut destructed_value_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_7(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_8(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_7(_x, _y)
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_5));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_5));
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
            operator_eq_7(
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
pub unsafe fn operator_eq_9(x: *const test_allocator_int_, y: *const test_allocator_int_) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_allocator_int__rebind_int_ {}
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe)]
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
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        unimplemented!();
        if !((self.stats_).is_null()) {
            (*self.stats_).count.prefix_dec();
        }
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_5));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_5));
    }
}
impl std::cmp::PartialEq for test_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_9(
                self as *const test_allocator_int_,
                other as *const test_allocator_int_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_int_ {}
impl Default for test_allocator_int_ {
    fn default() -> Self {
        unsafe {
            test_allocator_int_ {
                data_: unimplemented!(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_10(x: *const other_allocator_int_, y: *const other_allocator_int_) -> () {
    return (((*x).data_) == ((*y).data_));
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
}
impl std::cmp::PartialEq for other_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
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
pub struct tracking_allocator_int_ {
    allocs_: *mut (),
}
impl tracking_allocator_int_ {
    pub unsafe fn new(allocs: *mut ()) -> Self {
        let mut this = Self { allocs_: (allocs) };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoOp {}
impl NoOp {
    pub unsafe fn operator_call(&mut self) {}
}
pub unsafe fn test_alloc_11(
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
            let mut arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
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
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
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
        {
            let mut arr: [(); 1] = [1_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut arr: [(); 1] = [1_()];
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
            let mut arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let mut orig: () = unimplemented!();
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
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
    {
        {
            let mut lhs_arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 2] = [10_(), 13_()];
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 4] = [10_(), 13_(), 12_(), 0_()];
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 6] = [10_(), 13_(), 12_(), 0_(), 50_(), 2_()];
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
}
pub unsafe fn test_alloc_12(
    lhs_alloc: Option<*const min_allocator_int_>,
    rhs_alloc: Option<*const min_allocator_int_>,
    mut check_alloc_invariant: Option<NoOp>,
) {
    let mut __lhs_alloc_default: Option<min_allocator_int_> = None;
    let mut lhs_alloc: *const min_allocator_int_ = lhs_alloc.unwrap_or_else(|| unsafe {
        __lhs_alloc_default.insert(<min_allocator_int_>::default()) as *const min_allocator_int_
    });
    let mut __rhs_alloc_default: Option<min_allocator_int_> = None;
    let mut rhs_alloc: *const min_allocator_int_ = rhs_alloc.unwrap_or_else(|| unsafe {
        __rhs_alloc_default.insert(<min_allocator_int_>::default()) as *const min_allocator_int_
    });
    let mut check_alloc_invariant: NoOp =
        check_alloc_invariant.unwrap_or_else(|| unsafe { <NoOp>::default() });
    {
        {
            let mut arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
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
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
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
        {
            let mut arr: [(); 1] = [1_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut arr: [(); 1] = [1_()];
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
            let mut arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let mut orig: () = unimplemented!();
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
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
    {
        {
            let mut lhs_arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 2] = [10_(), 13_()];
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 4] = [10_(), 13_(), 12_(), 0_()];
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 6] = [10_(), 13_(), 12_(), 0_(), 50_(), 2_()];
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
}
pub unsafe fn test_alloc_13(
    lhs_alloc: *const tracking_allocator_int_,
    rhs_alloc: *const tracking_allocator_int_,
    mut check_alloc_invariant: AssertEmpty_14,
) {
    {
        {
            let mut arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
        {
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
    }
    {
        {
            let mut arr: [(); 1] = [1_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut arr: [(); 1] = [1_()];
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
            let mut arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
        {
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
    }
    {
        {
            let mut lhs_arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 2] = [10_(), 13_()];
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 4] = [10_(), 13_(), 12_(), 0_()];
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 4] = [1_(), 2_(), 4_(), 5_()];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 6] = [10_(), 13_(), 12_(), 0_(), 50_(), 2_()];
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_14::operator_call(&mut check_alloc_invariant) });
    }
}
pub unsafe fn test_15() {
    (unsafe { test_alloc_11(None, None, None) });
    (unsafe { test_alloc_12(None, None, None) });
    {
        let mut lhs_allocs: () = unimplemented!();
        let mut rhs_allocs: () = unimplemented!();
        (unsafe {
            let mut _lhs_alloc: tracking_allocator_int_ =
                tracking_allocator_int_::new({ &mut lhs_allocs });
            let mut _rhs_alloc: tracking_allocator_int_ =
                tracking_allocator_int_::new({ &mut rhs_allocs });
            let _check_alloc_invariant: AssertEmpty_14 =
                AssertEmpty_14::new({ &mut lhs_allocs }, { &mut rhs_allocs });
            test_alloc_13(&mut _lhs_alloc, &mut _rhs_alloc, _check_alloc_invariant)
        });
    }
    {
        {
            let mut arr: [(); 3] = [1_(), 2_(), 3_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
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
            let mut arr: [(); 3] = [1_(), 2_(), 3_()];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
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
            let mut arr: [(); 3] = [1_(), 2_(), 3_()];
            let mut orig: () = unimplemented!();
            unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut orig: () = unimplemented!();
            unimplemented!();
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
            {
                let mut arr: [(); 3] = [1_(), 2_(), 3_()];
                let orig: () = unimplemented!();
                let mut copy: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let orig: () = unimplemented!();
                let mut copy: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut arr: [(); 3] = [1_(), 2_(), 3_()];
                let orig: () = unimplemented!();
                let mut copy: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let orig: () = unimplemented!();
                let mut copy: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct AssertEmpty_14 {
    lhs_allocs_: *mut (),
    rhs_allocs_: *mut (),
}
impl AssertEmpty_14 {
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
    (unsafe { test_15() });
    return 0_();
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_int__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
    std::cell::LazyCell::force(&*&raw const destructed_value_5);
    std::cell::LazyCell::force(&*&raw const moved_value_6);
}
