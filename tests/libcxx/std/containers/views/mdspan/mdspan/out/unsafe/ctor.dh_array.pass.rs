extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntType {
    pub val: (),
}
impl IntType {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn operator_eq(&self, rhs: *const IntType) -> () {
        return ((self.val) == ((*rhs).val));
    }
    pub unsafe fn to__4(&self) -> () {
        return self.val;
    }
    pub unsafe fn to__5(&self) -> () {
        return self.val;
    }
    pub unsafe fn to__6(&self) -> () {
        return self.val;
    }
}
impl std::cmp::PartialEq for IntType {
    fn eq(&self, other: &Self) -> bool {
        unsafe { IntType::operator_eq(self, other as *const IntType) }
    }
}
impl std::cmp::Eq for IntType {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct IntTypeNC {
    pub val: (),
}
impl IntTypeNC {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
    pub unsafe fn operator_eq(&self, rhs: *const IntType) -> () {
        return ((self.val) == ((*rhs).val));
    }
    pub unsafe fn to__4(&mut self) -> () {
        return self.val;
    }
    pub unsafe fn to__5(&mut self) -> () {
        return self.val;
    }
    pub unsafe fn to__6(&mut self) -> () {
        return self.val;
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct MinimalElementType {
    pub val: (),
}
impl MinimalElementType {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct ElementPool_int__1024_ {
    ptr_: *mut (),
}
impl ElementPool_int__1024_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this.ptr_ = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn get_ptr(&mut self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn destructor(&mut self) {
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
impl Default for ElementPool_int__1024_ {
    fn default() -> Self {
        unsafe { ElementPool_int__1024_::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct ElementPool_const_int__1024_ {
    ptr_: *mut (),
}
impl ElementPool_const_int__1024_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this.ptr_ = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn get_ptr(&mut self) -> *const () {
        return (self.ptr_).cast_const();
    }
    pub unsafe fn destructor(&mut self) {
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
impl Default for ElementPool_const_int__1024_ {
    fn default() -> Self {
        unsafe { ElementPool_const_int__1024_::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct ElementPool_double__1024_ {
    ptr_: *mut (),
}
impl ElementPool_double__1024_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this.ptr_ = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn get_ptr(&mut self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn destructor(&mut self) {
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
impl Default for ElementPool_double__1024_ {
    fn default() -> Self {
        unsafe { ElementPool_double__1024_::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct ElementPool_const_double__1024_ {
    ptr_: *mut (),
}
impl ElementPool_const_double__1024_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this.ptr_ = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn get_ptr(&mut self) -> *const () {
        return (self.ptr_).cast_const();
    }
    pub unsafe fn destructor(&mut self) {
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
impl Default for ElementPool_const_double__1024_ {
    fn default() -> Self {
        unsafe { ElementPool_const_double__1024_::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct ElementPool_MinimalElementType__1024_ {
    ptr_: *mut MinimalElementType,
}
impl ElementPool_MinimalElementType__1024_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this.ptr_ = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn get_ptr(&mut self) -> *mut MinimalElementType {
        return self.ptr_;
    }
    pub unsafe fn destructor(&mut self) {
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
impl Default for ElementPool_MinimalElementType__1024_ {
    fn default() -> Self {
        unsafe { ElementPool_MinimalElementType__1024_::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct ElementPool_const_MinimalElementType__1024_ {
    ptr_: *mut MinimalElementType,
}
impl ElementPool_const_MinimalElementType__1024_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this.ptr_ = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn get_ptr(&mut self) -> *const MinimalElementType {
        return (self.ptr_).cast_const();
    }
    pub unsafe fn destructor(&mut self) {
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
impl Default for ElementPool_const_MinimalElementType__1024_ {
    fn default() -> Self {
        unsafe { ElementPool_const_MinimalElementType__1024_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct not_extents_constructible_tag {}
static mut Wrap_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 8_() });
pub unsafe fn operator_eq_1(
    lhs: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    rhs: *const layout_wrapping_integral_8__mapping_std_extents_int__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_8__mapping_std_extents_int__ {
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_int__ {
    pub unsafe fn new_1(ext: *mut ()) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(ext: *const (), mut _a1: not_extents_constructible_tag) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn extents(&self) -> *const () {
        return &self.extents_;
    }
}
impl std::cmp::PartialEq for layout_wrapping_integral_8__mapping_std_extents_int__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_1(
                self as *const layout_wrapping_integral_8__mapping_std_extents_int__,
                other as *const layout_wrapping_integral_8__mapping_std_extents_int__,
            )
        }
    }
}
impl std::cmp::Eq for layout_wrapping_integral_8__mapping_std_extents_int__ {}
impl Default for layout_wrapping_integral_8__mapping_std_extents_int__ {
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_8__mapping_std_extents_int__ {
                extents_: unimplemented!(),
            }
        }
    }
}
static mut Wrap_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 8_() });
pub unsafe fn operator_eq_3(
    lhs: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    rhs: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ {
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ {
    pub unsafe fn new_1(ext: *mut ()) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(ext: *const (), mut _a1: not_extents_constructible_tag) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn extents(&self) -> *const () {
        return &self.extents_;
    }
}
impl std::cmp::PartialEq
    for layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_3(self as *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__, other as *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__)
        }
    }
}
impl std::cmp::Eq
    for layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__
{
}
impl Default
    for layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__
{
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
static mut Wrap_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 8_() });
pub unsafe fn operator_eq_5(
    lhs: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    rhs: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    pub unsafe fn new_1(ext: *mut ()) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_2(ext: *const (), mut _a1: not_extents_constructible_tag) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn extents(&self) -> *const () {
        return &self.extents_;
    }
}
impl std::cmp::PartialEq for layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
                self as *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
                other as *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
            )
        }
    }
}
impl std::cmp::Eq for layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {}
impl Default for layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
                extents_: unimplemented!(),
            }
        }
    }
}
static mut Wrap_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { (8_() as ()) });
pub unsafe fn operator_eq_7(
    lhs : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    rhs : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { pub unsafe fn new_1 ( ext : *mut () , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn new_2 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } }
impl std::cmp::PartialEq for layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_7(self as *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__, other as *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__) }}}
impl std::cmp::Eq for layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ {}
impl Default for layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_ : unimplemented!() , } } } }
static mut Wrap_8: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 8_() });
pub unsafe fn operator_eq_9(
    lhs : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    rhs : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { pub unsafe fn new_1 ( ext : *mut () , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn new_2 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } }
impl std::cmp::PartialEq for layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_9(self as *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__, other as *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__) }}}
impl std::cmp::Eq for layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ {}
impl Default for layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_ : unimplemented!() , } } } }
static mut Wrap_10: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 8_() });
pub unsafe fn operator_eq_11(
    lhs : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    rhs : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { pub unsafe fn new_1 ( ext : *mut () , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn new_2 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } }
impl std::cmp::PartialEq for layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_11(self as *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__, other as *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__) }}}
impl std::cmp::Eq for layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ {}
impl Default for layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct layout_wrapping_integral_8_ {}
static mut Wrap_12: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_int__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_int__ {
    pub unsafe fn new_1(ext: *const (), mut _a1: not_extents_constructible_tag) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_int__ {
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_int__ {
                extents_: unimplemented!(),
            }
        }
    }
}
static mut Wrap_13: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    pub unsafe fn new_1(ext: *const (), mut _a1: not_extents_constructible_tag) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
}
impl Default
    for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__
{
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
static mut Wrap_14: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    pub unsafe fn new_1(ext: *const (), mut _a1: not_extents_constructible_tag) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    fn default() -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
                extents_: unimplemented!(),
            }
        }
    }
}
static mut Wrap_15: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { (4_() as ()) });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_ : unimplemented!() , } } } }
static mut Wrap_16: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_ : unimplemented!() , } } } }
static mut Wrap_17: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct layout_wrapping_integral_4_ {}
pub unsafe fn construct_mapping_18(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_19(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_20(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_21(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_22(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_23(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_24(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_25(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_26(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_27(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_28(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_29(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_30(
    mut _a0: layout_wrapping_integral_8_,
    mut exts: (),
) -> layout_wrapping_integral_8__mapping_std_extents_int__ {
    return layout_wrapping_integral_8__mapping_std_extents_int__::new_2({ &exts }, {
        <not_extents_constructible_tag>::default()
    });
}
pub unsafe fn construct_mapping_31(
    mut _a0: layout_wrapping_integral_8_,
    mut exts: (),
) -> layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ {
    return layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ :: new_2 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}
pub unsafe fn construct_mapping_32(
    mut _a0: layout_wrapping_integral_8_,
    mut exts: (),
) -> layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    return layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__::new_2({ &exts }, {
        <not_extents_constructible_tag>::default()
    });
}pub unsafe fn construct_mapping_33 ( mut _a0 : layout_wrapping_integral_8_ , mut exts : () , ) -> layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    return layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_2 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}pub unsafe fn construct_mapping_34 ( mut _a0 : layout_wrapping_integral_8_ , mut exts : () , ) -> layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    return layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_2 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}pub unsafe fn construct_mapping_35 ( mut _a0 : layout_wrapping_integral_8_ , mut exts : () , ) -> layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
    return layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: new_2 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}
pub unsafe fn construct_mapping_36(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__ {
    return layout_wrapping_integral_4__mapping_std_extents_int__::new_1({ &exts }, {
        <not_extents_constructible_tag>::default()
    });
}
pub unsafe fn construct_mapping_37(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}
pub unsafe fn construct_mapping_38(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::new_1({ &exts }, {
        <not_extents_constructible_tag>::default()
    });
}pub unsafe fn construct_mapping_39 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}pub unsafe fn construct_mapping_40 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}pub unsafe fn construct_mapping_41 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
    return layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct always_convertible_layout {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct no_default_ctor_handle_double_ {
    pub ptr: *mut (),
}
impl no_default_ctor_handle_double_ {
    pub unsafe fn new_1(mut ptr_: *mut ()) -> Self {
        let mut this = Self { ptr: ptr_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct not_const_convertible_handle_unsigned_int_ {
    pub ptr: *mut (),
}
impl not_const_convertible_handle_unsigned_int_ {
    pub unsafe fn operator_index(&self, mut i: ()) -> *mut () {
        return &mut (*self.ptr.offset((i) as isize));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct not_const_convertible_handle_const_unsigned_int_ {
    pub ptr: *const (),
}
impl not_const_convertible_handle_const_unsigned_int_ {
    pub unsafe fn operator_index(&self, mut i: ()) -> *const () {
        return &(*self.ptr.offset((i) as isize));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct move_counted_handle_const_int_ {
    pub ptr: *const (),
}
impl move_counted_handle_const_int_ {
    pub unsafe fn new_1(mut ptr_: *const ()) -> Self {
        let mut this = Self { ptr: ptr_ };
        this
    }
    pub unsafe fn move_counter() -> *mut () {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_42));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct move_counted_handle_int_ {
    pub ptr: *mut (),
}
impl move_counted_handle_int_ {
    pub unsafe fn new_1(mut ptr_: *mut ()) -> Self {
        let mut this = Self { ptr: ptr_ };
        this
    }
    pub unsafe fn move_counter() -> *mut () {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_43));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct move_counted_handle_const_float_ {
    pub ptr: *const (),
}
impl move_counted_handle_const_float_ {
    pub unsafe fn new_1(mut ptr_: *const ()) -> Self {
        let mut this = Self { ptr: ptr_ };
        this
    }
    pub unsafe fn operator_index(&self, mut i: ()) -> *const () {
        return &(*self.ptr.offset((i) as isize));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct move_counted_handle_const_double_ {
    pub ptr: *const (),
}
impl move_counted_handle_const_double_ {
    pub unsafe fn move_from(other: *mut move_counted_handle_const_double_) -> Self {
        let mut this = Self {
            ptr: std::ptr::null(),
        };
        this.ptr = (*other).ptr;
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })).postfix_inc();
        }
        this
    }
    pub unsafe fn new_1(mut ptr_: *const ()) -> Self {
        let mut this = Self { ptr: ptr_ };
        this
    }
    pub unsafe fn operator_index(&self, mut i: ()) -> *const () {
        return &(*self.ptr.offset((i) as isize));
    }
    pub unsafe fn move_counter() -> *mut () {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_44));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct move_counted_handle_double_ {
    pub ptr: *mut (),
}
impl move_counted_handle_double_ {
    pub unsafe fn move_counter() -> *mut () {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_45));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct move_counted_handle_MinimalElementType_ {
    pub ptr: *mut MinimalElementType,
}
impl move_counted_handle_MinimalElementType_ {
    pub unsafe fn new_1(mut ptr_: *mut MinimalElementType) -> Self {
        let mut this = Self { ptr: ptr_ };
        this
    }
    pub unsafe fn move_counter() -> *mut () {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_46));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct move_counted_handle_const_MinimalElementType_ {
    pub ptr: *const MinimalElementType,
}
impl move_counted_handle_const_MinimalElementType_ {
    pub unsafe fn new_1(mut ptr_: *const MinimalElementType) -> Self {
        let mut this = Self { ptr: ptr_ };
        this
    }
    pub unsafe fn move_counter() -> *mut () {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_47));
    }
}
static mut c_42: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut c_43: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut c_44: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut c_45: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut c_46: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut c_47: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct checked_accessor_const_int_ {
    pub N: (),
}
impl checked_accessor_const_int_ {
    pub unsafe fn new(mut N_: ()) -> Self {
        let mut this = Self { N: N_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct checked_accessor_int_ {
    pub N: (),
}
impl checked_accessor_int_ {
    pub unsafe fn new(mut N_: ()) -> Self {
        let mut this = Self { N: N_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct checked_accessor_double_ {
    pub N: (),
}
impl checked_accessor_double_ {
    pub unsafe fn new(mut N_: ()) -> Self {
        let mut this = Self { N: N_ };
        this
    }
    pub unsafe fn access(&self, mut p: no_default_ctor_handle_double_, mut i: ()) -> *mut () {
        unimplemented!();
        return &mut (*p.ptr.offset((i) as isize));
    }
    pub unsafe fn offset(
        &self,
        mut p: no_default_ctor_handle_double_,
        mut i: (),
    ) -> no_default_ctor_handle_double_ {
        unimplemented!();
        return no_default_ctor_handle_double_::new_1({ p.ptr.offset((i) as isize) });
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct checked_accessor_unsigned_int_ {
    pub N: (),
}
impl checked_accessor_unsigned_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { N: 0_() };
        this
    }
    pub unsafe fn new_2(mut N_: ()) -> Self {
        let mut this = Self { N: N_ };
        this
    }
    pub unsafe fn copy_from(acc: *const checked_accessor_unsigned_int_) -> Self {
        let mut this = Self { N: (*acc).N };
        this
    }
    pub unsafe fn access(
        &self,
        mut p: not_const_convertible_handle_unsigned_int_,
        mut i: (),
    ) -> () {
        unimplemented!();
        return (*(unsafe { not_const_convertible_handle_unsigned_int_::operator_index(&p, i) }));
    }
    pub unsafe fn offset(
        &self,
        mut p: not_const_convertible_handle_unsigned_int_,
        mut i: (),
    ) -> *mut () {
        unimplemented!();
        return p.ptr.offset((i) as isize);
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const checked_accessor_unsigned_int_,
    ) -> *mut checked_accessor_unsigned_int_ {
        self.N = (*_a0).N;
        return &mut (*(self as *mut checked_accessor_unsigned_int_));
    }
}
impl Clone for checked_accessor_unsigned_int_ {
    fn clone(&self) -> Self {
        unsafe {
            checked_accessor_unsigned_int_::copy_from(self as *const checked_accessor_unsigned_int_)
        }
    }
}
impl Default for checked_accessor_unsigned_int_ {
    fn default() -> Self {
        unsafe { checked_accessor_unsigned_int_::new_1() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct checked_accessor_const_unsigned_int_ {
    pub N: (),
}
impl checked_accessor_const_unsigned_int_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { N: 0_() };
        this
    }
    pub unsafe fn new_2(mut N_: ()) -> Self {
        let mut this = Self { N: N_ };
        this
    }
    pub unsafe fn copy_from(acc: *const checked_accessor_const_unsigned_int_) -> Self {
        let mut this = Self { N: (*acc).N };
        this
    }
    pub unsafe fn access(
        &self,
        mut p: not_const_convertible_handle_const_unsigned_int_,
        mut i: (),
    ) -> () {
        unimplemented!();
        return (*(unsafe {
            not_const_convertible_handle_const_unsigned_int_::operator_index(&p, i)
        }));
    }
    pub unsafe fn offset(
        &self,
        mut p: not_const_convertible_handle_const_unsigned_int_,
        mut i: (),
    ) -> *const () {
        unimplemented!();
        return p.ptr.offset((i) as isize);
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const checked_accessor_const_unsigned_int_,
    ) -> *mut checked_accessor_const_unsigned_int_ {
        self.N = (*_a0).N;
        return &mut (*(self as *mut checked_accessor_const_unsigned_int_));
    }
}
impl Clone for checked_accessor_const_unsigned_int_ {
    fn clone(&self) -> Self {
        unsafe {
            checked_accessor_const_unsigned_int_::copy_from(
                self as *const checked_accessor_const_unsigned_int_,
            )
        }
    }
}
impl Default for checked_accessor_const_unsigned_int_ {
    fn default() -> Self {
        unsafe { checked_accessor_const_unsigned_int_::new_1() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct checked_accessor_const_float_ {
    pub N: (),
}
impl checked_accessor_const_float_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { N: 0_() };
        this
    }
    pub unsafe fn new_2(mut N_: ()) -> Self {
        let mut this = Self { N: N_ };
        this
    }
    pub unsafe fn copy_from(acc: *const checked_accessor_const_float_) -> Self {
        let mut this = Self { N: (*acc).N };
        this
    }
    pub unsafe fn new_3(acc: *mut checked_accessor_float_) -> Self {
        let mut this = Self { N: (*acc).N };
        this
    }
    pub unsafe fn access(&self, mut p: move_counted_handle_const_float_, mut i: ()) -> *const () {
        unimplemented!();
        return &(*(unsafe { move_counted_handle_const_float_::operator_index(&p, i) }));
    }
    pub unsafe fn offset(
        &self,
        mut p: move_counted_handle_const_float_,
        mut i: (),
    ) -> move_counted_handle_const_float_ {
        unimplemented!();
        return move_counted_handle_const_float_::new_1({ p.ptr.offset((i) as isize) });
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const checked_accessor_const_float_,
    ) -> *mut checked_accessor_const_float_ {
        self.N = (*_a0).N;
        return &mut (*(self as *mut checked_accessor_const_float_));
    }
}
impl Clone for checked_accessor_const_float_ {
    fn clone(&self) -> Self {
        unsafe {
            checked_accessor_const_float_::copy_from(self as *const checked_accessor_const_float_)
        }
    }
}
impl Default for checked_accessor_const_float_ {
    fn default() -> Self {
        unsafe { checked_accessor_const_float_::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct checked_accessor_float_ {
    pub N: (),
}
impl checked_accessor_float_ {}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct checked_accessor_const_double_ {
    pub N: (),
}
impl checked_accessor_const_double_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { N: 0_() };
        this
    }
    pub unsafe fn new_2(mut N_: ()) -> Self {
        let mut this = Self { N: N_ };
        this
    }
    pub unsafe fn copy_from(acc: *const checked_accessor_const_double_) -> Self {
        let mut this = Self { N: (*acc).N };
        this
    }
    pub unsafe fn access(&self, mut p: move_counted_handle_const_double_, mut i: ()) -> *const () {
        unimplemented!();
        return &(*(unsafe { move_counted_handle_const_double_::operator_index(&p, i) }));
    }
    pub unsafe fn offset(
        &self,
        mut p: move_counted_handle_const_double_,
        mut i: (),
    ) -> move_counted_handle_const_double_ {
        unimplemented!();
        return move_counted_handle_const_double_::new_1({ p.ptr.offset((i) as isize) });
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const checked_accessor_const_double_,
    ) -> *mut checked_accessor_const_double_ {
        self.N = (*_a0).N;
        return &mut (*(self as *mut checked_accessor_const_double_));
    }
}
impl Clone for checked_accessor_const_double_ {
    fn clone(&self) -> Self {
        unsafe {
            checked_accessor_const_double_::copy_from(self as *const checked_accessor_const_double_)
        }
    }
}
impl Default for checked_accessor_const_double_ {
    fn default() -> Self {
        unsafe { checked_accessor_const_double_::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct checked_accessor_MinimalElementType_ {
    pub N: (),
}
impl checked_accessor_MinimalElementType_ {
    pub unsafe fn new(mut N_: ()) -> Self {
        let mut this = Self { N: N_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct checked_accessor_const_MinimalElementType_ {
    pub N: (),
}
impl checked_accessor_const_MinimalElementType_ {
    pub unsafe fn new(mut N_: ()) -> Self {
        let mut this = Self { N: N_ };
        this
    }
}
const _: () = assert!(
    true,
    "std::is_constructible_v<checked_accessor<const int>, const checked_accessor<int>&>"
);
const _: () = assert!(
    true,
    "!std::is_convertible_v<const checked_accessor<int>&, checked_accessor<const int>>"
);
pub unsafe fn array_from_extents_48(exts: *const (), mut _a1: ()) -> () {
    return unimplemented!();
}
pub unsafe fn array_from_extents_49(exts: *const (), mut _a1: ()) -> () {
    return unimplemented!();
}
pub unsafe fn array_from_extents_50(exts: *const (), mut _a1: ()) -> () {
    return unimplemented!();
}
pub unsafe fn array_from_extents_51(exts: *const (), mut _a1: ()) -> () {
    return unimplemented!();
}
pub unsafe fn array_from_extents_52(exts: *const (), mut _a1: ()) -> () {
    return unimplemented!();
}
pub unsafe fn array_from_extents_53(exts: *const (), mut _a1: ()) -> () {
    return unimplemented!();
}
pub unsafe fn test_mdspan_ctor_array_54(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_55(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_56(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_57(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_58(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_59(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_60(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_61(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_62(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_63(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_64(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_65(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_66(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_67(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_68(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_69(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_70(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_71(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_72(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_73(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_74(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_75(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_76(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_77(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_78(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_79(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_80(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_81(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_82(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_83(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_84(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_85(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_86(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_87(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_88(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_89(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_90(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_91(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_92(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_93(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_94(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_95(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_96(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_97(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_98(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_99(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_100(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_101(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_102(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_103(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_104(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_105(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_106(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_107(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_108(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_109(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_110(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_111(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_112(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_113(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_114(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_115(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_116(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_117(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_118(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_119(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_120(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_121(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_122(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_123(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_124(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_125(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_126(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_127(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_128(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_129(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_130(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_131(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_132(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_133(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_134(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_135(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_136(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_137(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_138(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_139(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_140(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_141(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_142(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_143(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_144(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_145(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_146(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_147(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_148(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_149(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_150(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_151(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_152(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_153(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_154(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_155(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_156(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_157(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_158(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_159(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_160(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_161(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_162(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_163(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_164(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_165(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_166(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_167(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_168(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_169(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_170(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_171(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_172(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_173(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_174(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_175(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_176(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_177(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_178(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_179(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_180(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_181(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_182(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_183(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_184(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_185(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_186(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_187(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_188(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_189(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_190(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_191(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_192(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_193(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_194(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_195(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_196(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_197(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_198(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_199(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_200(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_201(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_202(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_203(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {
            unimplemented!();
        }
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_204(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_205(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_206(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_207(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_208(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_209(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_210(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_211(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_212(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_213(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_214(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_215(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_216(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_217(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_218(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_219(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_220(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_221(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_222(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_223(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_224(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_225(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_226(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_227(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_228(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_229(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_230(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_231(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_232(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_233(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_234(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_235(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_236(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_237(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_238(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_239(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_240(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_241(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_242(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_243(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_244(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_245(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_246(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_247(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_248(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_249(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_250(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_251(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_252(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_253(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_254(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_255(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_256(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_257(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_258(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_259(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_260(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_261(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_262(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_array_263(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut exts: (),
) {
    {
        (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
    }
    let mut m: () = unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "!noexcept(MDS(handle, exts))");;
    const _: () = assert!(
        true,
        "check_mdspan_ctor_implicit<MDS, decltype(exts)> == (N == MDS::rank_dynamic())"
    );;
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub unsafe fn test_mdspan_ctor_264(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_54(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_54(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_266(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_55(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_55(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_267(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_56(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_57(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_268(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_58(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_59(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_269(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_60(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_61(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_270(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_62(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_63(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_271(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_64(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_64(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_272(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_65(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_65(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_273(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_66(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_67(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_274(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_68(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_69(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_275(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_70(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_71(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_276(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_72(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_73(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_277(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_48(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_int__::extents(&(*map))
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_74(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_74(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_278(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_49(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_75(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_75(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_279(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_50(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__::extents(
                        &(*map),
                    )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_76(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_77(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_280(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_51(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_78(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_79(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_281(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_52(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_80(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_81(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_282(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_53(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_82(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_83(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_283(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_284(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_285(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_286(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_287(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_288(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_289(
    handle: *const move_counted_handle_int_,
    map: *const (),
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_290(
    handle: *const move_counted_handle_int_,
    map: *const (),
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_291(
    handle: *const move_counted_handle_int_,
    map: *const (),
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_292(
    handle: *const move_counted_handle_int_,
    map: *const (),
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_293(
    handle: *const move_counted_handle_int_,
    map: *const (),
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_294(
    handle: *const move_counted_handle_int_,
    map: *const (),
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_295(
    handle: *const move_counted_handle_int_,
    map: *const (),
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_296(
    handle: *const move_counted_handle_int_,
    map: *const (),
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_297(
    handle: *const move_counted_handle_int_,
    map: *const (),
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_298(
    handle: *const move_counted_handle_int_,
    map: *const (),
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_299(
    handle: *const move_counted_handle_int_,
    map: *const (),
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_300(
    handle: *const move_counted_handle_int_,
    map: *const (),
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_301(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_302(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_303(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_304(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_305(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_306(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_307(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_308(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_309(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_310(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_311(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_312(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_313(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_84(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_84(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_314(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_85(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_85(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_315(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_86(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_87(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_316(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_88(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_89(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_317(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_90(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_91(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_318(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_92(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_93(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_319(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_94(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_94(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_320(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_95(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_95(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_321(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_96(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_97(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_322(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_98(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_99(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_323(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_100(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_101(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_324(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_102(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_103(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_325(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_48(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_int__::extents(&(*map))
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_104(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_104(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_326(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_49(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_105(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_105(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_327(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_50(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__::extents(
                        &(*map),
                    )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_106(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_107(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_328(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_51(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_108(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_109(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_329(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_52(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_110(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_111(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_330(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_53(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_112(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_113(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_331(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_332(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_333(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_334(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_335(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_336(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_337(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_338(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_339(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_340(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_341(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_342(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_343(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_344(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_345(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_346(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_347(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_348(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_349(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_350(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_351(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_352(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_353(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_354(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_355(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_356(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_357(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_358(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_359(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_360(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_361(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_114(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_114(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_362(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_115(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_115(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_363(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_116(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_117(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_364(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_118(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_119(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_365(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_120(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_121(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_366(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_122(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_123(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_367(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_124(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_124(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_368(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_125(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_125(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_369(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_126(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_127(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_370(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_128(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_129(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_371(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_130(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_131(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_372(handle: *const *mut (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_132(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_133(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_373(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_48(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_int__::extents(&(*map))
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_134(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_134(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_374(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_49(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_135(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_135(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_375(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_50(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__::extents(
                        &(*map),
                    )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_136(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_137(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_376(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_51(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_138(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_139(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_377(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_52(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_140(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_141(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_378(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_53(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_142(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_143(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_379(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_380(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_381(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_382(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_383(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_384(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_385(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_386(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_387(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_388(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_389(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_390(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_391(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_392(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_393(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_394(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_395(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_396(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_397(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_398(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_399(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_400(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_401(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_402(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_403(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_404(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_405(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_406(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_407(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_408(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_409(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_144(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_144(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_410(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_145(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_145(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_411(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_146(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_147(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_412(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_148(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_149(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_413(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_150(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_151(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_414(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_152(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_153(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_415(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_154(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_154(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_416(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_155(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_155(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_417(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_156(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_157(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_418(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_158(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_159(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_419(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_160(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_161(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_420(handle: *const *const (), map: *const (), acc: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_162(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_163(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_421(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_48(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_int__::extents(&(*map))
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_164(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_164(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_422(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_49(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_165(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_165(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_423(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_50(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__::extents(
                        &(*map),
                    )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_166(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_167(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_424(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_51(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_168(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_169(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_425(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_52(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_170(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_171(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_426(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_53(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_172(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const () = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_173(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_427(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_428(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_429(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_430(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_431(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_432(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_433(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_174(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_174(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_434(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_175(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_175(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_435(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_176(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_177(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_436(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_178(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_179(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_437(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_180(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_181(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_438(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_182(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_183(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_439(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_184(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_184(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_440(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_185(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_185(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_441(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_186(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_187(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_442(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_188(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_189(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_443(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_190(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_191(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_444(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_192(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const () = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_193(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_445(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_48(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_int__::extents(&(*map))
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_194(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_194(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_446(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_49(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_195(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_195(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_447(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_50(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__::extents(
                        &(*map),
                    )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_196(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_197(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_448(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_51(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_198(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_199(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_449(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_52(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_200(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_201(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_450(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_53(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_202(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const move_counted_handle_const_double_ = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const checked_accessor_const_double_ = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_203(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_451(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_452(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_453(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_454(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_455(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_456(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_457(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_204(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_204(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_458(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_205(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_205(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_459(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_206(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_207(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_460(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_208(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_209(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_461(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_210(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_211(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_462(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_212(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_213(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_463(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_214(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_214(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_464(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_215(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_215(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_465(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_216(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_217(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_466(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_218(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_219(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_467(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_220(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_221(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_468(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_222(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_223(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_469(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_48(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_int__::extents(&(*map))
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_224(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_224(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_470(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_49(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_225(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_225(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_471(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_50(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__::extents(
                        &(*map),
                    )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_226(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_227(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_472(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_51(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_228(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_229(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_473(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_52(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_230(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_231(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_474(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_53(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_232(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *mut MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_233(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_475(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_476(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_477(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_478(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_479(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_480(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_481(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_482(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_483(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_484(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_485(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_486(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_487(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_488(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_489(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_490(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_491(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_492(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_493(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_494(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_495(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_496(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_497(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_498(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_499(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_500(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_501(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_502(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_503(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_504(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_505(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_234(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_234(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_506(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_235(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_235(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_507(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_236(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_237(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_508(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_238(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_239(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_509(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_240(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_241(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_510(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_242(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_243(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_511(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_48(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_244(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_244(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_512(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_49(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_245(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_245(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_513(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_50(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_246(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_247(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_514(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_51(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_248(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_249(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_515(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_52(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_250(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_251(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_516(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe { array_from_extents_53(unimplemented!(), unimplemented!()) });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_252(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const () = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_253(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_517(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_48(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_int__::extents(&(*map))
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_254(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_int__ = map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_254(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_518(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_49(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_255(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_255(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_519(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_50(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__::extents(
                        &(*map),
                    )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_256(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
                map;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_257(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_520(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_51(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_258(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_259(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_521(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_52(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_260(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_261(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_522(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut exts: () = (unsafe {
            array_from_extents_53(
                (unsafe {
                    layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: extents ( & ( * map )  , )
                }),
                unimplemented!(),
            )
        });
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts.clone();
            test_mdspan_ctor_array_262(_handle, _map, _arg2, _exts)
        });
        let mut exts_dynamic: () = unimplemented!();
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!())
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265)))
            {
                (*unimplemented!()) = (*unimplemented!());
            }
            r.postfix_inc();
        }
        (unsafe {
            let _handle: *const *const MinimalElementType = handle;
            let _map: *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = map ;
            let _arg2: *const () = acc;
            let _exts: () = exts_dynamic.clone();
            test_mdspan_ctor_array_263(_handle, _map, _arg2, _exts)
        });
    }
}
pub unsafe fn test_mdspan_ctor_523(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_524(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_525(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_526(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_527(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_528(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_529(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_530(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_531(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_532(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_533(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_534(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_535(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_536(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_537(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_538(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_539(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_540(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_541(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_542(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_543(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_544(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_545(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_546(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_547(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_548(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_549(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_550(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_551(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn test_mdspan_ctor_552(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const std::array<typename MDS::index_type, MDS::rank()>&>"
        );;
    }
}
pub unsafe fn mixin_extents_553(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_264(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_266(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_267(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_268(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_269(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_270(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_554(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_271(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_272(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_273(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_274(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_275(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_276(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_555(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_8_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_277(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_278(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_279(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_280(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_281(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_282(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_556(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_283(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_284(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_285(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_286(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_287(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_288(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_557(
    handle: *const move_counted_handle_int_,
    layout: *const (),
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_289(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_290(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_291(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_292(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_293(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_294(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_558(
    handle: *const move_counted_handle_int_,
    layout: *const (),
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_295(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_296(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_297(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_298(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_299(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_300(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_559(
    handle: *const move_counted_handle_int_,
    layout: *const layout_wrapping_integral_8_,
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_301(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_302(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_303(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_304(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_305(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_306(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_560(
    handle: *const move_counted_handle_int_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_307(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_308(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_309(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_310(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_311(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_int_ = acc;
        test_mdspan_ctor_312(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_561(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_313(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_314(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_315(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_316(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_317(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_318(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_562(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_319(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_320(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_321(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_322(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_323(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_324(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_563(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_8_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_325(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_326(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_327(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_328(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_329(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_330(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_564(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_331(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_332(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_333(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_334(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_335(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_336(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_565(
    handle: *const move_counted_handle_const_int_,
    layout: *const (),
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_337(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_338(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_339(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_340(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_341(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_342(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_566(
    handle: *const move_counted_handle_const_int_,
    layout: *const (),
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_343(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_344(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_345(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_346(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_347(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_348(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_567(
    handle: *const move_counted_handle_const_int_,
    layout: *const layout_wrapping_integral_8_,
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_349(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_350(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_351(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_352(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_353(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_354(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_568(
    handle: *const move_counted_handle_const_int_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_355(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_356(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_357(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_358(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_359(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_int_ = acc;
        test_mdspan_ctor_360(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_569(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_361(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_362(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_363(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_364(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_365(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_366(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_570(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_367(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_368(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_369(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_370(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_371(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_372(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_571(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_8_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_373(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_374(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_375(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_376(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_377(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_378(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_572(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_379(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_380(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_381(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_382(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_383(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_384(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_573(
    handle: *const no_default_ctor_handle_double_,
    layout: *const (),
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_385(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_386(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_387(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_388(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_389(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_390(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_574(
    handle: *const no_default_ctor_handle_double_,
    layout: *const (),
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_391(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_392(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_393(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_394(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_395(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_396(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_575(
    handle: *const no_default_ctor_handle_double_,
    layout: *const layout_wrapping_integral_8_,
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_397(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_398(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_399(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_400(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_401(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_402(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_576(
    handle: *const no_default_ctor_handle_double_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_403(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_404(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_405(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_406(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_407(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_double_ = acc;
        test_mdspan_ctor_408(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_577(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_409(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_410(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_411(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_412(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_413(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_414(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_578(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_415(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_416(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_417(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_418(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_419(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_420(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_579(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_8_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_421(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_422(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_423(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_424(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_425(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_426(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_580(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_427(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_428(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_429(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_430(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_431(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_432(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_581(
    handle: *const move_counted_handle_const_double_,
    layout: *const (),
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_433(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_434(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_435(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_436(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_437(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_438(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_582(
    handle: *const move_counted_handle_const_double_,
    layout: *const (),
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_439(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_440(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_441(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_442(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_443(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_444(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_583(
    handle: *const move_counted_handle_const_double_,
    layout: *const layout_wrapping_integral_8_,
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_445(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_446(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_447(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_448(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_449(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_450(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_584(
    handle: *const move_counted_handle_const_double_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_451(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_452(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_453(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_454(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_455(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_double_ = acc;
        test_mdspan_ctor_456(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_585(
    handle: *const *mut MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_457(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_458(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_459(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_460(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_461(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_462(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_586(
    handle: *const *mut MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_463(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_464(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_465(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_466(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_467(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_468(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_587(
    handle: *const *mut MinimalElementType,
    layout: *const layout_wrapping_integral_8_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_469(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_470(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_471(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_472(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_473(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_474(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_588(
    handle: *const *mut MinimalElementType,
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_475(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_476(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_477(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_478(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_479(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_480(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_589(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_481(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_482(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_483(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_484(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_485(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_486(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_590(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_487(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_488(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_489(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_490(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_491(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_492(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_591(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const layout_wrapping_integral_8_,
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_493(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_494(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_495(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_496(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_497(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_498(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_592(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_499(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_500(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_501(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_502(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_503(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_ctor_504(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_593(
    handle: *const *const MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_505(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_506(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_507(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_508(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_509(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_510(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_594(
    handle: *const *const MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_511(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_512(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_513(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_514(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_515(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_516(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_595(
    handle: *const *const MinimalElementType,
    layout: *const layout_wrapping_integral_8_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_517(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_518(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_519(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_520(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_521(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_522(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_596(
    handle: *const *const MinimalElementType,
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_523(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_524(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_ctor_525(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_526(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_527(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_ctor_528(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_597(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_529(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_530(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_531(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_532(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_533(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_534(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_598(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_535(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_536(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_537(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_538(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_539(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_540(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_599(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const layout_wrapping_integral_8_,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_541(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_542(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_543(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_544(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_545(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_546(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_600(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_547(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_548(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_549(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_550(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_551(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_ctor_552(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_layout_601(handle: *const *mut (), acc: *const ()) {
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_553(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_554(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const () = acc;
        mixin_extents_555(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_556(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_602(
    handle: *const move_counted_handle_int_,
    acc: *const checked_accessor_int_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_int_ = acc;
        mixin_extents_557(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_int_ = acc;
        mixin_extents_558(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const checked_accessor_int_ = acc;
        mixin_extents_559(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_int_ = acc;
        mixin_extents_560(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_603(handle: *const *const (), acc: *const ()) {
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_561(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_562(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const () = acc;
        mixin_extents_563(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_564(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_604(
    handle: *const move_counted_handle_const_int_,
    acc: *const checked_accessor_const_int_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_int_ = acc;
        mixin_extents_565(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_int_ = acc;
        mixin_extents_566(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const checked_accessor_const_int_ = acc;
        mixin_extents_567(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_const_int_ = acc;
        mixin_extents_568(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_605(handle: *const *mut (), acc: *const ()) {
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_569(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_570(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const () = acc;
        mixin_extents_571(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_572(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_606(
    handle: *const no_default_ctor_handle_double_,
    acc: *const checked_accessor_double_,
) {
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_double_ = acc;
        mixin_extents_573(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_double_ = acc;
        mixin_extents_574(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const checked_accessor_double_ = acc;
        mixin_extents_575(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_double_ = acc;
        mixin_extents_576(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_607(handle: *const *const (), acc: *const ()) {
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_577(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_578(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const () = acc;
        mixin_extents_579(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_580(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_608(
    handle: *const move_counted_handle_const_double_,
    acc: *const checked_accessor_const_double_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_double_ = acc;
        mixin_extents_581(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_double_ = acc;
        mixin_extents_582(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const checked_accessor_const_double_ = acc;
        mixin_extents_583(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_const_double_ = acc;
        mixin_extents_584(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_609(handle: *const *mut MinimalElementType, acc: *const ()) {
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_585(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_586(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const () = acc;
        mixin_extents_587(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_588(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_610(
    handle: *const move_counted_handle_MinimalElementType_,
    acc: *const checked_accessor_MinimalElementType_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        mixin_extents_589(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        mixin_extents_590(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        mixin_extents_591(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        mixin_extents_592(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_611(handle: *const *const MinimalElementType, acc: *const ()) {
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_593(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_594(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const () = acc;
        mixin_extents_595(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_596(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_612(
    handle: *const move_counted_handle_const_MinimalElementType_,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        mixin_extents_597(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        mixin_extents_598(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        mixin_extents_599(_handle, &mut _layout, _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        mixin_extents_600(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_accessor_613() {
    let mut elements: ElementPool_int__1024_ = ElementPool_int__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_int__1024_::destructor);
    (unsafe {
        let mut _handle: *mut () = (unsafe { ElementPool_int__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_601(&mut _handle, &mut _acc)
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    (unsafe {
        let mut _handle: move_counted_handle_int_ = move_counted_handle_int_::new_1({
            (unsafe { ElementPool_int__1024_::get_ptr(&mut elements) })
        });
        let mut _acc: checked_accessor_int_ = checked_accessor_int_::new({ 1024_() });
        mixin_layout_602(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_614() {
    let mut elements: ElementPool_const_int__1024_ = ElementPool_const_int__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_const_int__1024_::destructor);
    (unsafe {
        let mut _handle: *const () =
            (unsafe { ElementPool_const_int__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_603(&mut _handle, &mut _acc)
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    (unsafe {
        let mut _handle: move_counted_handle_const_int_ = move_counted_handle_const_int_::new_1({
            (unsafe { ElementPool_const_int__1024_::get_ptr(&mut elements) })
        });
        let mut _acc: checked_accessor_const_int_ = checked_accessor_const_int_::new({ 1024_() });
        mixin_layout_604(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_615() {
    let mut elements: ElementPool_double__1024_ = ElementPool_double__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_double__1024_::destructor);
    (unsafe {
        let mut _handle: *mut () = (unsafe { ElementPool_double__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_605(&mut _handle, &mut _acc)
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    (unsafe {
        let mut _handle: no_default_ctor_handle_double_ = no_default_ctor_handle_double_::new_1({
            (unsafe { ElementPool_double__1024_::get_ptr(&mut elements) })
        });
        let mut _acc: checked_accessor_double_ = checked_accessor_double_::new({ 1024_() });
        mixin_layout_606(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_616() {
    let mut elements: ElementPool_const_double__1024_ = ElementPool_const_double__1024_::new();
    let _dtor_elements = ScopedDestructorUnsafe::new(
        &raw mut elements,
        ElementPool_const_double__1024_::destructor,
    );
    (unsafe {
        let mut _handle: *const () =
            (unsafe { ElementPool_const_double__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_607(&mut _handle, &mut _acc)
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    (unsafe {
        let mut _handle: move_counted_handle_const_double_ =
            move_counted_handle_const_double_::new_1({
                (unsafe { ElementPool_const_double__1024_::get_ptr(&mut elements) })
            });
        let mut _acc: checked_accessor_const_double_ =
            checked_accessor_const_double_::new_2({ 1024_() });
        mixin_layout_608(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_617() {
    let mut elements: ElementPool_MinimalElementType__1024_ =
        ElementPool_MinimalElementType__1024_::new();
    let _dtor_elements = ScopedDestructorUnsafe::new(
        &raw mut elements,
        ElementPool_MinimalElementType__1024_::destructor,
    );
    (unsafe {
        let mut _handle: *mut MinimalElementType =
            (unsafe { ElementPool_MinimalElementType__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_609(&mut _handle, &mut _acc)
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    (unsafe {
        let mut _handle: move_counted_handle_MinimalElementType_ =
            move_counted_handle_MinimalElementType_::new_1({
                (unsafe { ElementPool_MinimalElementType__1024_::get_ptr(&mut elements) })
            });
        let mut _acc: checked_accessor_MinimalElementType_ =
            checked_accessor_MinimalElementType_::new({ 1024_() });
        mixin_layout_610(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_618() {
    let mut elements: ElementPool_const_MinimalElementType__1024_ =
        ElementPool_const_MinimalElementType__1024_::new();
    let _dtor_elements = ScopedDestructorUnsafe::new(
        &raw mut elements,
        ElementPool_const_MinimalElementType__1024_::destructor,
    );
    (unsafe {
        let mut _handle: *const MinimalElementType =
            (unsafe { ElementPool_const_MinimalElementType__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_611(&mut _handle, &mut _acc)
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    (unsafe {
        let mut _handle: move_counted_handle_const_MinimalElementType_ =
            move_counted_handle_const_MinimalElementType_::new_1({
                (unsafe { ElementPool_const_MinimalElementType__1024_::get_ptr(&mut elements) })
            });
        let mut _acc: checked_accessor_const_MinimalElementType_ =
            checked_accessor_const_MinimalElementType_::new({ 1024_() });
        mixin_layout_612(&mut _handle, &mut _acc)
    });
}
pub unsafe fn test_619() -> () {
    (unsafe { mixin_accessor_613() });
    (unsafe { mixin_accessor_614() });
    (unsafe { mixin_accessor_615() });
    (unsafe { mixin_accessor_616() });
    (unsafe { mixin_accessor_617() });
    (unsafe { mixin_accessor_618() });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_265));
    const _: () = assert!(
        true,
        "std::is_constructible_v<mds_t, float*, std::array<int, 3>>"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible_v<mds_t, float*, std::array<int, 2>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, std::array<int, 1>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, std::array<int, 4>>"
    );;
    const _: () = assert!(true, "std::is_convertible_v<const IntType&, int>");;
    const _: () = assert!(true, "!std::is_convertible_v<const IntType&, unsigned>");;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, std::array<IntType, 2>>"
    );;
    const _: () = assert!(true, "std::is_convertible_v<IntType, unsigned char>");;
    const _: () = assert!(true, "std::is_convertible_v<const IntType&, unsigned char>");;
    const _: () = assert!(
        true,
        "!std::is_nothrow_constructible_v<unsigned char, const IntType&>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_uchar_t, float*, std::array<IntType, 2>>"
    );;
    const _: () = assert!(true, "std::is_convertible_v<IntTypeNC, int>");;
    const _: () = assert!(true, "!std::is_convertible_v<const IntTypeNC&, int>");;
    const _: () = assert!(true, "std::is_nothrow_constructible_v<int, IntTypeNC>");;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_int_t, float*, std::array<IntTypeNC, 2>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, const float*, std::array<int, 2>>"
    );;
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
    (unsafe { test_619() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const Wrap_0);
    std::cell::LazyCell::force(&*&raw const Wrap_2);
    std::cell::LazyCell::force(&*&raw const Wrap_4);
    std::cell::LazyCell::force(&*&raw const Wrap_6);
    std::cell::LazyCell::force(&*&raw const Wrap_8);
    std::cell::LazyCell::force(&*&raw const Wrap_10);
    std::cell::LazyCell::force(&*&raw const Wrap_12);
    std::cell::LazyCell::force(&*&raw const Wrap_13);
    std::cell::LazyCell::force(&*&raw const Wrap_14);
    std::cell::LazyCell::force(&*&raw const Wrap_15);
    std::cell::LazyCell::force(&*&raw const Wrap_16);
    std::cell::LazyCell::force(&*&raw const Wrap_17);
}
