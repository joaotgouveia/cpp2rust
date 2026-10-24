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
pub unsafe fn is_contiguous_container_asan_correct_4(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_5(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_6(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_contiguous_container_asan_correct_7(_a0: *const ()) -> () {
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
pub unsafe fn operator_eq_8(
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
const _: () = assert!(
    true,
    "!std::__libcpp_is_trivially_relocatable<NonTriviallyRelocatable>::value"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_12)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_9)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_10)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_11)) = 0_();
    }
}
pub static mut outstanding_bytes_9: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_10: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_13(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_14(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_15(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_14(_x, y)
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
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
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
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_14(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_16(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_17(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_18(
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
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
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
}
impl std::cmp::PartialEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_17(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_19(
    mut x : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_20(
    mut x : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_21(
    mut x : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_20(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut (),
}
impl min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }    pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__{
        self.ptr_.prefix_inc();
        return & mut ( * ( self as *mut min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  )  )  ;
    }    pub unsafe fn operator_dec_10 ( &mut self , ) -> *mut min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__{
        self.ptr_.prefix_dec();
        return & mut ( * ( self as *mut min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  )  )  ;
    }    pub unsafe fn operator_add_assign ( &mut self , mut n : () , ) -> *mut min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__{
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return & mut ( * ( self as *mut min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  )  )  ;
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
    {
        let mut tmp : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__  )  ) ;
        (unsafe {
            min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , )
        });
        return tmp;
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
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
pub unsafe fn operator_sub_22(
    mut x : min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_23(
    mut x : min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_24(
    mut x : min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
{
    ptr_: *const (),
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
impl std::cmp::PartialEq for min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_23(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_const_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_25(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<MoveOnly>()) as () as ());
}
pub unsafe fn operator_eq_26(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_27(
    mut x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_26(_x, y)
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
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
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
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut MoveOnly {
        return &mut (*self.ptr_.offset((n) as isize));
    }
}
impl std::cmp::PartialEq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_26(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_MoveOnly__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_28(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<MoveOnly>()) as () as ());
}
pub unsafe fn operator_eq_29(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_30(
    mut x: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
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
        unsafe { operator_eq_29(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_MoveOnly__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_31(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<TrivialMoveOnly>()) as ()
        as ());
}
pub unsafe fn operator_eq_32(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_33(
    mut x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_32(_x, y)
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
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
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
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut TrivialMoveOnly {
        return &mut (*self.ptr_.offset((n) as isize));
    }
}
impl std::cmp::PartialEq for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_32(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_34(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<TrivialMoveOnly>()) as ()
        as ());
}
pub unsafe fn operator_eq_35(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_36(
    mut x: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
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
        unsafe { operator_eq_35(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_TrivialMoveOnly__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_37(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<NonTriviallyRelocatable>())
        as () as ());
}
pub unsafe fn operator_eq_38(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_39(
    mut x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_38(_x, y)
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
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
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
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut NonTriviallyRelocatable {
        return &mut (*self.ptr_.offset((n) as isize));
    }
}
impl std::cmp::PartialEq
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_38(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq
    for min_pointer_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
}
pub unsafe fn operator_sub_40(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<NonTriviallyRelocatable>())
        as () as ());
}
pub unsafe fn operator_eq_41(
    mut x: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_42(
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
    pub unsafe fn operator_arrow(&self) -> *const NonTriviallyRelocatable {
        return self.ptr_;
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
}
impl std::cmp::PartialEq
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_41(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq
    for min_pointer_const_NonTriviallyRelocatable__std_integral_constant_unsigned_long__0__
{
}
pub unsafe fn operator_sub_43(
    mut x : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_44(
    mut x : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_45(
    mut x : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_44(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut (),
}
impl min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut () , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn operator_deref ( &self , ) -> *mut () { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut () { return self . ptr_   ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_10 ( &mut self , ) -> *mut min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : () , ) -> *mut min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : () , ) -> min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } pub unsafe fn operator_index ( &self , mut n : () , ) -> *mut () { return & mut ( * self . ptr_  . offset ( ( n ) as isize ) )  ;
 } }
impl std::cmp::PartialEq for min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_44(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_46(
    mut x : min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_47(
    mut x : min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_48(
    mut x : min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__
{
    ptr_: *const (),
}
impl min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { pub unsafe fn new_1 ( mut p : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : ( p . ptr_  ) .cast_const() , } ;
 this } }
impl std::cmp::PartialEq for min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_47(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_const_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_49(mut _a0: min_allocator_int_, mut _a1: min_allocator_int_) -> () {
    return true;
}
pub unsafe fn operator_ne_50(mut x: min_allocator_int_, mut y: min_allocator_int_) -> () {
    return !(unsafe {
        let _arg0: min_allocator_int_ = x;
        operator_eq_49(_arg0, y)
    });
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
impl std::cmp::PartialEq for min_allocator_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_49(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_vector_int__min_allocator_int___ {}
impl min_allocator_std_vector_int__min_allocator_int___ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__
    {
        return min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_std_vector_int__min_allocator_int____std_integral_constant_unsigned_long__0__,
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
pub unsafe fn operator_eq_51(
    mut _a0: min_allocator_NonTriviallyRelocatable_,
    mut _a1: min_allocator_NonTriviallyRelocatable_,
) -> () {
    return true;
}
pub unsafe fn operator_ne_52(
    mut x: min_allocator_NonTriviallyRelocatable_,
    mut y: min_allocator_NonTriviallyRelocatable_,
) -> () {
    return !(unsafe {
        let _arg0: min_allocator_NonTriviallyRelocatable_ = x;
        operator_eq_51(_arg0, y)
    });
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
impl std::cmp::PartialEq for min_allocator_NonTriviallyRelocatable_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_51(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_allocator_NonTriviallyRelocatable_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable___
{}
impl min_allocator_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable___ {
    pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__{
        return min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_std_vector_NonTriviallyRelocatable__min_allocator_NonTriviallyRelocatable____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
pub unsafe fn operator_eq_53(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_54(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_55(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_56(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_57(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_58(x: *const MoveOnly, y: *const MoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_59(_a0: *const MoveOnly, _a1: *const MoveOnly) -> () {
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
        unsafe { operator_cmp_59(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_53(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_60(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_61(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_62(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_63(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_64(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_65(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> () {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_66(_a0: *const TrivialMoveOnly, _a1: *const TrivialMoveOnly) -> () {
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
            operator_cmp_66(
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
            operator_eq_60(
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
pub unsafe fn tests_67() {
    {
        let mut arr: [(); 3] = [1_(), 2_(), 3_()];
        {
            {
                let mut v: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut first: () = unimplemented!();
                let mut last: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut first: () = unimplemented!();
                let mut last: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut first: () = unimplemented!();
                let mut last: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                {
                    let mut v: () = unimplemented!();
                    let mut i: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut v: () = unimplemented!();
                    let mut i: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut v: () = unimplemented!();
                    let mut i: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
            }
            {
                let mut v: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
    }
    {
        let mut outer: () = unimplemented!();
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
            let mut v: () = unimplemented!();
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
        }
    }
}
pub unsafe fn tests_68() {
    {
        let mut arr: [NonTriviallyRelocatable; 3] = [
            NonTriviallyRelocatable::new_2({ 1_() }),
            NonTriviallyRelocatable::new_2({ 2_() }),
            NonTriviallyRelocatable::new_2({ 3_() }),
        ];
        {
            {
                let mut v: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut first: () = unimplemented!();
                let mut last: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut first: () = unimplemented!();
                let mut last: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut first: () = unimplemented!();
                let mut last: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                {
                    let mut v: () = unimplemented!();
                    let mut i: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut v: () = unimplemented!();
                    let mut i: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut v: () = unimplemented!();
                    let mut i: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
            }
            {
                let mut v: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
    }
    {
        let mut outer: () = unimplemented!();
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
            let mut v: () = unimplemented!();
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
        }
    }
}
pub unsafe fn tests_69() {
    {
        let mut arr: [(); 3] = [1_(), 2_(), 3_()];
        {
            {
                let mut v: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut first: () = unimplemented!();
                let mut last: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut first: () = unimplemented!();
                let mut last: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut first: () = unimplemented!();
                let mut last: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                {
                    let mut v: () = unimplemented!();
                    let mut i: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut v: () = unimplemented!();
                    let mut i: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut v: () = unimplemented!();
                    let mut i: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
            }
            {
                let mut v: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
    }
    {
        let mut outer: () = unimplemented!();
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
            let mut v: () = unimplemented!();
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
        }
    }
}
pub unsafe fn tests_70() {
    {
        let mut arr: [NonTriviallyRelocatable; 3] = [
            NonTriviallyRelocatable::new_2({ 1_() }),
            NonTriviallyRelocatable::new_2({ 2_() }),
            NonTriviallyRelocatable::new_2({ 3_() }),
        ];
        {
            {
                let mut v: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut first: () = unimplemented!();
                let mut last: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut first: () = unimplemented!();
                let mut last: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut first: () = unimplemented!();
                let mut last: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                {
                    let mut v: () = unimplemented!();
                    let mut i: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut v: () = unimplemented!();
                    let mut i: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut v: () = unimplemented!();
                    let mut i: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
            }
            {
                let mut v: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut v: () = unimplemented!();
                let mut i: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
    }
    {
        let mut outer: () = unimplemented!();
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
            let mut v: () = unimplemented!();
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
        }
    }
}
pub unsafe fn tests_71() -> () {
    (unsafe { tests_67() });
    (unsafe { tests_68() });
    (unsafe { tests_69() });
    (unsafe { tests_70() });
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
    (unsafe { tests_71() });
    const _: () = assert!(true, "tests()");;
    {
        let mut v: () = unimplemented!();
        let mut short_string: () = unimplemented!();
        let mut long_string: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut original: () = unimplemented!();
        let mut it: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut tracker: Tracker = <Tracker>::default();
        let mut v: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (5_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_9);
    std::cell::LazyCell::force(&*&raw const alloc_count_10);
    std::cell::LazyCell::force(&*&raw const dealloc_count_11);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_12);
}
