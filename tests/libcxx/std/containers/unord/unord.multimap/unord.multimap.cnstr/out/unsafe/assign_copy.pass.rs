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
        return (unsafe { (unimplemented!() as std___hash_impl_int__void_)((*x)) });
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
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        return min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
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
pub struct min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { ptr_ : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_index ( &self , mut n : () , ) -> *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_  . offset ( ( n ) as isize ) )  ;
 } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__int__ {}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { pub unsafe fn new_1 ( mut _a0 : min_allocator_std_pair_const_int__int__ , ) -> Self { let mut this = Self { } ;
 this } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
{}
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { return min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
 } pub unsafe fn deallocate ( &mut self , mut p : min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ , mut n : () , ) { unimplemented!()  ;
 } }
impl min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___ { }
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
pub unsafe fn operator_eq_9(
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
    pub unsafe fn new_1(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
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
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_5));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_5));
    }
}
impl std::cmp::PartialEq for test_allocator_std_pair_const_int__int__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_9(
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
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_10(
    x: *const other_allocator_std_pair_const_int__int__,
    y: *const other_allocator_std_pair_const_int__int__,
) -> () {
    return (((*x).data_) == ((*y).data_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct other_allocator_std_pair_const_int__int__ {
    data_: (),
}
impl other_allocator_std_pair_const_int__int__ {
    pub unsafe fn new_1(mut i: ()) -> Self {
        let mut this = Self { data_: i };
        this
    }
}
impl std::cmp::PartialEq for other_allocator_std_pair_const_int__int__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
                self as *const other_allocator_std_pair_const_int__int__,
                other as *const other_allocator_std_pair_const_int__int__,
            )
        }
    }
}
impl std::cmp::Eq for other_allocator_std_pair_const_int__int__ {}
impl Default for other_allocator_std_pair_const_int__int__ {
    fn default() -> Self {
        unsafe {
            other_allocator_std_pair_const_int__int__ {
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
pub struct tracking_allocator_std_pair_const_int__int__ {
    allocs_: *mut (),
}
impl tracking_allocator_std_pair_const_int__int__ {
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
            let mut arr: [(); 4] = [
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
                    let mut __tmp_4: () = 4_();
                    let mut __tmp_5: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_4 }, { &mut __tmp_5 })
                },
                {
                    let mut __tmp_6: () = 4_();
                    let mut __tmp_7: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_6 }, { &mut __tmp_7 })
                },
            ];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                let mut range: () = unimplemented!();
                let mut first_element: () = (unsafe { next_12(range.first.clone(), 0_()) });
                let mut second_element: () = (unsafe { next_12(range.first.clone(), 1_()) });
                let mut end: () = (unsafe { next_12(range.first.clone(), 2_()) });
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                let mut range: () = unimplemented!();
                let mut first_element: () = (unsafe { next_13(range.first.clone(), 0_()) });
                let mut second_element: () = (unsafe { next_13(range.first.clone(), 1_()) });
                let mut end: () = (unsafe { next_13(range.first.clone(), 2_()) });
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
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
            let mut arr: [(); 4] = [
                {
                    let mut __tmp_8: () = 1_();
                    let mut __tmp_9: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_8 }, { &mut __tmp_9 })
                },
                {
                    let mut __tmp_10: () = 2_();
                    let mut __tmp_11: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_10 }, { &mut __tmp_11 })
                },
                {
                    let mut __tmp_12: () = 4_();
                    let mut __tmp_13: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_12 }, { &mut __tmp_13 })
                },
                {
                    let mut __tmp_14: () = 5_();
                    let mut __tmp_15: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_14 }, { &mut __tmp_15 })
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
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
    {
        {
            let mut arr: [(); 1] = [{
                let mut __tmp_16: () = 1_();
                let mut __tmp_17: () = 1_();
                std_pair_const_int__int_::new_1({ &mut __tmp_16 }, { &mut __tmp_17 })
            }];
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
            let mut arr: [(); 1] = [{
                let mut __tmp_18: () = 1_();
                let mut __tmp_19: () = 1_();
                std_pair_const_int__int_::new_1({ &mut __tmp_18 }, { &mut __tmp_19 })
            }];
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
            let mut arr: [(); 4] = [
                {
                    let mut __tmp_20: () = 1_();
                    let mut __tmp_21: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_20 }, { &mut __tmp_21 })
                },
                {
                    let mut __tmp_22: () = 2_();
                    let mut __tmp_23: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_22 }, { &mut __tmp_23 })
                },
                {
                    let mut __tmp_24: () = 4_();
                    let mut __tmp_25: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_24 }, { &mut __tmp_25 })
                },
                {
                    let mut __tmp_26: () = 5_();
                    let mut __tmp_27: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_26 }, { &mut __tmp_27 })
                },
            ];
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
            let mut lhs_arr: [(); 4] = [
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
                    let mut __tmp_32: () = 4_();
                    let mut __tmp_33: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_32 }, { &mut __tmp_33 })
                },
                {
                    let mut __tmp_34: () = 5_();
                    let mut __tmp_35: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_34 }, { &mut __tmp_35 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 2] = [
                {
                    let mut __tmp_36: () = 10_();
                    let mut __tmp_37: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_36 }, { &mut __tmp_37 })
                },
                {
                    let mut __tmp_38: () = 13_();
                    let mut __tmp_39: () = 5_();
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 4] = [
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
                    let mut __tmp_44: () = 4_();
                    let mut __tmp_45: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_44 }, { &mut __tmp_45 })
                },
                {
                    let mut __tmp_46: () = 5_();
                    let mut __tmp_47: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_46 }, { &mut __tmp_47 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 4] = [
                {
                    let mut __tmp_48: () = 10_();
                    let mut __tmp_49: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_48 }, { &mut __tmp_49 })
                },
                {
                    let mut __tmp_50: () = 13_();
                    let mut __tmp_51: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_50 }, { &mut __tmp_51 })
                },
                {
                    let mut __tmp_52: () = 12_();
                    let mut __tmp_53: () = 324_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_52 }, { &mut __tmp_53 })
                },
                {
                    let mut __tmp_54: () = 0_();
                    let mut __tmp_55: () = 54_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_54 }, { &mut __tmp_55 })
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let mut __tmp_56: () = 1_();
                    let mut __tmp_57: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_56 }, { &mut __tmp_57 })
                },
                {
                    let mut __tmp_58: () = 2_();
                    let mut __tmp_59: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_58 }, { &mut __tmp_59 })
                },
                {
                    let mut __tmp_60: () = 4_();
                    let mut __tmp_61: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_60 }, { &mut __tmp_61 })
                },
                {
                    let mut __tmp_62: () = 5_();
                    let mut __tmp_63: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_62 }, { &mut __tmp_63 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 6] = [
                {
                    let mut __tmp_64: () = 10_();
                    let mut __tmp_65: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_64 }, { &mut __tmp_65 })
                },
                {
                    let mut __tmp_66: () = 13_();
                    let mut __tmp_67: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_66 }, { &mut __tmp_67 })
                },
                {
                    let mut __tmp_68: () = 12_();
                    let mut __tmp_69: () = 324_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_68 }, { &mut __tmp_69 })
                },
                {
                    let mut __tmp_70: () = 0_();
                    let mut __tmp_71: () = 54_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_70 }, { &mut __tmp_71 })
                },
                {
                    let mut __tmp_72: () = 50_();
                    let mut __tmp_73: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_72 }, { &mut __tmp_73 })
                },
                {
                    let mut __tmp_74: () = 2_();
                    let mut __tmp_75: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_74 }, { &mut __tmp_75 })
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
}
pub unsafe fn test_alloc_14(
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
            let mut arr: [(); 4] = [
                {
                    let mut __tmp_76: () = 1_();
                    let mut __tmp_77: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_76 }, { &mut __tmp_77 })
                },
                {
                    let mut __tmp_78: () = 2_();
                    let mut __tmp_79: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_78 }, { &mut __tmp_79 })
                },
                {
                    let mut __tmp_80: () = 4_();
                    let mut __tmp_81: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_80 }, { &mut __tmp_81 })
                },
                {
                    let mut __tmp_82: () = 4_();
                    let mut __tmp_83: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_82 }, { &mut __tmp_83 })
                },
            ];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                let mut range: () = unimplemented!();
                let mut first_element: () = (unsafe { next_15(range.first.clone(), 0_()) });
                let mut second_element: () = (unsafe { next_15(range.first.clone(), 1_()) });
                let mut end: () = (unsafe { next_15(range.first.clone(), 2_()) });
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                let mut range: () = unimplemented!();
                let mut first_element: () = (unsafe { next_16(range.first.clone(), 0_()) });
                let mut second_element: () = (unsafe { next_16(range.first.clone(), 1_()) });
                let mut end: () = (unsafe { next_16(range.first.clone(), 2_()) });
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
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
            let mut arr: [(); 4] = [
                {
                    let mut __tmp_84: () = 1_();
                    let mut __tmp_85: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_84 }, { &mut __tmp_85 })
                },
                {
                    let mut __tmp_86: () = 2_();
                    let mut __tmp_87: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_86 }, { &mut __tmp_87 })
                },
                {
                    let mut __tmp_88: () = 4_();
                    let mut __tmp_89: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_88 }, { &mut __tmp_89 })
                },
                {
                    let mut __tmp_90: () = 5_();
                    let mut __tmp_91: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_90 }, { &mut __tmp_91 })
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
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
    {
        {
            let mut arr: [(); 1] = [{
                let mut __tmp_92: () = 1_();
                let mut __tmp_93: () = 1_();
                std_pair_const_int__int_::new_1({ &mut __tmp_92 }, { &mut __tmp_93 })
            }];
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
            let mut arr: [(); 1] = [{
                let mut __tmp_94: () = 1_();
                let mut __tmp_95: () = 1_();
                std_pair_const_int__int_::new_1({ &mut __tmp_94 }, { &mut __tmp_95 })
            }];
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
            let mut arr: [(); 4] = [
                {
                    let mut __tmp_96: () = 1_();
                    let mut __tmp_97: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_96 }, { &mut __tmp_97 })
                },
                {
                    let mut __tmp_98: () = 2_();
                    let mut __tmp_99: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_98 }, { &mut __tmp_99 })
                },
                {
                    let mut __tmp_100: () = 4_();
                    let mut __tmp_101: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_100 }, { &mut __tmp_101 })
                },
                {
                    let mut __tmp_102: () = 5_();
                    let mut __tmp_103: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_102 }, { &mut __tmp_103 })
                },
            ];
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
            let mut lhs_arr: [(); 4] = [
                {
                    let mut __tmp_104: () = 1_();
                    let mut __tmp_105: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_104 }, { &mut __tmp_105 })
                },
                {
                    let mut __tmp_106: () = 2_();
                    let mut __tmp_107: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_106 }, { &mut __tmp_107 })
                },
                {
                    let mut __tmp_108: () = 4_();
                    let mut __tmp_109: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_108 }, { &mut __tmp_109 })
                },
                {
                    let mut __tmp_110: () = 5_();
                    let mut __tmp_111: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_110 }, { &mut __tmp_111 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 2] = [
                {
                    let mut __tmp_112: () = 10_();
                    let mut __tmp_113: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_112 }, { &mut __tmp_113 })
                },
                {
                    let mut __tmp_114: () = 13_();
                    let mut __tmp_115: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_114 }, { &mut __tmp_115 })
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let mut __tmp_116: () = 1_();
                    let mut __tmp_117: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_116 }, { &mut __tmp_117 })
                },
                {
                    let mut __tmp_118: () = 2_();
                    let mut __tmp_119: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_118 }, { &mut __tmp_119 })
                },
                {
                    let mut __tmp_120: () = 4_();
                    let mut __tmp_121: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_120 }, { &mut __tmp_121 })
                },
                {
                    let mut __tmp_122: () = 5_();
                    let mut __tmp_123: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_122 }, { &mut __tmp_123 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 4] = [
                {
                    let mut __tmp_124: () = 10_();
                    let mut __tmp_125: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_124 }, { &mut __tmp_125 })
                },
                {
                    let mut __tmp_126: () = 13_();
                    let mut __tmp_127: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_126 }, { &mut __tmp_127 })
                },
                {
                    let mut __tmp_128: () = 12_();
                    let mut __tmp_129: () = 324_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_128 }, { &mut __tmp_129 })
                },
                {
                    let mut __tmp_130: () = 0_();
                    let mut __tmp_131: () = 54_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_130 }, { &mut __tmp_131 })
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let mut __tmp_132: () = 1_();
                    let mut __tmp_133: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_132 }, { &mut __tmp_133 })
                },
                {
                    let mut __tmp_134: () = 2_();
                    let mut __tmp_135: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_134 }, { &mut __tmp_135 })
                },
                {
                    let mut __tmp_136: () = 4_();
                    let mut __tmp_137: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_136 }, { &mut __tmp_137 })
                },
                {
                    let mut __tmp_138: () = 5_();
                    let mut __tmp_139: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_138 }, { &mut __tmp_139 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 6] = [
                {
                    let mut __tmp_140: () = 10_();
                    let mut __tmp_141: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_140 }, { &mut __tmp_141 })
                },
                {
                    let mut __tmp_142: () = 13_();
                    let mut __tmp_143: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_142 }, { &mut __tmp_143 })
                },
                {
                    let mut __tmp_144: () = 12_();
                    let mut __tmp_145: () = 324_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_144 }, { &mut __tmp_145 })
                },
                {
                    let mut __tmp_146: () = 0_();
                    let mut __tmp_147: () = 54_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_146 }, { &mut __tmp_147 })
                },
                {
                    let mut __tmp_148: () = 50_();
                    let mut __tmp_149: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_148 }, { &mut __tmp_149 })
                },
                {
                    let mut __tmp_150: () = 2_();
                    let mut __tmp_151: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_150 }, { &mut __tmp_151 })
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { NoOp::operator_call(&mut check_alloc_invariant) });
    }
}
pub unsafe fn test_alloc_17(
    lhs_alloc: *const tracking_allocator_std_pair_const_int__int__,
    rhs_alloc: *const tracking_allocator_std_pair_const_int__int__,
    mut check_alloc_invariant: AssertEmpty_18,
) {
    {
        {
            let mut arr: [(); 4] = [
                {
                    let mut __tmp_152: () = 1_();
                    let mut __tmp_153: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_152 }, { &mut __tmp_153 })
                },
                {
                    let mut __tmp_154: () = 2_();
                    let mut __tmp_155: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_154 }, { &mut __tmp_155 })
                },
                {
                    let mut __tmp_156: () = 4_();
                    let mut __tmp_157: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_156 }, { &mut __tmp_157 })
                },
                {
                    let mut __tmp_158: () = 4_();
                    let mut __tmp_159: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_158 }, { &mut __tmp_159 })
                },
            ];
            let orig: () = unimplemented!();
            let mut copy: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                let mut range: () = unimplemented!();
                let mut first_element: () = (unsafe { next_12(range.first.clone(), 0_()) });
                let mut second_element: () = (unsafe { next_12(range.first.clone(), 1_()) });
                let mut end: () = (unsafe { next_12(range.first.clone(), 2_()) });
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            {
                let mut range: () = unimplemented!();
                let mut first_element: () = (unsafe { next_13(range.first.clone(), 0_()) });
                let mut second_element: () = (unsafe { next_13(range.first.clone(), 1_()) });
                let mut end: () = (unsafe { next_13(range.first.clone(), 2_()) });
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
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
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
        {
            let mut arr: [(); 4] = [
                {
                    let mut __tmp_160: () = 1_();
                    let mut __tmp_161: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_160 }, { &mut __tmp_161 })
                },
                {
                    let mut __tmp_162: () = 2_();
                    let mut __tmp_163: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_162 }, { &mut __tmp_163 })
                },
                {
                    let mut __tmp_164: () = 4_();
                    let mut __tmp_165: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_164 }, { &mut __tmp_165 })
                },
                {
                    let mut __tmp_166: () = 5_();
                    let mut __tmp_167: () = 2_();
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
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
    }
    {
        {
            let mut arr: [(); 1] = [{
                let mut __tmp_168: () = 1_();
                let mut __tmp_169: () = 1_();
                std_pair_const_int__int_::new_1({ &mut __tmp_168 }, { &mut __tmp_169 })
            }];
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
            let mut arr: [(); 1] = [{
                let mut __tmp_170: () = 1_();
                let mut __tmp_171: () = 1_();
                std_pair_const_int__int_::new_1({ &mut __tmp_170 }, { &mut __tmp_171 })
            }];
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
            let mut arr: [(); 4] = [
                {
                    let mut __tmp_172: () = 1_();
                    let mut __tmp_173: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_172 }, { &mut __tmp_173 })
                },
                {
                    let mut __tmp_174: () = 2_();
                    let mut __tmp_175: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_174 }, { &mut __tmp_175 })
                },
                {
                    let mut __tmp_176: () = 4_();
                    let mut __tmp_177: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_176 }, { &mut __tmp_177 })
                },
                {
                    let mut __tmp_178: () = 5_();
                    let mut __tmp_179: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_178 }, { &mut __tmp_179 })
                },
            ];
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
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
        {
            let mut orig: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
    }
    {
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let mut __tmp_180: () = 1_();
                    let mut __tmp_181: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_180 }, { &mut __tmp_181 })
                },
                {
                    let mut __tmp_182: () = 2_();
                    let mut __tmp_183: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_182 }, { &mut __tmp_183 })
                },
                {
                    let mut __tmp_184: () = 4_();
                    let mut __tmp_185: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_184 }, { &mut __tmp_185 })
                },
                {
                    let mut __tmp_186: () = 5_();
                    let mut __tmp_187: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_186 }, { &mut __tmp_187 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 2] = [
                {
                    let mut __tmp_188: () = 10_();
                    let mut __tmp_189: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_188 }, { &mut __tmp_189 })
                },
                {
                    let mut __tmp_190: () = 13_();
                    let mut __tmp_191: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_190 }, { &mut __tmp_191 })
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let mut __tmp_192: () = 1_();
                    let mut __tmp_193: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_192 }, { &mut __tmp_193 })
                },
                {
                    let mut __tmp_194: () = 2_();
                    let mut __tmp_195: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_194 }, { &mut __tmp_195 })
                },
                {
                    let mut __tmp_196: () = 4_();
                    let mut __tmp_197: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_196 }, { &mut __tmp_197 })
                },
                {
                    let mut __tmp_198: () = 5_();
                    let mut __tmp_199: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_198 }, { &mut __tmp_199 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 4] = [
                {
                    let mut __tmp_200: () = 10_();
                    let mut __tmp_201: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_200 }, { &mut __tmp_201 })
                },
                {
                    let mut __tmp_202: () = 13_();
                    let mut __tmp_203: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_202 }, { &mut __tmp_203 })
                },
                {
                    let mut __tmp_204: () = 12_();
                    let mut __tmp_205: () = 324_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_204 }, { &mut __tmp_205 })
                },
                {
                    let mut __tmp_206: () = 0_();
                    let mut __tmp_207: () = 54_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_206 }, { &mut __tmp_207 })
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
        {
            let mut lhs_arr: [(); 4] = [
                {
                    let mut __tmp_208: () = 1_();
                    let mut __tmp_209: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_208 }, { &mut __tmp_209 })
                },
                {
                    let mut __tmp_210: () = 2_();
                    let mut __tmp_211: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_210 }, { &mut __tmp_211 })
                },
                {
                    let mut __tmp_212: () = 4_();
                    let mut __tmp_213: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_212 }, { &mut __tmp_213 })
                },
                {
                    let mut __tmp_214: () = 5_();
                    let mut __tmp_215: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_214 }, { &mut __tmp_215 })
                },
            ];
            let orig: () = unimplemented!();
            let mut rhs_arr: [(); 6] = [
                {
                    let mut __tmp_216: () = 10_();
                    let mut __tmp_217: () = 4_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_216 }, { &mut __tmp_217 })
                },
                {
                    let mut __tmp_218: () = 13_();
                    let mut __tmp_219: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_218 }, { &mut __tmp_219 })
                },
                {
                    let mut __tmp_220: () = 12_();
                    let mut __tmp_221: () = 324_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_220 }, { &mut __tmp_221 })
                },
                {
                    let mut __tmp_222: () = 0_();
                    let mut __tmp_223: () = 54_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_222 }, { &mut __tmp_223 })
                },
                {
                    let mut __tmp_224: () = 50_();
                    let mut __tmp_225: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_224 }, { &mut __tmp_225 })
                },
                {
                    let mut __tmp_226: () = 2_();
                    let mut __tmp_227: () = 5_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_226 }, { &mut __tmp_227 })
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        (unsafe { AssertEmpty_18::operator_call(&mut check_alloc_invariant) });
    }
}
pub unsafe fn test_19() {
    (unsafe { test_alloc_11(None, None, None) });
    (unsafe { test_alloc_14(None, None, None) });
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
        {
            let mut arr: [(); 3] = [
                {
                    let mut __tmp_228: () = 1_();
                    let mut __tmp_229: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_228 }, { &mut __tmp_229 })
                },
                {
                    let mut __tmp_230: () = 2_();
                    let mut __tmp_231: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_230 }, { &mut __tmp_231 })
                },
                {
                    let mut __tmp_232: () = 3_();
                    let mut __tmp_233: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_232 }, { &mut __tmp_233 })
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
            let mut arr: [(); 3] = [
                {
                    let mut __tmp_234: () = 1_();
                    let mut __tmp_235: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_234 }, { &mut __tmp_235 })
                },
                {
                    let mut __tmp_236: () = 2_();
                    let mut __tmp_237: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_236 }, { &mut __tmp_237 })
                },
                {
                    let mut __tmp_238: () = 3_();
                    let mut __tmp_239: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_238 }, { &mut __tmp_239 })
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
            let mut arr: [(); 3] = [
                {
                    let mut __tmp_240: () = 1_();
                    let mut __tmp_241: () = 1_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_240 }, { &mut __tmp_241 })
                },
                {
                    let mut __tmp_242: () = 2_();
                    let mut __tmp_243: () = 2_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_242 }, { &mut __tmp_243 })
                },
                {
                    let mut __tmp_244: () = 3_();
                    let mut __tmp_245: () = 3_();
                    std_pair_const_int__int_::new_1({ &mut __tmp_244 }, { &mut __tmp_245 })
                },
            ];
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
                let mut arr: [(); 3] = [
                    {
                        let mut __tmp_246: () = 1_();
                        let mut __tmp_247: () = 1_();
                        std_pair_const_int__int_::new_1({ &mut __tmp_246 }, { &mut __tmp_247 })
                    },
                    {
                        let mut __tmp_248: () = 2_();
                        let mut __tmp_249: () = 2_();
                        std_pair_const_int__int_::new_1({ &mut __tmp_248 }, { &mut __tmp_249 })
                    },
                    {
                        let mut __tmp_250: () = 3_();
                        let mut __tmp_251: () = 3_();
                        std_pair_const_int__int_::new_1({ &mut __tmp_250 }, { &mut __tmp_251 })
                    },
                ];
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
                let mut arr: [(); 3] = [
                    {
                        let mut __tmp_252: () = 1_();
                        let mut __tmp_253: () = 1_();
                        std_pair_const_int__int_::new_1({ &mut __tmp_252 }, { &mut __tmp_253 })
                    },
                    {
                        let mut __tmp_254: () = 2_();
                        let mut __tmp_255: () = 2_();
                        std_pair_const_int__int_::new_1({ &mut __tmp_254 }, { &mut __tmp_255 })
                    },
                    {
                        let mut __tmp_256: () = 3_();
                        let mut __tmp_257: () = 3_();
                        std_pair_const_int__int_::new_1({ &mut __tmp_256 }, { &mut __tmp_257 })
                    },
                ];
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
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_int__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
    std::cell::LazyCell::force(&*&raw const destructed_value_5);
    std::cell::LazyCell::force(&*&raw const moved_value_6);
}
