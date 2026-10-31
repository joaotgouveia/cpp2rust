extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_contiguous_container_asan_correct_0(_a0: *const Vec<i32>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_1(_a0: *const Vec<Vec<i32>>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_2(
    _a0: *const Vec<NonTriviallyRelocatable>,
) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_3(
    _a0: *const Vec<Vec<NonTriviallyRelocatable>>,
) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_4(_a0: *const Vec<i32>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_5(_a0: *const Vec<Vec<i32>>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_6(
    _a0: *const Vec<NonTriviallyRelocatable>,
) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_7(
    _a0: *const Vec<Vec<NonTriviallyRelocatable>>,
) -> bool {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tracker {
    pub copy_assignments: i32,
    pub move_assignments: i32,
}
impl Default for Tracker {
    fn default() -> Self {
        unsafe {
            Tracker {
                copy_assignments: 0,
                move_assignments: 0,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct TrackedAssignment {
    pub tracker_: *mut Tracker,
}
impl TrackedAssignment {
    pub unsafe fn new(mut tracker: *mut Tracker) -> Self {
        let mut this = Self { tracker_: tracker };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const TrackedAssignment) -> *mut TrackedAssignment {
        (*self.tracker_).copy_assignments.postfix_inc();
        return &mut (*(self as *mut TrackedAssignment));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TrackedAssignment) -> *mut TrackedAssignment {
        (*self.tracker_).move_assignments.postfix_inc();
        return &mut (*(self as *mut TrackedAssignment));
    }
}
pub unsafe fn operator_eq_8(
    a: *const NonTriviallyRelocatable,
    b: *const NonTriviallyRelocatable,
) -> bool {
    return (((*a).value_) == ((*b).value_));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct NonTriviallyRelocatable {
    pub value_: i32,
}
impl NonTriviallyRelocatable {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { value_: 0 };
        this
    }
    pub unsafe fn new_2(mut v: i32) -> Self {
        let mut this = Self { value_: v };
        this
    }
    pub unsafe fn copy_from(other: *const NonTriviallyRelocatable) -> Self {
        let mut this = Self {
            value_: (*other).value_,
        };
        this
    }
    pub unsafe fn move_from(other: *mut NonTriviallyRelocatable) -> Self {
        let mut this = Self {
            value_: (*other).value_,
        };
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        other: *const NonTriviallyRelocatable,
    ) -> *mut NonTriviallyRelocatable {
        self.value_ = (*other).value_;
        return &mut (*(self as *mut NonTriviallyRelocatable));
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut NonTriviallyRelocatable,
    ) -> *mut NonTriviallyRelocatable {
        self.value_ = (*other).value_;
        return &mut (*(self as *mut NonTriviallyRelocatable));
    }
}
impl std::cmp::PartialEq for NonTriviallyRelocatable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_8(
                self as *const NonTriviallyRelocatable,
                other as *const NonTriviallyRelocatable,
            )
        }
    }
}
impl std::cmp::Eq for NonTriviallyRelocatable {}
impl Clone for NonTriviallyRelocatable {
    fn clone(&self) -> Self {
        unsafe { NonTriviallyRelocatable::copy_from(self as *const NonTriviallyRelocatable) }
    }
}
impl Default for NonTriviallyRelocatable {
    fn default() -> Self {
        unsafe { NonTriviallyRelocatable::new_1() }
    }
}
const _: () = assert!(true, "");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_12)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_9)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11)) = 0_usize;
    }
}
pub static mut outstanding_bytes_9: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_10: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_11: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_12: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_13(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_14(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_15(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_14(_x, y)
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
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
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
        unsafe { operator_eq_14(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_16(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_17(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_18(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(
    min_pointer_int__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const i32,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
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
        mut n: isize,
    ) -> *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
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
        unsafe { operator_eq_17(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *const ::libc::c_void,
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_19(
    mut x : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<Vec<i32>>()) as i64
        as isize);
}
pub unsafe fn operator_eq_20(
    mut x : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_21(
    mut x : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_20(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut Vec<i32>,
}
impl min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut Vec<i32>) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut Vec<i32> {
        return &mut (*self.ptr_);
    }    pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__{
        self.ptr_.prefix_inc();
        return & mut ( * ( self as *mut min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  )  )  ;
    }    pub unsafe fn operator_add_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__{
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return & mut ( * ( self as *mut min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  )  )  ;
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
    {
        let mut tmp : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  )  ) ;
        (unsafe {
            min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq
    for min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_20(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq
    for min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
}
impl UnsafeCppAdvance
    for min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
pub unsafe fn operator_sub_22(
    mut x : min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<Vec<i32>>()) as i64
        as isize);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(
    min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
pub struct min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
    ptr_: *const Vec<i32>,
}
impl
    min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
    pub unsafe fn new_1(
        mut p : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
}
pub unsafe fn operator_sub_23(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<MoveOnly>()) as i64
        as isize);
}
pub unsafe fn operator_eq_24(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_25(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_24(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut MoveOnly,
}
impl min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut MoveOnly) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut MoveOnly {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut MoveOnly {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self
            as *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::operator_sub_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_24(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
pub unsafe fn operator_sub_26(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<MoveOnly>()) as i64
        as isize);
}
pub unsafe fn operator_eq_27(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(
    min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
pub struct min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    ptr_: *const MoveOnly,
}
impl min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(
        mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
}
impl std::cmp::PartialEq for min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_27(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_28(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<TrivialMoveOnly>()) as i64
        as isize);
}
pub unsafe fn operator_eq_29(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_30(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_29(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut TrivialMoveOnly,
}
impl min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut TrivialMoveOnly) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut TrivialMoveOnly {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut TrivialMoveOnly {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self
            as *mut min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ = (*(self
            as *const min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , )
        });
        return tmp;
    }
    pub unsafe fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ = (*(self
            as *const min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ :: operator_sub_assign ( &mut tmp  , n , )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_29(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
pub unsafe fn operator_sub_31(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<TrivialMoveOnly>()) as i64
        as isize);
}
pub unsafe fn operator_eq_32(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(
    min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
pub struct min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    ptr_: *const TrivialMoveOnly,
}
impl min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(
        mut p: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
}
impl std::cmp::PartialEq
    for min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_32(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_33(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<NonTriviallyRelocatable>())
        as i64 as isize);
}
pub unsafe fn operator_eq_34(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_35(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_34(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut NonTriviallyRelocatable,
}
impl min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut NonTriviallyRelocatable) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut NonTriviallyRelocatable {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return & mut ( * ( self as *mut min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__  )  )  ;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return & mut ( * ( self as *mut min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__  )  )  ;
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        let mut tmp : min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__  )  ) ;
        (unsafe {
            min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_34(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
}
impl UnsafeCppAdvance
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
pub unsafe fn operator_sub_36(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<NonTriviallyRelocatable>())
        as i64 as isize);
}
pub unsafe fn operator_eq_37(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_38(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(
    min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
pub struct min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    ptr_: *const NonTriviallyRelocatable,
}
impl min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(
        mut p: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const NonTriviallyRelocatable {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_inc_9(
        &mut self,
    ) -> *mut min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
    {
        self.ptr_.prefix_inc();
        return & mut ( * ( self as *mut min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__  )  )  ;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
    {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return & mut ( * ( self as *mut min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__  )  )  ;
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        let mut tmp : min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__  )  ) ;
        (unsafe {
            min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_37(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
}
impl UnsafeCppAdvance
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
pub unsafe fn operator_sub_39(
    mut x : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<Vec<NonTriviallyRelocatable>>()) as i64 as isize);
}
pub unsafe fn operator_eq_40(
    mut x : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_41(
    mut x : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_40(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut Vec<NonTriviallyRelocatable>,
}
impl min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut Vec<NonTriviallyRelocatable > , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn to_bool ( &self , ) -> bool { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut Vec<NonTriviallyRelocatable > { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : isize , ) -> *mut min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : isize , ) -> min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::PartialEq for min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_40(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { unsafe fn cpp_inc(&mut self) { self.operator_inc_8(); } }
pub unsafe fn operator_sub_42(
    mut x : min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<Vec<NonTriviallyRelocatable>>()) as i64 as isize);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ , new_1, value)]
pub struct min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__
{
    ptr_: *const Vec<NonTriviallyRelocatable>,
}
impl min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { pub unsafe fn new_1 ( mut p : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : ( p . ptr_  ) .cast_const() , } ;
 this } }
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_vector_int__min_allocator_int___ {}
impl min_allocator_std_vector_int__min_allocator_int___ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
    {
        return min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_MoveOnly_ {}
impl min_allocator_MoveOnly_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_TrivialMoveOnly_ {}
impl min_allocator_TrivialMoveOnly_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_NonTriviallyRelocatable_ {}
impl min_allocator_NonTriviallyRelocatable_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        return min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable___
{}
impl min_allocator_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable___ {
    pub unsafe fn allocate ( &mut self , mut n : usize , ) -> min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__{
        return min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
pub unsafe fn operator_eq_43(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_44(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_45(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_46(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_47(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_48(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_49(_a0: *const MoveOnly, _a1: *const MoveOnly) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct MoveOnly {
    data_: i32,
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0;
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0;
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_49(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_43(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_50(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_51(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_52(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_53(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_54(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_55(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_56(
    _a0: *const TrivialMoveOnly,
    _a1: *const TrivialMoveOnly,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct TrivialMoveOnly {
    data_: i32,
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(_a0: *mut TrivialMoveOnly) -> Self {
        let mut this = Self {
            data_: (*_a0).data_,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TrivialMoveOnly) -> *mut TrivialMoveOnly {
        self.data_ = (*_a0).data_;
        return &mut (*(self as *mut TrivialMoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_56(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_50(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        unsafe { TrivialMoveOnly::new(None) }
    }
}
pub unsafe fn tests_57() {
    {
        let mut arr: [i32; 3] = [1, 2, 3];
        {
            {
                let mut v: Vec<i32> = Vec::new();
                let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                assert!(v.is_empty());
                assert!(i == v.end());
                assert!((unsafe { is_contiguous_container_asan_correct_0(&v,) }));
            }
            {
                let mut v: Vec<i32> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push((*__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut first: UnsafeRandomAccessIterator<i32> = unimplemented!();
                let mut last: UnsafeRandomAccessIterator<i32> = unimplemented!();
                let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == last);
                assert!((unsafe { is_contiguous_container_asan_correct_0(&v,) }));
            }
            {
                let mut v: Vec<i32> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push((*__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut first: UnsafeRandomAccessIterator<i32> = unimplemented!().iter_add(1_isize);
                let mut last: UnsafeRandomAccessIterator<i32> = unimplemented!().iter_add(1_isize);
                let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == last);
                assert!((unsafe { is_contiguous_container_asan_correct_0(&v,) }));
            }
            {
                let mut v: Vec<i32> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push((*__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut first: UnsafeRandomAccessIterator<i32> = unimplemented!();
                let mut last: UnsafeRandomAccessIterator<i32> = unimplemented!();
                let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == last);
                assert!((unsafe { is_contiguous_container_asan_correct_0(&v,) }));
            }
        }
        {
            {
                {
                    let mut v: Vec<i32> = {
                        let mut __v = Vec::new();
                        let mut __it = arr.as_mut_ptr();
                        while __it != arr.as_mut_ptr().offset((3) as isize) {
                            __v.push((*__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    };
                    let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                    assert!(unimplemented!());
                    assert!(i == v.begin());
                    assert!((unsafe { is_contiguous_container_asan_correct_0(&v,) }));
                }
                {
                    let mut v: Vec<i32> = {
                        let mut __v = Vec::new();
                        let mut __it = arr.as_mut_ptr();
                        while __it != arr.as_mut_ptr().offset((3) as isize) {
                            __v.push((*__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    };
                    let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                    assert!(unimplemented!());
                    assert!(i == v.begin());
                    assert!((unsafe { is_contiguous_container_asan_correct_0(&v,) }));
                }
                {
                    let mut v: Vec<i32> = {
                        let mut __v = Vec::new();
                        let mut __it = arr.as_mut_ptr();
                        while __it != arr.as_mut_ptr().offset((3) as isize) {
                            __v.push((*__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    };
                    let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                    assert!(((v.len()) == (0_usize)));
                    assert!(i == v.begin());
                    assert!((unsafe { is_contiguous_container_asan_correct_0(&v,) }));
                }
            }
            {
                let mut v: Vec<i32> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push((*__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                assert!(((v.len()) == (2_usize)));
                assert!(((*(&mut v).as_mut_ptr().add(0_usize as usize)) == (arr[(0) as usize])));
                assert!(((*(&mut v).as_mut_ptr().add(1_usize as usize)) == (arr[(2) as usize])));
                assert!(i == v.begin().iter_add(1_isize));
                assert!((unsafe { is_contiguous_container_asan_correct_0(&v,) }));
            }
            {
                let mut v: Vec<i32> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push((*__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == v.begin().iter_add(1_isize));
                assert!((unsafe { is_contiguous_container_asan_correct_0(&v,) }));
            }
        }
    }
    {
        let mut outer: Vec<Vec<i32>> =
            vec![(0..(1_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(); 2_usize as usize];
        unimplemented!();
        assert!(((outer.len()) == (2_usize)));
        assert!((((*(&mut outer).as_mut_ptr().add(0_usize as usize)).len()) == (1_usize)));
        assert!((((*(&mut outer).as_mut_ptr().add(1_usize as usize)).len()) == (1_usize)));
        assert!((unsafe { is_contiguous_container_asan_correct_1(&outer,) }));
        assert!(
            (unsafe {
                is_contiguous_container_asan_correct_0(
                    &(*(&mut outer).as_mut_ptr().add(0_usize as usize)),
                )
            })
        );
        assert!(
            (unsafe {
                is_contiguous_container_asan_correct_0(
                    &(*(&mut outer).as_mut_ptr().add(1_usize as usize)),
                )
            })
        );
    }
    {
        {
            let mut v: Vec<MoveOnly> = Vec::new();
            {
                let __init = MoveOnly::new({ Some(1) });
                v.push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(2) });
                v.push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(3) });
                v.push(__init)
            };
            unimplemented!();
            assert!(((v.len()) == (1_usize)));
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: MoveOnly = MoveOnly::new({ Some(3) });
                    operator_eq_43(_x, &mut _y)
                })
            );
        }
        {
            let mut v: Vec<TrivialMoveOnly> = Vec::new();
            {
                let __init = TrivialMoveOnly::new({ Some(1) });
                v.push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(2) });
                v.push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(3) });
                v.push(__init)
            };
            unimplemented!();
            assert!(((v.len()) == (1_usize)));
            assert!(
                (unsafe {
                    let _x: *const TrivialMoveOnly =
                        &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: TrivialMoveOnly = TrivialMoveOnly::new({ Some(3) });
                    operator_eq_50(_x, &mut _y)
                })
            );
        }
    }
}
pub unsafe fn tests_58() {
    {
        let mut arr: [NonTriviallyRelocatable; 3] = [
            NonTriviallyRelocatable::new_2({ 1 }),
            NonTriviallyRelocatable::new_2({ 2 }),
            NonTriviallyRelocatable::new_2({ 3 }),
        ];
        {
            {
                let mut v: Vec<NonTriviallyRelocatable> = Vec::new();
                let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = unimplemented!();
                assert!(v.is_empty());
                assert!(i == v.end());
                assert!((unsafe { is_contiguous_container_asan_correct_2(&v,) }));
            }
            {
                let mut v: Vec<NonTriviallyRelocatable> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut first: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                    unimplemented!();
                let mut last: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                    unimplemented!();
                let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == last);
                assert!((unsafe { is_contiguous_container_asan_correct_2(&v,) }));
            }
            {
                let mut v: Vec<NonTriviallyRelocatable> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut first: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                    unimplemented!().iter_add(1_isize);
                let mut last: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                    unimplemented!().iter_add(1_isize);
                let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == last);
                assert!((unsafe { is_contiguous_container_asan_correct_2(&v,) }));
            }
            {
                let mut v: Vec<NonTriviallyRelocatable> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut first: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                    unimplemented!();
                let mut last: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                    unimplemented!();
                let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == last);
                assert!((unsafe { is_contiguous_container_asan_correct_2(&v,) }));
            }
        }
        {
            {
                {
                    let mut v: Vec<NonTriviallyRelocatable> = {
                        let mut __v = Vec::new();
                        let mut __it = arr.as_mut_ptr();
                        while __it != arr.as_mut_ptr().offset((3) as isize) {
                            __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    };
                    let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                        unimplemented!();
                    assert!(unimplemented!());
                    assert!(i == v.begin());
                    assert!((unsafe { is_contiguous_container_asan_correct_2(&v,) }));
                }
                {
                    let mut v: Vec<NonTriviallyRelocatable> = {
                        let mut __v = Vec::new();
                        let mut __it = arr.as_mut_ptr();
                        while __it != arr.as_mut_ptr().offset((3) as isize) {
                            __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    };
                    let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                        unimplemented!();
                    assert!(unimplemented!());
                    assert!(i == v.begin());
                    assert!((unsafe { is_contiguous_container_asan_correct_2(&v,) }));
                }
                {
                    let mut v: Vec<NonTriviallyRelocatable> = {
                        let mut __v = Vec::new();
                        let mut __it = arr.as_mut_ptr();
                        while __it != arr.as_mut_ptr().offset((3) as isize) {
                            __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    };
                    let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                        unimplemented!();
                    assert!(((v.len()) == (0_usize)));
                    assert!(i == v.begin());
                    assert!((unsafe { is_contiguous_container_asan_correct_2(&v,) }));
                }
            }
            {
                let mut v: Vec<NonTriviallyRelocatable> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = unimplemented!();
                assert!(((v.len()) == (2_usize)));
                assert!(
                    (unsafe {
                        let _a: *const NonTriviallyRelocatable =
                            &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                        operator_eq_8(_a, &arr[(0) as usize])
                    })
                );
                assert!(
                    (unsafe {
                        let _a: *const NonTriviallyRelocatable =
                            &(*(&mut v).as_mut_ptr().add(1_usize as usize));
                        operator_eq_8(_a, &arr[(2) as usize])
                    })
                );
                assert!(i == v.begin().iter_add(1_isize));
                assert!((unsafe { is_contiguous_container_asan_correct_2(&v,) }));
            }
            {
                let mut v: Vec<NonTriviallyRelocatable> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == v.begin().iter_add(1_isize));
                assert!((unsafe { is_contiguous_container_asan_correct_2(&v,) }));
            }
        }
    }
    {
        let mut outer: Vec<Vec<NonTriviallyRelocatable>> = vec![
            (0..(1_usize) as usize)
                .map(|_| <NonTriviallyRelocatable>::default())
                .collect::<Vec<_>>();
            2_usize as usize
        ];
        unimplemented!();
        assert!(((outer.len()) == (2_usize)));
        assert!((((*(&mut outer).as_mut_ptr().add(0_usize as usize)).len()) == (1_usize)));
        assert!((((*(&mut outer).as_mut_ptr().add(1_usize as usize)).len()) == (1_usize)));
        assert!((unsafe { is_contiguous_container_asan_correct_3(&outer,) }));
        assert!(
            (unsafe {
                is_contiguous_container_asan_correct_2(
                    &(*(&mut outer).as_mut_ptr().add(0_usize as usize)),
                )
            })
        );
        assert!(
            (unsafe {
                is_contiguous_container_asan_correct_2(
                    &(*(&mut outer).as_mut_ptr().add(1_usize as usize)),
                )
            })
        );
    }
    {
        {
            let mut v: Vec<MoveOnly> = Vec::new();
            {
                let __init = MoveOnly::new({ Some(1) });
                v.push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(2) });
                v.push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(3) });
                v.push(__init)
            };
            unimplemented!();
            assert!(((v.len()) == (1_usize)));
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: MoveOnly = MoveOnly::new({ Some(3) });
                    operator_eq_43(_x, &mut _y)
                })
            );
        }
        {
            let mut v: Vec<TrivialMoveOnly> = Vec::new();
            {
                let __init = TrivialMoveOnly::new({ Some(1) });
                v.push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(2) });
                v.push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(3) });
                v.push(__init)
            };
            unimplemented!();
            assert!(((v.len()) == (1_usize)));
            assert!(
                (unsafe {
                    let _x: *const TrivialMoveOnly =
                        &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: TrivialMoveOnly = TrivialMoveOnly::new({ Some(3) });
                    operator_eq_50(_x, &mut _y)
                })
            );
        }
    }
}
pub unsafe fn tests_59() {
    {
        let mut arr: [i32; 3] = [1, 2, 3];
        {
            {
                let mut v: Vec<i32> = Vec::new();
                let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                assert!(v.is_empty());
                assert!(i == v.end());
                assert!((unsafe { is_contiguous_container_asan_correct_4(&v,) }));
            }
            {
                let mut v: Vec<i32> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push((*__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut first: UnsafeRandomAccessIterator<i32> = unimplemented!();
                let mut last: UnsafeRandomAccessIterator<i32> = unimplemented!();
                let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == last);
                assert!((unsafe { is_contiguous_container_asan_correct_4(&v,) }));
            }
            {
                let mut v: Vec<i32> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push((*__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut first: UnsafeRandomAccessIterator<i32> = unimplemented!().iter_add(1_isize);
                let mut last: UnsafeRandomAccessIterator<i32> = unimplemented!().iter_add(1_isize);
                let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == last);
                assert!((unsafe { is_contiguous_container_asan_correct_4(&v,) }));
            }
            {
                let mut v: Vec<i32> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push((*__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut first: UnsafeRandomAccessIterator<i32> = unimplemented!();
                let mut last: UnsafeRandomAccessIterator<i32> = unimplemented!();
                let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == last);
                assert!((unsafe { is_contiguous_container_asan_correct_4(&v,) }));
            }
        }
        {
            {
                {
                    let mut v: Vec<i32> = {
                        let mut __v = Vec::new();
                        let mut __it = arr.as_mut_ptr();
                        while __it != arr.as_mut_ptr().offset((3) as isize) {
                            __v.push((*__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    };
                    let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                    assert!(unimplemented!());
                    assert!(i == v.begin());
                    assert!((unsafe { is_contiguous_container_asan_correct_4(&v,) }));
                }
                {
                    let mut v: Vec<i32> = {
                        let mut __v = Vec::new();
                        let mut __it = arr.as_mut_ptr();
                        while __it != arr.as_mut_ptr().offset((3) as isize) {
                            __v.push((*__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    };
                    let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                    assert!(unimplemented!());
                    assert!(i == v.begin());
                    assert!((unsafe { is_contiguous_container_asan_correct_4(&v,) }));
                }
                {
                    let mut v: Vec<i32> = {
                        let mut __v = Vec::new();
                        let mut __it = arr.as_mut_ptr();
                        while __it != arr.as_mut_ptr().offset((3) as isize) {
                            __v.push((*__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    };
                    let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                    assert!(((v.len()) == (0_usize)));
                    assert!(i == v.begin());
                    assert!((unsafe { is_contiguous_container_asan_correct_4(&v,) }));
                }
            }
            {
                let mut v: Vec<i32> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push((*__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                assert!(((v.len()) == (2_usize)));
                assert!(((*(&mut v).as_mut_ptr().add(0_usize as usize)) == (arr[(0) as usize])));
                assert!(((*(&mut v).as_mut_ptr().add(1_usize as usize)) == (arr[(2) as usize])));
                assert!(i == v.begin().iter_add(1_isize));
                assert!((unsafe { is_contiguous_container_asan_correct_4(&v,) }));
            }
            {
                let mut v: Vec<i32> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push((*__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut i: UnsafeRandomAccessIterator<i32> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == v.begin().iter_add(1_isize));
                assert!((unsafe { is_contiguous_container_asan_correct_4(&v,) }));
            }
        }
    }
    {
        let mut outer: Vec<Vec<i32>> =
            vec![(0..(1_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>(); 2_usize as usize];
        unimplemented!();
        assert!(((outer.len()) == (2_usize)));
        assert!((((*(&mut outer).as_mut_ptr().add(0_usize as usize)).len()) == (1_usize)));
        assert!((((*(&mut outer).as_mut_ptr().add(1_usize as usize)).len()) == (1_usize)));
        assert!((unsafe { is_contiguous_container_asan_correct_5(&outer,) }));
        assert!(
            (unsafe {
                is_contiguous_container_asan_correct_4(
                    &(*(&mut outer).as_mut_ptr().add(0_usize as usize)),
                )
            })
        );
        assert!(
            (unsafe {
                is_contiguous_container_asan_correct_4(
                    &(*(&mut outer).as_mut_ptr().add(1_usize as usize)),
                )
            })
        );
    }
    {
        {
            let mut v: Vec<MoveOnly> = Vec::new();
            {
                let __init = MoveOnly::new({ Some(1) });
                v.push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(2) });
                v.push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(3) });
                v.push(__init)
            };
            unimplemented!();
            assert!(((v.len()) == (1_usize)));
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: MoveOnly = MoveOnly::new({ Some(3) });
                    operator_eq_43(_x, &mut _y)
                })
            );
        }
        {
            let mut v: Vec<TrivialMoveOnly> = Vec::new();
            {
                let __init = TrivialMoveOnly::new({ Some(1) });
                v.push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(2) });
                v.push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(3) });
                v.push(__init)
            };
            unimplemented!();
            assert!(((v.len()) == (1_usize)));
            assert!(
                (unsafe {
                    let _x: *const TrivialMoveOnly =
                        &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: TrivialMoveOnly = TrivialMoveOnly::new({ Some(3) });
                    operator_eq_50(_x, &mut _y)
                })
            );
        }
    }
}
pub unsafe fn tests_60() {
    {
        let mut arr: [NonTriviallyRelocatable; 3] = [
            NonTriviallyRelocatable::new_2({ 1 }),
            NonTriviallyRelocatable::new_2({ 2 }),
            NonTriviallyRelocatable::new_2({ 3 }),
        ];
        {
            {
                let mut v: Vec<NonTriviallyRelocatable> = Vec::new();
                let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = unimplemented!();
                assert!(v.is_empty());
                assert!(i == v.end());
                assert!((unsafe { is_contiguous_container_asan_correct_6(&v,) }));
            }
            {
                let mut v: Vec<NonTriviallyRelocatable> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut first: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                    unimplemented!();
                let mut last: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                    unimplemented!();
                let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == last);
                assert!((unsafe { is_contiguous_container_asan_correct_6(&v,) }));
            }
            {
                let mut v: Vec<NonTriviallyRelocatable> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut first: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                    unimplemented!().iter_add(1_isize);
                let mut last: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                    unimplemented!().iter_add(1_isize);
                let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == last);
                assert!((unsafe { is_contiguous_container_asan_correct_6(&v,) }));
            }
            {
                let mut v: Vec<NonTriviallyRelocatable> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut first: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                    unimplemented!();
                let mut last: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                    unimplemented!();
                let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == last);
                assert!((unsafe { is_contiguous_container_asan_correct_6(&v,) }));
            }
        }
        {
            {
                {
                    let mut v: Vec<NonTriviallyRelocatable> = {
                        let mut __v = Vec::new();
                        let mut __it = arr.as_mut_ptr();
                        while __it != arr.as_mut_ptr().offset((3) as isize) {
                            __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    };
                    let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                        unimplemented!();
                    assert!(unimplemented!());
                    assert!(i == v.begin());
                    assert!((unsafe { is_contiguous_container_asan_correct_6(&v,) }));
                }
                {
                    let mut v: Vec<NonTriviallyRelocatable> = {
                        let mut __v = Vec::new();
                        let mut __it = arr.as_mut_ptr();
                        while __it != arr.as_mut_ptr().offset((3) as isize) {
                            __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    };
                    let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                        unimplemented!();
                    assert!(unimplemented!());
                    assert!(i == v.begin());
                    assert!((unsafe { is_contiguous_container_asan_correct_6(&v,) }));
                }
                {
                    let mut v: Vec<NonTriviallyRelocatable> = {
                        let mut __v = Vec::new();
                        let mut __it = arr.as_mut_ptr();
                        while __it != arr.as_mut_ptr().offset((3) as isize) {
                            __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                                NonTriviallyRelocatable,
                            >>::cpp_from(__it.iter_deref()));
                            __it.iter_inc();
                        }
                        __v
                    };
                    let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> =
                        unimplemented!();
                    assert!(((v.len()) == (0_usize)));
                    assert!(i == v.begin());
                    assert!((unsafe { is_contiguous_container_asan_correct_6(&v,) }));
                }
            }
            {
                let mut v: Vec<NonTriviallyRelocatable> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = unimplemented!();
                assert!(((v.len()) == (2_usize)));
                assert!(
                    (unsafe {
                        let _a: *const NonTriviallyRelocatable =
                            &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                        operator_eq_8(_a, &arr[(0) as usize])
                    })
                );
                assert!(
                    (unsafe {
                        let _a: *const NonTriviallyRelocatable =
                            &(*(&mut v).as_mut_ptr().add(1_usize as usize));
                        operator_eq_8(_a, &arr[(2) as usize])
                    })
                );
                assert!(i == v.begin().iter_add(1_isize));
                assert!((unsafe { is_contiguous_container_asan_correct_6(&v,) }));
            }
            {
                let mut v: Vec<NonTriviallyRelocatable> = {
                    let mut __v = Vec::new();
                    let mut __it = arr.as_mut_ptr();
                    while __it != arr.as_mut_ptr().offset((3) as isize) {
                        __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                            NonTriviallyRelocatable,
                        >>::cpp_from(__it.iter_deref()));
                        __it.iter_inc();
                    }
                    __v
                };
                let mut i: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = unimplemented!();
                assert!(unimplemented!());
                assert!(i == v.begin().iter_add(1_isize));
                assert!((unsafe { is_contiguous_container_asan_correct_6(&v,) }));
            }
        }
    }
    {
        let mut outer: Vec<Vec<NonTriviallyRelocatable>> = vec![
            (0..(1_usize) as usize)
                .map(|_| <NonTriviallyRelocatable>::default())
                .collect::<Vec<_>>();
            2_usize as usize
        ];
        unimplemented!();
        assert!(((outer.len()) == (2_usize)));
        assert!((((*(&mut outer).as_mut_ptr().add(0_usize as usize)).len()) == (1_usize)));
        assert!((((*(&mut outer).as_mut_ptr().add(1_usize as usize)).len()) == (1_usize)));
        assert!((unsafe { is_contiguous_container_asan_correct_7(&outer,) }));
        assert!(
            (unsafe {
                is_contiguous_container_asan_correct_6(
                    &(*(&mut outer).as_mut_ptr().add(0_usize as usize)),
                )
            })
        );
        assert!(
            (unsafe {
                is_contiguous_container_asan_correct_6(
                    &(*(&mut outer).as_mut_ptr().add(1_usize as usize)),
                )
            })
        );
    }
    {
        {
            let mut v: Vec<MoveOnly> = Vec::new();
            {
                let __init = MoveOnly::new({ Some(1) });
                v.push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(2) });
                v.push(__init)
            };
            {
                let __init = MoveOnly::new({ Some(3) });
                v.push(__init)
            };
            unimplemented!();
            assert!(((v.len()) == (1_usize)));
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: MoveOnly = MoveOnly::new({ Some(3) });
                    operator_eq_43(_x, &mut _y)
                })
            );
        }
        {
            let mut v: Vec<TrivialMoveOnly> = Vec::new();
            {
                let __init = TrivialMoveOnly::new({ Some(1) });
                v.push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(2) });
                v.push(__init)
            };
            {
                let __init = TrivialMoveOnly::new({ Some(3) });
                v.push(__init)
            };
            unimplemented!();
            assert!(((v.len()) == (1_usize)));
            assert!(
                (unsafe {
                    let _x: *const TrivialMoveOnly =
                        &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: TrivialMoveOnly = TrivialMoveOnly::new({ Some(3) });
                    operator_eq_50(_x, &mut _y)
                })
            );
        }
    }
}
pub unsafe fn tests_61() -> bool {
    (unsafe { tests_57() });
    (unsafe { tests_58() });
    (unsafe { tests_59() });
    (unsafe { tests_60() });
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { tests_61() });
    const _: () = assert!(true, "tests()");;
    {
        let mut v: Vec<Vec<libc::c_char>> = Vec::new();
        let mut short_string: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"short".as_ptr(),
            libcc2rs::c_str_len(c"short".as_ptr()) + 1,
        )
        .to_vec();
        let mut long_string: Vec<libc::c_char> = vec![('x' as libc::c_char); (256_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once((0 as libc::c_char)))
            .collect();
        let mut i: i32 = 0;
        'loop_: while ((i) != (10)) {
            {
                let a0_clone = if (((i) % (2)) == (0)) {
                    short_string
                } else {
                    long_string
                }
                .clone();
                v.push(a0_clone)
            };
            i.prefix_inc();
        }
        let mut original: Vec<Vec<libc::c_char>> = v.clone();
        let mut it: UnsafeRandomAccessIterator<Vec<libc::c_char>> = unimplemented!();
        assert!(((v.len()) == (4_usize)));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(it == v.begin().iter_add(2_isize));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_9);
    std::cell::LazyCell::force(&*&raw const alloc_count_10);
    std::cell::LazyCell::force(&*&raw const dealloc_count_11);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_12);
}
