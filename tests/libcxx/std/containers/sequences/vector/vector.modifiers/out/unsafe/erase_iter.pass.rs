extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_contiguous_container_asan_correct_0(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_1(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_2(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_3(_a0: *const ()) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Tracker {
    pub copy_assignments: (),
    pub move_assignments: (),
}
impl Default for Tracker {
    fn default() -> Self {
        unsafe {
            Tracker {
                copy_assignments: 0_(),
                move_assignments: 0_(),
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
) -> () {
    return (((*a).value_) == ((*b).value_));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CopyAssignUnsafe, MoveAssignUnsafe)]
pub struct NonTriviallyRelocatable {
    pub value_: (),
}
impl NonTriviallyRelocatable {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { value_: 0_() };
        this
    }
    pub unsafe fn new_2(mut v: ()) -> Self {
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
pub unsafe fn operator_sub_9(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_10(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_11(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_10(_x, y)
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
    pub unsafe fn to_(&self) -> () {
        return !((self.ptr_).is_null());
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
        unsafe { operator_eq_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_12(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_ne_13(
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_14(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<MoveOnly>()) as () as ());
}
pub unsafe fn operator_eq_15(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_16(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
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
    pub unsafe fn to_(&self) -> () {
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
        mut n: (),
    ) -> *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
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
        mut n: (),
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
pub unsafe fn operator_sub_17(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<MoveOnly>()) as () as ());
}
pub unsafe fn operator_eq_18(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
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
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<TrivialMoveOnly>()) as ()
        as ());
}
pub unsafe fn operator_eq_20(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_21(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
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
    pub unsafe fn to_(&self) -> () {
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
        mut n: (),
    ) -> *mut min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
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
        mut n: (),
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
pub unsafe fn operator_sub_22(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<TrivialMoveOnly>()) as ()
        as ());
}
pub unsafe fn operator_eq_23(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
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
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<NonTriviallyRelocatable>())
        as () as ());
}
pub unsafe fn operator_eq_25(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_26(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
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
    pub unsafe fn to_(&self) -> () {
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
        mut n: (),
    ) -> *mut min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return & mut ( * ( self as *mut min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__  )  )  ;
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
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
pub unsafe fn operator_sub_27(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<NonTriviallyRelocatable>())
        as () as ());
}
pub unsafe fn operator_ne_28(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
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
        mut n: (),
    ) -> *mut min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
    {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return & mut ( * ( self as *mut min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__  )  )  ;
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        let mut tmp : min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__  )  ) ;
        (unsafe {
            min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , )
        });
        return tmp;
    }
}
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
pub struct min_allocator_MoveOnly_ {}
impl min_allocator_MoveOnly_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: (),
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
        mut n: (),
    ) -> min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
        return min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
        mut n: (),
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
        mut n: (),
    ) -> min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ {
        return min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub unsafe fn operator_eq_29(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_30(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_31(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_32(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_33(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_34(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_35(_a0: *const MoveOnly, _a1: *const MoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {
    data_: (),
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0_();
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> () {
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
pub unsafe fn operator_eq_36(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_37(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_38(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_39(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_40(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_41(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_42(_a0: *const TrivialMoveOnly, _a1: *const TrivialMoveOnly) -> () {
    {
        let mut cmp: () = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct TrivialMoveOnly {
    data_: (),
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
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
    pub unsafe fn get(&self) -> () {
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
        let mut arr: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        {
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut expected: [(); 4] = [1_(), 3_(), 4_(), 5_()];
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut expected: [(); 4] = [1_(), 2_(), 3_(), 4_()];
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn tests_44() {
    {
        let mut arr: [NonTriviallyRelocatable; 5] = [
            NonTriviallyRelocatable::new_2({ 1_() }),
            NonTriviallyRelocatable::new_2({ 2_() }),
            NonTriviallyRelocatable::new_2({ 3_() }),
            NonTriviallyRelocatable::new_2({ 4_() }),
            NonTriviallyRelocatable::new_2({ 5_() }),
        ];
        {
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut expected: [NonTriviallyRelocatable; 4] = [
                NonTriviallyRelocatable::new_2({ 1_() }),
                NonTriviallyRelocatable::new_2({ 3_() }),
                NonTriviallyRelocatable::new_2({ 4_() }),
                NonTriviallyRelocatable::new_2({ 5_() }),
            ];
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut expected: [NonTriviallyRelocatable; 4] = [
                NonTriviallyRelocatable::new_2({ 1_() }),
                NonTriviallyRelocatable::new_2({ 2_() }),
                NonTriviallyRelocatable::new_2({ 3_() }),
                NonTriviallyRelocatable::new_2({ 4_() }),
            ];
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn tests_45() {
    {
        let mut arr: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        {
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut expected: [(); 4] = [1_(), 3_(), 4_(), 5_()];
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut expected: [(); 4] = [1_(), 2_(), 3_(), 4_()];
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn tests_46() {
    {
        let mut arr: [NonTriviallyRelocatable; 5] = [
            NonTriviallyRelocatable::new_2({ 1_() }),
            NonTriviallyRelocatable::new_2({ 2_() }),
            NonTriviallyRelocatable::new_2({ 3_() }),
            NonTriviallyRelocatable::new_2({ 4_() }),
            NonTriviallyRelocatable::new_2({ 5_() }),
        ];
        {
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut expected: [NonTriviallyRelocatable; 4] = [
                NonTriviallyRelocatable::new_2({ 1_() }),
                NonTriviallyRelocatable::new_2({ 3_() }),
                NonTriviallyRelocatable::new_2({ 4_() }),
                NonTriviallyRelocatable::new_2({ 5_() }),
            ];
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut expected: [NonTriviallyRelocatable; 4] = [
                NonTriviallyRelocatable::new_2({ 1_() }),
                NonTriviallyRelocatable::new_2({ 2_() }),
                NonTriviallyRelocatable::new_2({ 3_() }),
                NonTriviallyRelocatable::new_2({ 4_() }),
            ];
            let mut v: () = unimplemented!();
            let mut it: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn tests_47() -> () {
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { tests_47() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_5);
    std::cell::LazyCell::force(&*&raw const alloc_count_6);
    std::cell::LazyCell::force(&*&raw const dealloc_count_7);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_8);
}
