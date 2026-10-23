extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn DoNotOptimize_0(value: *const ()) -> *const () {
    c"";
    return value;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_1(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_2(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_double_ended_contiguous_container_asan_correct_3(_a0: *const ()) -> () {
    return true;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct Emplaceable {
    int_: (),
    double_: (),
}
impl Emplaceable {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            int_: 0_(),
            double_: 0_(),
        };
        this
    }
    pub unsafe fn new_2(mut i: (), mut d: ()) -> Self {
        let mut this = Self {
            int_: i,
            double_: d,
        };
        this
    }
    pub unsafe fn move_from(x: *mut Emplaceable) -> Self {
        let mut this = Self {
            int_: (*x).int_,
            double_: (*x).double_,
        };
        (*x).int_ = 0_();
        (*x).double_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut Emplaceable) -> *mut Emplaceable {
        self.int_ = (*x).int_;
        (*x).int_ = 0_();
        self.double_ = (*x).double_;
        (*x).double_ = 0_();
        return &mut (*(self as *mut Emplaceable));
    }
    pub unsafe fn operator_eq(&self, x: *const Emplaceable) -> () {
        return ((self.int_) == ((*x).int_)) && ((self.double_) == ((*x).double_));
    }
    pub unsafe fn operator_lt(&self, x: *const Emplaceable) -> () {
        return ((self.int_) < ((*x).int_))
            || (((self.int_) == ((*x).int_)) && ((self.double_) < ((*x).double_)));
    }
    pub unsafe fn get(&self) -> () {
        return self.int_;
    }
}
impl std::cmp::Ord for Emplaceable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Emplaceable::operator_lt(self, other as *const Emplaceable) {
                std::cmp::Ordering::Less
            } else if Emplaceable::operator_lt(other, self as *const Emplaceable) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Emplaceable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Emplaceable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Emplaceable::operator_eq(self, other as *const Emplaceable) }
    }
}
impl std::cmp::Eq for Emplaceable {}
impl Default for Emplaceable {
    fn default() -> Self {
        unsafe { Emplaceable::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_5))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_6))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_7)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_4)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_5)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_6)) = 0_();
    }
}
pub static mut outstanding_bytes_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_8(
    mut x: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<Emplaceable>()) as ()
        as ());
}
pub unsafe fn operator_eq_9(
    mut x: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_10(
    mut x: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_9(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut Emplaceable,
}
impl min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut Emplaceable) -> Self {
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
    pub unsafe fn operator_deref(&self) -> *mut Emplaceable {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut Emplaceable {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self
            as *mut min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_dec_10(
        &mut self,
    ) -> *mut min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_dec();
        return &mut (*(self
            as *mut min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__::operator_sub_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_9(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_11(
    mut x: min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<Emplaceable>()) as ()
        as ());
}
pub unsafe fn operator_eq_12(
    mut x: min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__ {
    ptr_: *const Emplaceable,
}
impl min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(
        mut p: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
    ) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const Emplaceable {
        return &(*self.ptr_);
    }
}
impl std::cmp::PartialEq
    for min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_12(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_13(
    mut x : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>(
        )) as () as ());
}
pub unsafe fn operator_eq_14(
    mut x : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_15(
    mut x : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  = x ;
        operator_eq_14(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { unsafe fn new_1 ( mut p : *mut min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : p , } ;
 this } pub unsafe fn new_2 ( mut _a0 : () , ) -> Self { let mut this = Self { ptr_ : std::ptr::null_mut() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *mut min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ { return & mut ( * self . ptr_   )  ;
 } pub unsafe fn operator_arrow ( &self , ) -> *mut min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ { return self . ptr_   ;
 } pub unsafe fn operator_inc_8 ( &mut self , ) -> *mut min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_10 ( &mut self , ) -> *mut min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : () , ) -> *mut min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : () , ) -> *mut min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add ( &self , mut n : () , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_add_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } pub unsafe fn operator_sub ( &self , mut n : () , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { let mut tmp : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ = ( * ( self as *const min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  ) ;
  ;
 ;
 ( unsafe { min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: operator_sub_assign ( &mut tmp  , n , ) } )  ;
 return tmp  ;
 } }
impl std::cmp::PartialEq for min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_14(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_16(
    mut x : min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize)
        / ::std::mem::size_of::<min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__>(
        )) as () as ());
}
pub unsafe fn operator_eq_17(
    mut x : min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_18(
    mut x : min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
    mut y : min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *const min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { pub unsafe fn new_1 ( mut p : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ , ) -> Self { let mut this = Self { ptr_ : ( p . ptr_  ) .cast_const() , } ;
 this } pub unsafe fn to_ ( &self , ) -> () { return ! ( ( self . ptr_  ).is_null() )  ;
 } pub unsafe fn operator_deref ( &self , ) -> *const min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ { return & ( * self . ptr_   )  ;
 } pub unsafe fn operator_inc_9 ( &mut self , ) -> *mut min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_inc() ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_dec_11 ( &mut self , ) -> *mut min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  .prefix_dec() ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_add_assign ( &mut self , mut n : () , ) -> *mut min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_add ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } pub unsafe fn operator_sub_assign ( &mut self , mut n : () , ) -> *mut min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { self . ptr_  = ( self . ptr_  ) . wrapping_sub ( n as usize ) ;
 return & mut ( * ( self as *mut min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__  )  )  ;
 } }
impl std::cmp::PartialEq for min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_17(self.clone(), other.clone()) }}}
impl std::cmp::Eq for min_pointer_const_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *mut min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__
{
    ptr_: *const min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0__,
}
impl min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_Emplaceable_ {}
impl min_allocator_Emplaceable_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__ {
        return min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_Emplaceable__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ {}
impl min_allocator_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0___ {
    pub unsafe fn allocate ( &mut self , mut n : () , ) -> min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__{
        return min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ :: new_1 ( { unimplemented!()  } , )   ;
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p : min_pointer_min_pointer_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0___ {
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_Emplaceable_ {}
impl safe_allocator_Emplaceable_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut Emplaceable {
        let mut memory: *mut Emplaceable = unimplemented!();
        {
            (unsafe {
                memset_19(
                    (memory as ()),
                    0_(),
                    (::std::mem::size_of::<Emplaceable>() as ()).wrapping_mul(n),
                )
            });
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut Emplaceable, mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    memset_19(
                        (p as ()),
                        0_(),
                        (::std::mem::size_of::<Emplaceable>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl Default for safe_allocator_Emplaceable_ {
    fn default() -> Self {
        unsafe { safe_allocator_Emplaceable_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_Emplaceable_ptr_ {}
impl safe_allocator_Emplaceable_ptr_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut *mut Emplaceable {
        let mut memory: *mut *mut Emplaceable = unimplemented!();
        {
            (unsafe {
                memset_19(
                    (memory as ()),
                    0_(),
                    (::std::mem::size_of::<*mut Emplaceable>() as ()).wrapping_mul(n),
                )
            });
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut Emplaceable, mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    memset_19(
                        (p as ()),
                        0_(),
                        (::std::mem::size_of::<*mut Emplaceable>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl Default for safe_allocator_Emplaceable_ptr_ {
    fn default() -> Self {
        unsafe { safe_allocator_Emplaceable_ptr_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct safe_allocator_const_Emplaceable_ptr_ {}
impl safe_allocator_const_Emplaceable_ptr_ {}
pub unsafe fn make_20(mut size: (), mut start: ()) -> () {
    let b: () = (4096_()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut init: () = 0_();
    if ((start) > (0_())) {
        init = ((((start) + (1_())) / (b)) + ((((start) + (1_())) % (b)) != (0_())));
        init *= b;
        init.prefix_dec();
    }
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < ((init) - (start))) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (size)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (start)) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub unsafe fn make_21(mut size: (), mut start: ()) -> () {
    let b: () = (4096_()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut init: () = 0_();
    if ((start) > (0_())) {
        init = ((((start) + (1_())) / (b)) + ((((start) + (1_())) % (b)) != (0_())));
        init *= b;
        init.prefix_dec();
    }
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < ((init) - (start))) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (size)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (start)) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub unsafe fn make_22(mut size: (), mut start: ()) -> () {
    let b: () = (4096_()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut init: () = 0_();
    if ((start) > (0_())) {
        init = ((((start) + (1_())) / (b)) + ((((start) + (1_())) % (b)) != (0_())));
        init *= b;
        init.prefix_dec();
    }
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < ((init) - (start))) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (size)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (start)) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub unsafe fn test_23(mut P: (), c1: *mut ()) {
    let mut c1_osize: () = unimplemented!();
    let mut i: () = {
        let mut __tmp_0: () = unimplemented!();
        std___deque_iterator_Emplaceable__const_Emplaceable_ptr__const_Emplaceable_ref__const_Emplaceable_ptrconst_ptr__long__256_ :: new_1 ( { & mut __tmp_0 } , )
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_24(mut P: (), c1: *mut ()) {
    let mut c1_osize: () = unimplemented!();
    let mut i: () = {
        let mut __tmp_1: () = unimplemented!();
        std___deque_iterator_Emplaceable__min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____const_Emplaceable_ref__min_pointer_const_min_pointer_const_Emplaceable__std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0____long__256_ :: new_1 ( { & mut __tmp_1 } , )
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_25(mut P: (), c1: *mut ()) {
    let mut c1_osize: () = unimplemented!();
    let mut i: () = {
        let mut __tmp_2: () = unimplemented!();
        std___deque_iterator_Emplaceable__const_Emplaceable_ptr__const_Emplaceable_ref__const_Emplaceable_ptrconst_ptr__long__256_ :: new_1 ( { & mut __tmp_2 } , )
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testN_26(mut start: (), mut N: ()) {
    let mut i: () = 0_();
    'loop_: while ((i) <= (3_())) {
        if ((0_()) <= (i)) && ((i) <= (N)) {
            let mut c1: () = (unsafe { make_20(N, start) });
            (unsafe { test_23(i, &mut c1) });
        }
        i.prefix_inc();
    }
    let mut i: () = (((N) / (2_())) - (1_()));
    'loop_: while ((i) <= (((N) / (2_())) + (1_()))) {
        if ((0_()) <= (i)) && ((i) <= (N)) {
            let mut c1: () = (unsafe { make_20(N, start) });
            (unsafe { test_23(i, &mut c1) });
        }
        i.prefix_inc();
    }
    let mut i: () = ((N) - (3_()));
    'loop_: while ((i) <= (N)) {
        if ((0_()) <= (i)) && ((i) <= (N)) {
            let mut c1: () = (unsafe { make_20(N, start) });
            (unsafe { test_23(i, &mut c1) });
        }
        i.prefix_inc();
    }
}
pub unsafe fn testN_27(mut start: (), mut N: ()) {
    let mut i: () = 0_();
    'loop_: while ((i) <= (3_())) {
        if ((0_()) <= (i)) && ((i) <= (N)) {
            let mut c1: () = (unsafe { make_21(N, start) });
            (unsafe { test_24(i, &mut c1) });
        }
        i.prefix_inc();
    }
    let mut i: () = (((N) / (2_())) - (1_()));
    'loop_: while ((i) <= (((N) / (2_())) + (1_()))) {
        if ((0_()) <= (i)) && ((i) <= (N)) {
            let mut c1: () = (unsafe { make_21(N, start) });
            (unsafe { test_24(i, &mut c1) });
        }
        i.prefix_inc();
    }
    let mut i: () = ((N) - (3_()));
    'loop_: while ((i) <= (N)) {
        if ((0_()) <= (i)) && ((i) <= (N)) {
            let mut c1: () = (unsafe { make_21(N, start) });
            (unsafe { test_24(i, &mut c1) });
        }
        i.prefix_inc();
    }
}
pub unsafe fn testN_28(mut start: (), mut N: ()) {
    let mut i: () = 0_();
    'loop_: while ((i) <= (3_())) {
        if ((0_()) <= (i)) && ((i) <= (N)) {
            let mut c1: () = (unsafe { make_22(N, start) });
            (unsafe { test_25(i, &mut c1) });
        }
        i.prefix_inc();
    }
    let mut i: () = (((N) / (2_())) - (1_()));
    'loop_: while ((i) <= (((N) / (2_())) + (1_()))) {
        if ((0_()) <= (i)) && ((i) <= (N)) {
            let mut c1: () = (unsafe { make_22(N, start) });
            (unsafe { test_25(i, &mut c1) });
        }
        i.prefix_inc();
    }
    let mut i: () = ((N) - (3_()));
    'loop_: while ((i) <= (N)) {
        if ((0_()) <= (i)) && ((i) <= (N)) {
            let mut c1: () = (unsafe { make_22(N, start) });
            (unsafe { test_25(i, &mut c1) });
        }
        i.prefix_inc();
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
    {
        let mut rng: [(); 10] = [
            0_(),
            1_(),
            2_(),
            3_(),
            1023_(),
            1024_(),
            1025_(),
            2047_(),
            2048_(),
            2049_(),
        ];
        let N: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut i: () = 0_();
        'loop_: while ((i) < (N)) {
            let mut j: () = 0_();
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: () = rng[(i) as usize];
                    let _N: () = rng[(j) as usize];
                    testN_26(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [(); 10] = [
            0_(),
            1_(),
            2_(),
            3_(),
            1023_(),
            1024_(),
            1025_(),
            2047_(),
            2048_(),
            2049_(),
        ];
        let N: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut i: () = 0_();
        'loop_: while ((i) < (N)) {
            let mut j: () = 0_();
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: () = rng[(i) as usize];
                    let _N: () = rng[(j) as usize];
                    testN_27(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    {
        let mut rng: [(); 10] = [
            0_(),
            1_(),
            2_(),
            3_(),
            1023_(),
            1024_(),
            1025_(),
            2047_(),
            2048_(),
            2049_(),
        ];
        let N: () = (::std::mem::size_of::<[(); 10]>() as ())
            .wrapping_div((::std::mem::size_of::<()>() as ()));
        let mut i: () = 0_();
        'loop_: while ((i) < (N)) {
            let mut j: () = 0_();
            'loop_: while ((j) < (N)) {
                (unsafe {
                    let _start: () = rng[(i) as usize];
                    let _N: () = rng[(j) as usize];
                    testN_28(_start, _N)
                });
                j.prefix_inc();
            }
            i.prefix_inc();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_4);
    std::cell::LazyCell::force(&*&raw const alloc_count_5);
    std::cell::LazyCell::force(&*&raw const dealloc_count_6);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_7);
}
