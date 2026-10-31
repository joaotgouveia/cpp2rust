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
pub unsafe fn is_contiguous_container_asan_correct_1(
    _a0: *const Vec<NonTriviallyRelocatable>,
) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_2(_a0: *const Vec<i32>) -> bool {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_3(
    _a0: *const Vec<NonTriviallyRelocatable>,
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
pub unsafe fn operator_eq_4(
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
            operator_eq_4(
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
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_6))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_7)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_6))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_7))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_8)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_5)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_6)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_7)) = 0_usize;
    }
}
pub static mut outstanding_bytes_5: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_6: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_7: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_8: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_9(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_10(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_11(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_10(_x, y)
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
    pub unsafe fn operator_arrow(&self) -> *mut i32 {
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
        unsafe { operator_eq_10(self.clone(), other.clone()) }
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
impl UnsafeCppIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn iter_deref(&self) -> *mut i32 {
        unsafe {
            min_pointer_int__std_integral_constant_unsigned_long__0__::operator_deref(self)
                as *mut i32
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            min_pointer_int__std_integral_constant_unsigned_long__0__::operator_inc_8(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<i32> for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppBidirectionalIterator<i32>
    for min_pointer_int__std_integral_constant_unsigned_long__0__
{
    fn iter_dec(&mut self) -> &mut Self {
        unsafe {
            min_pointer_int__std_integral_constant_unsigned_long__0__::operator_dec_10(self);
        }
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_12(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_ne_13(
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
        unsafe { !(operator_ne_13(self.clone(), other.clone())) }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
impl UnsafeCppIterator<i32> for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn iter_deref(&self) -> *mut i32 {
        unsafe {
            min_pointer_const_int__std_integral_constant_unsigned_long__0__::operator_deref(self)
                as *mut i32
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            min_pointer_const_int__std_integral_constant_unsigned_long__0__::operator_inc_9(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<i32>
    for min_pointer_const_int__std_integral_constant_unsigned_long__0__
{
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *const ::libc::c_void,
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_14(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<MoveOnly>()) as i64
        as isize);
}
pub unsafe fn operator_eq_15(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_16(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_15(_x, y)
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
    pub unsafe fn operator_dec_10(
        &mut self,
    ) -> *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_dec();
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
        unsafe { operator_eq_15(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_10();
    }
}
impl UnsafeCppIterator<MoveOnly>
    for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__
{
    fn iter_deref(&self) -> *mut MoveOnly {
        unsafe {
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::operator_deref(self)
                as *mut MoveOnly
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::operator_inc_8(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<MoveOnly>
    for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__
{
}
impl UnsafeCppBidirectionalIterator<MoveOnly>
    for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__
{
    fn iter_dec(&mut self) -> &mut Self {
        unsafe {
            min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::operator_dec_10(self);
        }
        self
    }
}
pub unsafe fn operator_sub_17(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<MoveOnly>()) as i64
        as isize);
}
pub unsafe fn operator_eq_18(
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
        unsafe { operator_eq_18(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_19(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<TrivialMoveOnly>()) as i64
        as isize);
}
pub unsafe fn operator_eq_20(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_21(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_20(_x, y)
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
    pub unsafe fn operator_dec_10(
        &mut self,
    ) -> *mut min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_dec();
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
        unsafe { operator_eq_20(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_10();
    }
}
impl UnsafeCppIterator<TrivialMoveOnly>
    for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__
{
    fn iter_deref(&self) -> *mut TrivialMoveOnly {
        unsafe {
            min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__::operator_deref(
                self,
            ) as *mut TrivialMoveOnly
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__::operator_inc_8(
                self,
            );
        }
        self
    }
}
impl UnsafeCppInputIterator<TrivialMoveOnly>
    for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__
{
}
impl UnsafeCppBidirectionalIterator<TrivialMoveOnly>
    for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__
{
    fn iter_dec(&mut self) -> &mut Self {
        unsafe {
            min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__::operator_dec_10(
                self,
            );
        }
        self
    }
}
pub unsafe fn operator_sub_22(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<TrivialMoveOnly>()) as i64
        as isize);
}
pub unsafe fn operator_eq_23(
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
        unsafe { operator_eq_23(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_24(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<NonTriviallyRelocatable>())
        as i64 as isize);
}
pub unsafe fn operator_eq_25(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_26(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_25(_x, y)
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
    pub unsafe fn operator_arrow(&self) -> *mut NonTriviallyRelocatable {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return & mut ( * ( self as *mut min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__  )  )  ;
    }
    pub unsafe fn operator_dec_10(
        &mut self,
    ) -> *mut min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_dec();
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
        unsafe { operator_eq_25(self.clone(), other.clone()) }
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
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_10();
    }
}
impl UnsafeCppIterator<NonTriviallyRelocatable>
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn iter_deref(&self) -> *mut NonTriviallyRelocatable {
        unsafe {
            min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__::operator_deref(self) as *mut NonTriviallyRelocatable
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__::operator_inc_8(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<NonTriviallyRelocatable>
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
}
impl UnsafeCppBidirectionalIterator<NonTriviallyRelocatable>
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn iter_dec(&mut self) -> &mut Self {
        unsafe {
            min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__::operator_dec_10(self);
        }
        self
    }
}
pub unsafe fn operator_sub_27(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<NonTriviallyRelocatable>())
        as i64 as isize);
}
pub unsafe fn operator_ne_28(
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
        unsafe { !(operator_ne_28(self.clone(), other.clone())) }
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
impl UnsafeCppIterator<NonTriviallyRelocatable>
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn iter_deref(&self) -> *mut NonTriviallyRelocatable {
        unsafe {
            min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__::operator_deref(self) as *mut NonTriviallyRelocatable
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__::operator_inc_9(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<NonTriviallyRelocatable>
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
}
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
pub unsafe fn operator_eq_29(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_30(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_31(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_32(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_33(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_34(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_35(_a0: *const MoveOnly, _a1: *const MoveOnly) -> std::cmp::Ordering {
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
        unsafe { operator_cmp_35(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_29(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_36(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_37(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_38(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_39(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_40(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_41(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_42(
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
            operator_cmp_42(
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
            operator_eq_36(
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
pub unsafe fn tests_43() {
    {
        let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
        {
            let mut v: Vec<i32> = {
                let mut __v = Vec::new();
                let mut __it = arr.as_mut_ptr();
                while __it != arr.as_mut_ptr().offset((5) as isize) {
                    __v.push((*__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            };
            let mut it: UnsafeRandomAccessIterator<i32> = {
                let __it = unimplemented!();
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(unimplemented!());
            assert!(it == v.begin());
            assert!((unsafe { is_contiguous_container_asan_correct_0(&v,) }));
        }
        {
            let mut expected: [i32; 4] = [1, 3, 4, 5];
            let mut v: Vec<i32> = {
                let mut __v = Vec::new();
                let mut __it = arr.as_mut_ptr();
                while __it != arr.as_mut_ptr().offset((5) as isize) {
                    __v.push((*__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            };
            let mut it: UnsafeRandomAccessIterator<i32> = {
                let __it = unimplemented!().iter_add(1_isize);
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(unimplemented!());
            assert!(it == v.begin().iter_add(1_isize));
            assert!((unsafe { is_contiguous_container_asan_correct_0(&v,) }));
        }
        {
            let mut expected: [i32; 4] = [1, 2, 3, 4];
            let mut v: Vec<i32> = {
                let mut __v = Vec::new();
                let mut __it = arr.as_mut_ptr();
                while __it != arr.as_mut_ptr().offset((5) as isize) {
                    __v.push((*__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            };
            let mut it: UnsafeRandomAccessIterator<i32> = {
                let __it = unimplemented!().iter_add(4_isize);
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(unimplemented!());
            assert!(it == v.end());
            assert!((unsafe { is_contiguous_container_asan_correct_0(&v,) }));
        }
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
            {
                let __it = <UnsafeRandomAccessIterator<MoveOnly>>::from(v.begin());
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(((v.len()) == (2_usize)));
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: MoveOnly = MoveOnly::new({ Some(2) });
                    operator_eq_29(_x, &mut _y)
                })
            );
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*(&mut v).as_mut_ptr().add(1_usize as usize));
                    let mut _y: MoveOnly = MoveOnly::new({ Some(3) });
                    operator_eq_29(_x, &mut _y)
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
            {
                let __it = <UnsafeRandomAccessIterator<TrivialMoveOnly>>::from(v.begin());
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(((v.len()) == (2_usize)));
            assert!(
                (unsafe {
                    let _x: *const TrivialMoveOnly =
                        &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: TrivialMoveOnly = TrivialMoveOnly::new({ Some(2) });
                    operator_eq_36(_x, &mut _y)
                })
            );
            assert!(
                (unsafe {
                    let _x: *const TrivialMoveOnly =
                        &(*(&mut v).as_mut_ptr().add(1_usize as usize));
                    let mut _y: TrivialMoveOnly = TrivialMoveOnly::new({ Some(3) });
                    operator_eq_36(_x, &mut _y)
                })
            );
        }
    }
}
pub unsafe fn tests_44() {
    {
        let mut arr: [NonTriviallyRelocatable; 5] = [
            NonTriviallyRelocatable::new_2({ 1 }),
            NonTriviallyRelocatable::new_2({ 2 }),
            NonTriviallyRelocatable::new_2({ 3 }),
            NonTriviallyRelocatable::new_2({ 4 }),
            NonTriviallyRelocatable::new_2({ 5 }),
        ];
        {
            let mut v: Vec<NonTriviallyRelocatable> = {
                let mut __v = Vec::new();
                let mut __it = arr.as_mut_ptr();
                while __it != arr.as_mut_ptr().offset((5) as isize) {
                    __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                        NonTriviallyRelocatable,
                    >>::cpp_from(__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            };
            let mut it: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = {
                let __it = unimplemented!();
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(unimplemented!());
            assert!(it == v.begin());
            assert!((unsafe { is_contiguous_container_asan_correct_1(&v,) }));
        }
        {
            let mut expected: [NonTriviallyRelocatable; 4] = [
                NonTriviallyRelocatable::new_2({ 1 }),
                NonTriviallyRelocatable::new_2({ 3 }),
                NonTriviallyRelocatable::new_2({ 4 }),
                NonTriviallyRelocatable::new_2({ 5 }),
            ];
            let mut v: Vec<NonTriviallyRelocatable> = {
                let mut __v = Vec::new();
                let mut __it = arr.as_mut_ptr();
                while __it != arr.as_mut_ptr().offset((5) as isize) {
                    __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                        NonTriviallyRelocatable,
                    >>::cpp_from(__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            };
            let mut it: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = {
                let __it = unimplemented!().iter_add(1_isize);
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(unimplemented!());
            assert!(it == v.begin().iter_add(1_isize));
            assert!((unsafe { is_contiguous_container_asan_correct_1(&v,) }));
        }
        {
            let mut expected: [NonTriviallyRelocatable; 4] = [
                NonTriviallyRelocatable::new_2({ 1 }),
                NonTriviallyRelocatable::new_2({ 2 }),
                NonTriviallyRelocatable::new_2({ 3 }),
                NonTriviallyRelocatable::new_2({ 4 }),
            ];
            let mut v: Vec<NonTriviallyRelocatable> = {
                let mut __v = Vec::new();
                let mut __it = arr.as_mut_ptr();
                while __it != arr.as_mut_ptr().offset((5) as isize) {
                    __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                        NonTriviallyRelocatable,
                    >>::cpp_from(__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            };
            let mut it: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = {
                let __it = unimplemented!().iter_add(4_isize);
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(unimplemented!());
            assert!(it == v.end());
            assert!((unsafe { is_contiguous_container_asan_correct_1(&v,) }));
        }
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
            {
                let __it = <UnsafeRandomAccessIterator<MoveOnly>>::from(v.begin());
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(((v.len()) == (2_usize)));
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: MoveOnly = MoveOnly::new({ Some(2) });
                    operator_eq_29(_x, &mut _y)
                })
            );
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*(&mut v).as_mut_ptr().add(1_usize as usize));
                    let mut _y: MoveOnly = MoveOnly::new({ Some(3) });
                    operator_eq_29(_x, &mut _y)
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
            {
                let __it = <UnsafeRandomAccessIterator<TrivialMoveOnly>>::from(v.begin());
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(((v.len()) == (2_usize)));
            assert!(
                (unsafe {
                    let _x: *const TrivialMoveOnly =
                        &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: TrivialMoveOnly = TrivialMoveOnly::new({ Some(2) });
                    operator_eq_36(_x, &mut _y)
                })
            );
            assert!(
                (unsafe {
                    let _x: *const TrivialMoveOnly =
                        &(*(&mut v).as_mut_ptr().add(1_usize as usize));
                    let mut _y: TrivialMoveOnly = TrivialMoveOnly::new({ Some(3) });
                    operator_eq_36(_x, &mut _y)
                })
            );
        }
    }
}
pub unsafe fn tests_45() {
    {
        let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
        {
            let mut v: Vec<i32> = {
                let mut __v = Vec::new();
                let mut __it = arr.as_mut_ptr();
                while __it != arr.as_mut_ptr().offset((5) as isize) {
                    __v.push((*__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            };
            let mut it: UnsafeRandomAccessIterator<i32> = {
                let __it = unimplemented!();
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(unimplemented!());
            assert!(it == v.begin());
            assert!((unsafe { is_contiguous_container_asan_correct_2(&v,) }));
        }
        {
            let mut expected: [i32; 4] = [1, 3, 4, 5];
            let mut v: Vec<i32> = {
                let mut __v = Vec::new();
                let mut __it = arr.as_mut_ptr();
                while __it != arr.as_mut_ptr().offset((5) as isize) {
                    __v.push((*__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            };
            let mut it: UnsafeRandomAccessIterator<i32> = {
                let __it = unimplemented!().iter_add(1_isize);
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(unimplemented!());
            assert!(it == v.begin().iter_add(1_isize));
            assert!((unsafe { is_contiguous_container_asan_correct_2(&v,) }));
        }
        {
            let mut expected: [i32; 4] = [1, 2, 3, 4];
            let mut v: Vec<i32> = {
                let mut __v = Vec::new();
                let mut __it = arr.as_mut_ptr();
                while __it != arr.as_mut_ptr().offset((5) as isize) {
                    __v.push((*__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            };
            let mut it: UnsafeRandomAccessIterator<i32> = {
                let __it = unimplemented!().iter_add(4_isize);
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(unimplemented!());
            assert!(it == v.end());
            assert!((unsafe { is_contiguous_container_asan_correct_2(&v,) }));
        }
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
            {
                let __it = <()>::from(v.begin());
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(((v.len()) == (2_usize)));
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: MoveOnly = MoveOnly::new({ Some(2) });
                    operator_eq_29(_x, &mut _y)
                })
            );
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*(&mut v).as_mut_ptr().add(1_usize as usize));
                    let mut _y: MoveOnly = MoveOnly::new({ Some(3) });
                    operator_eq_29(_x, &mut _y)
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
            {
                let __it = <()>::from(v.begin());
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(((v.len()) == (2_usize)));
            assert!(
                (unsafe {
                    let _x: *const TrivialMoveOnly =
                        &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: TrivialMoveOnly = TrivialMoveOnly::new({ Some(2) });
                    operator_eq_36(_x, &mut _y)
                })
            );
            assert!(
                (unsafe {
                    let _x: *const TrivialMoveOnly =
                        &(*(&mut v).as_mut_ptr().add(1_usize as usize));
                    let mut _y: TrivialMoveOnly = TrivialMoveOnly::new({ Some(3) });
                    operator_eq_36(_x, &mut _y)
                })
            );
        }
    }
}
pub unsafe fn tests_46() {
    {
        let mut arr: [NonTriviallyRelocatable; 5] = [
            NonTriviallyRelocatable::new_2({ 1 }),
            NonTriviallyRelocatable::new_2({ 2 }),
            NonTriviallyRelocatable::new_2({ 3 }),
            NonTriviallyRelocatable::new_2({ 4 }),
            NonTriviallyRelocatable::new_2({ 5 }),
        ];
        {
            let mut v: Vec<NonTriviallyRelocatable> = {
                let mut __v = Vec::new();
                let mut __it = arr.as_mut_ptr();
                while __it != arr.as_mut_ptr().offset((5) as isize) {
                    __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                        NonTriviallyRelocatable,
                    >>::cpp_from(__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            };
            let mut it: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = {
                let __it = unimplemented!();
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(unimplemented!());
            assert!(it == v.begin());
            assert!((unsafe { is_contiguous_container_asan_correct_3(&v,) }));
        }
        {
            let mut expected: [NonTriviallyRelocatable; 4] = [
                NonTriviallyRelocatable::new_2({ 1 }),
                NonTriviallyRelocatable::new_2({ 3 }),
                NonTriviallyRelocatable::new_2({ 4 }),
                NonTriviallyRelocatable::new_2({ 5 }),
            ];
            let mut v: Vec<NonTriviallyRelocatable> = {
                let mut __v = Vec::new();
                let mut __it = arr.as_mut_ptr();
                while __it != arr.as_mut_ptr().offset((5) as isize) {
                    __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                        NonTriviallyRelocatable,
                    >>::cpp_from(__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            };
            let mut it: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = {
                let __it = unimplemented!().iter_add(1_isize);
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(unimplemented!());
            assert!(it == v.begin().iter_add(1_isize));
            assert!((unsafe { is_contiguous_container_asan_correct_3(&v,) }));
        }
        {
            let mut expected: [NonTriviallyRelocatable; 4] = [
                NonTriviallyRelocatable::new_2({ 1 }),
                NonTriviallyRelocatable::new_2({ 2 }),
                NonTriviallyRelocatable::new_2({ 3 }),
                NonTriviallyRelocatable::new_2({ 4 }),
            ];
            let mut v: Vec<NonTriviallyRelocatable> = {
                let mut __v = Vec::new();
                let mut __it = arr.as_mut_ptr();
                while __it != arr.as_mut_ptr().offset((5) as isize) {
                    __v.push(<NonTriviallyRelocatable as CppFromUnsafe<
                        NonTriviallyRelocatable,
                    >>::cpp_from(__it.iter_deref()));
                    __it.iter_inc();
                }
                __v
            };
            let mut it: UnsafeRandomAccessIterator<NonTriviallyRelocatable> = {
                let __it = unimplemented!().iter_add(4_isize);
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(unimplemented!());
            assert!(it == v.end());
            assert!((unsafe { is_contiguous_container_asan_correct_3(&v,) }));
        }
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
            {
                let __it = <()>::from(v.begin());
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(((v.len()) == (2_usize)));
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: MoveOnly = MoveOnly::new({ Some(2) });
                    operator_eq_29(_x, &mut _y)
                })
            );
            assert!(
                (unsafe {
                    let _x: *const MoveOnly = &(*(&mut v).as_mut_ptr().add(1_usize as usize));
                    let mut _y: MoveOnly = MoveOnly::new({ Some(3) });
                    operator_eq_29(_x, &mut _y)
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
            {
                let __it = <()>::from(v.begin());
                let pos =
                    __it.iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr())) as usize;
                v.remove(pos);
                __it
            };
            assert!(((v.len()) == (2_usize)));
            assert!(
                (unsafe {
                    let _x: *const TrivialMoveOnly =
                        &(*(&mut v).as_mut_ptr().add(0_usize as usize));
                    let mut _y: TrivialMoveOnly = TrivialMoveOnly::new({ Some(2) });
                    operator_eq_36(_x, &mut _y)
                })
            );
            assert!(
                (unsafe {
                    let _x: *const TrivialMoveOnly =
                        &(*(&mut v).as_mut_ptr().add(1_usize as usize));
                    let mut _y: TrivialMoveOnly = TrivialMoveOnly::new({ Some(3) });
                    operator_eq_36(_x, &mut _y)
                })
            );
        }
    }
}
pub unsafe fn tests_47() -> bool {
    (unsafe { tests_43() });
    (unsafe { tests_44() });
    (unsafe { tests_45() });
    (unsafe { tests_46() });
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
    (unsafe { tests_47() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_5);
    std::cell::LazyCell::force(&*&raw const alloc_count_6);
    std::cell::LazyCell::force(&*&raw const dealloc_count_7);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_8);
}
