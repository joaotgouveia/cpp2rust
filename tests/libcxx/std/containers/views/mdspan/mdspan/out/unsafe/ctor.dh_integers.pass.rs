extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
        this.ptr_ = (unimplemented!() as *mut ()).cast::<()>();
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn get_ptr(&mut self) -> *mut () {
        return (self.ptr_ as *mut ()).cast::<()>();
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
        this.ptr_ = (unimplemented!() as *mut ()).cast::<()>();
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn get_ptr(&mut self) -> *const () {
        return ((self.ptr_).cast_const() as *const ()).cast::<()>();
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
        this.ptr_ = (unimplemented!() as *mut ()).cast::<()>();
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn get_ptr(&mut self) -> *mut () {
        return (self.ptr_ as *mut ()).cast::<()>();
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
        this.ptr_ = (unimplemented!() as *mut ()).cast::<()>();
        let mut i: () = 0_();
        'loop_: while ((i) != (1024_())) {
            unimplemented!();
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn get_ptr(&mut self) -> *const () {
        return ((self.ptr_).cast_const() as *const ()).cast::<()>();
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
pub unsafe fn test_mdspan_types_48(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_49(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_50(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_51(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_52(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_53(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_54(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_55(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_56(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_57(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_58(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_59(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_60(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_61(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_62(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_63(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_64(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_65(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_66(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_67(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_68(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_69(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_70(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_71(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_72(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_73(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_74(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_75(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_76(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_77(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_78(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_79(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_80(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_81(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_82(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_83(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_84(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_85(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_86(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_87(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_88(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_89(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_90(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_91(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_92(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_93(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_94(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_95(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_96(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_97(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_98(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_99(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_100(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_101(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_102(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_103(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_104(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_105(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_106(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_107(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_108(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_109(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_110(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_111(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_112(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_113(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_114(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_115(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_116(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_117(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_118(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_119(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_120(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_121(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_122(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_123(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_124(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_125(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_126(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_127(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_128(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_129(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_130(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_131(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_132(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_133(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_134(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_135(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_136(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_137(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_138(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_139(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_140(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_141(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_142(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_143(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_144(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_145(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_146(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_147(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_148(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_149(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_150(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_151(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_152(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_153(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_154(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_155(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_156(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_157(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_158(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_159(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_160(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_161(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_162(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_163(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_164(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_165(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_166(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_167(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_168(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_169(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_170(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_171(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_172(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_173(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_174(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_175(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_176(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_177(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_178(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_179(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_180(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_181(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_182(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_183(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_184(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_185(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_186(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_187(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_188(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_189(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_190(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_191(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_192(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_193(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_194(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_195(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_196(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_197(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_198(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_199(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_200(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_201(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_202(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_203(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_204(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_int_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_205(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_206(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_207(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_208(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_209(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_210(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_211(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_212(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_213(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_214(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_215(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_216(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_217(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_218(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_219(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_220(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_221(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_222(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_223(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_224(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_225(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_226(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_227(
    handle: *const *mut (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_228(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_229(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_230(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_231(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_232(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_233(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_234(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_235(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_236(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_237(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_238(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_239(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_240(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_241(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_242(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_243(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_244(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_245(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_246(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_247(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_248(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_249(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_250(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_251(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_252(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_253(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_254(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_255(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_256(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_257(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_258(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_259(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_260(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_261(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_262(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_263(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_264(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_265(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_266(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_267(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_268(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_269(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_270(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_271(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_272(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_273(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_274(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_275(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_276(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_277(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_278(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_279(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_280(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_281(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_282(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_283(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_284(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_285(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_286(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_287(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_288(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_289(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_290(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_291(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_292(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_293(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_294(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_295(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_296(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_297(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_298(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_299(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_300(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_301(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_302(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_303(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_304(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_305(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_306(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_307(
    handle: *const *const (),
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_308(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_309(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_310(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_311(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_312(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_313(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_314(
    handle: *const *const (),
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_315(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_316(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_317(
    handle: *const *const (),
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_318(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_319(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_320(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_321(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_322(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_323(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_324(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_325(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_326(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_327(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_328(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_329(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_330(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_331(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_332(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_333(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_334(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_335(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_336(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_337(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_338(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_339(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_340(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_341(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_342(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_343(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_344(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_345(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_346(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_347(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_348(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_349(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_350(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_351(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_352(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_353(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_354(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_355(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_356(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_357(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_358(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_359(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_360(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_361(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_362(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_363(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_364(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_double_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_365(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_366(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_367(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_368(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_369(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_370(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_371(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_372(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_373(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_374(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_375(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_376(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_377(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_378(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_379(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_380(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_381(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_382(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_383(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_384(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_385(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_386(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_387(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_388(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_389(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_390(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_391(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_392(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_393(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_394(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_395(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_396(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_397(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_398(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_399(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_400(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_401(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_402(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_403(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_404(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_405(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_406(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_407(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_408(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_409(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_410(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_411(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_412(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_413(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_414(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_415(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_416(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_417(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_418(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_419(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_420(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_421(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_422(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_423(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_424(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_425(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_426(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_427(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_428(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_429(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_430(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_431(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_432(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_433(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_434(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_435(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_436(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_437(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_438(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_439(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_440(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_441(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_442(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_443(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_444(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_445(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_446(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_447(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_448(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_449(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_450(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_451(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_452(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_453(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_454(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_455(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_456(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_457(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_458(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_459(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_460(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_461(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_462(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_463(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_464(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_465(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_466(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_467(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_468(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_469(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_470(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_471(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_472(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_473(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_474(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_475(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_476(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_477(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(
            true,
            "check_mdspan_ctor_implicit<MDS, H, std::array<typename MDS::index_type, MDS::rank_dynamic()>>"
        );;
        const _: () = assert!(!(false), "!check_mdspan_ctor_implicit<MDS, H, Idxs...>");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_478(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_479(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_480(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_481(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_482(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_483(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_484(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_485(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_486(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_487(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_488(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_489(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_490(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_491(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_492(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_493(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_494(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_495(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_496(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_497(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_498(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_499(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_500(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_501(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_502(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_503(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_504(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_505(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_506(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_507(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_508(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_509(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_510(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_511(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_512(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_513(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_514(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_515(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_516(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_517(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_518(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_519(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_520(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_521(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_522(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_523(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_524(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_525(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_526(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn test_mdspan_types_527(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
    mut idxs_3: (),
    mut idxs_4: (),
    mut idxs_5: (),
    mut idxs_6: (),
    mut idxs_7: (),
    mut idxs_8: (),
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, Idxs... >");;
    }
}
pub unsafe fn mixin_extents_528(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_48(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_49(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_50(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_51(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_52(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_53(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_54(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_55(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_56(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_57(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_530(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_58(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_59(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_60(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_61(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_62(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_63(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_64(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_65(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_66(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_67(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_531(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_8_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_68(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_69(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_70(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_71(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_72(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_73(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_74(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_75(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_76(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_77(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_532(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_78(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_79(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_80(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_81(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_82(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_83(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_84(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_85(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_86(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_87(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_533(
    handle: *const move_counted_handle_int_,
    layout: *const (),
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_88(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_89(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_90(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_91(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_92(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_93(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_94(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_95(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_96(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_97(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_534(
    handle: *const move_counted_handle_int_,
    layout: *const (),
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_98(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_99(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_100(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_101(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_102(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_103(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_104(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_105(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_106(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_107(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_535(
    handle: *const move_counted_handle_int_,
    layout: *const layout_wrapping_integral_8_,
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_108(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_109(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_110(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_111(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_112(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_113(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_114(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_115(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_116(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_117(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_536(
    handle: *const move_counted_handle_int_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_118(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_119(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_120(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_121(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_122(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_123(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_124(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_125(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_126(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_127(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_537(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_128(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_129(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_130(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_131(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_132(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_133(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_134(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_135(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_136(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_137(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_538(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_138(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_139(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_140(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_141(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_142(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_143(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_144(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_145(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_146(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_147(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_539(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_8_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_148(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_149(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_150(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_151(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_152(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_153(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_154(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_155(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_156(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_157(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_540(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_158(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_159(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_160(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_161(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_162(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_163(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_164(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_165(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_166(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_167(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_541(
    handle: *const move_counted_handle_const_int_,
    layout: *const (),
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_168(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_169(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_170(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_171(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_172(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_173(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_174(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_175(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_176(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_177(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_542(
    handle: *const move_counted_handle_const_int_,
    layout: *const (),
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_178(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_179(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_180(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_181(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_182(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_183(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_184(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_185(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_186(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_187(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_543(
    handle: *const move_counted_handle_const_int_,
    layout: *const layout_wrapping_integral_8_,
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_188(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_189(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_190(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_191(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_192(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_193(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_194(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_195(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_196(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_197(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_544(
    handle: *const move_counted_handle_const_int_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_198(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_199(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_200(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_201(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_202(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_203(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_204(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_205(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_206(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_207(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_545(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_208(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_209(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_210(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_211(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_212(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_213(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_214(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_215(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_216(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_217(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_546(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_218(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_219(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_220(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_221(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_222(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_223(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_224(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_225(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_226(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_227(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_547(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_8_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_228(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_229(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_230(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_231(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_232(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_233(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_234(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_235(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_236(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_237(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_548(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_238(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_239(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_240(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_241(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_242(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_243(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_244(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_245(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_246(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_247(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_549(
    handle: *const no_default_ctor_handle_double_,
    layout: *const (),
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_248(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_249(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_250(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_251(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_252(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_253(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_254(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_255(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_256(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_257(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_550(
    handle: *const no_default_ctor_handle_double_,
    layout: *const (),
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_258(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_259(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_260(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_261(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_262(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_263(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_264(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_265(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_266(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_267(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_551(
    handle: *const no_default_ctor_handle_double_,
    layout: *const layout_wrapping_integral_8_,
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_268(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_269(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_270(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_271(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_272(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_273(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_274(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_275(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_276(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_277(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_552(
    handle: *const no_default_ctor_handle_double_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_278(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_279(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_280(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_281(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_282(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_283(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_284(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_285(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_286(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_287(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_553(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_288(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_289(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_290(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_291(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_292(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_293(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_294(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_295(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_296(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_297(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_554(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_298(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_299(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_300(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_301(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_302(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_303(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_304(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_305(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_306(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_307(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_555(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_8_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_308(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_309(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_310(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_311(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_312(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_313(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_314(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_315(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_316(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_317(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_556(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_318(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_319(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_320(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_321(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_322(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_323(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_324(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_325(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_326(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_327(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_557(
    handle: *const move_counted_handle_const_double_,
    layout: *const (),
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_328(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_329(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_330(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_331(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_332(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_333(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_334(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_335(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_336(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_337(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_558(
    handle: *const move_counted_handle_const_double_,
    layout: *const (),
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_338(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_339(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_340(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_341(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_342(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_343(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_344(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_345(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_346(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_347(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_559(
    handle: *const move_counted_handle_const_double_,
    layout: *const layout_wrapping_integral_8_,
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_348(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_349(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_350(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_351(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_352(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_353(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_354(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_355(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_356(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_357(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_560(
    handle: *const move_counted_handle_const_double_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_358(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_359(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_360(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_361(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_362(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_363(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_364(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_365(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_366(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_367(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_561(
    handle: *const *mut MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_368(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_369(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_370(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_371(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_372(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_373(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_374(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_375(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_376(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_377(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_562(
    handle: *const *mut MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_378(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_379(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_380(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_381(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_382(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_383(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_384(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_385(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_386(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_387(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_563(
    handle: *const *mut MinimalElementType,
    layout: *const layout_wrapping_integral_8_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_388(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_389(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_390(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_391(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_392(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_393(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_394(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_395(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_396(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_397(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_564(
    handle: *const *mut MinimalElementType,
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_398(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_399(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_400(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_401(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_402(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_403(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_404(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_405(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_406(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_407(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_565(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_408(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_409(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_410(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_411(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_412(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_413(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_414(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_415(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_416(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_417(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_566(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_418(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_419(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_420(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_421(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_422(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_423(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_424(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_425(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_426(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_427(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_567(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const layout_wrapping_integral_8_,
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_428(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_429(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_430(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_431(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_432(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_433(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_434(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_435(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_436(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_437(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_568(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_438(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_439(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_440(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_441(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_442(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_443(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_444(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_445(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_446(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_447(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_569(
    handle: *const *const MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_448(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_449(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_450(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_451(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_452(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_453(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_454(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_455(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_456(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_457(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_570(
    handle: *const *const MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_458(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_459(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_460(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_461(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_462(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_463(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_464(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_465(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_466(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_467(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_571(
    handle: *const *const MinimalElementType,
    layout: *const layout_wrapping_integral_8_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_468(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_469(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_470(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_471(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_472(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_473(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_474(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_475(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_476(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_477(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_572(
    handle: *const *const MinimalElementType,
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_478(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_479(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_480(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_481(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_482(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_483(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_484(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_485(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_486(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_487(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_573(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_488(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_489(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_490(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_491(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_492(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_493(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_494(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_495(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_496(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_497(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_574(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_24(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_498(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_25(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_499(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_500(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_501(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_502(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_503(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_26(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_504(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_27(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_505(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_28(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_506(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_29(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_507(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_575(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const layout_wrapping_integral_8_,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_int__ =
            (unsafe { construct_mapping_30((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_508(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_31 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_509(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_510(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_511(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_512(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_513(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_32((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_514(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_33 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_515(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_34 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_516(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_35 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_517(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_extents_576(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_36((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_518(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_37 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_519(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_520(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_521(_handle, &mut _map, _arg2, 2_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_522(_handle, &mut _map, _arg2, 0_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_523(_handle, &mut _map, _arg2, 1_(), 2_(), 3_(), 2_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_38((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_524(_handle, &mut _map, _arg2, 7_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_39 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_525(_handle, &mut _map, _arg2, 2_(), 4_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_40 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_526(_handle, &mut _map, _arg2, 0_(), 7_(), 3_())
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_41 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_527(
            _handle,
            &mut _map,
            _arg2,
            1_(),
            7_(),
            2_(),
            4_(),
            3_(),
            2_(),
        )
    });
}
pub unsafe fn mixin_layout_577(handle: *const *mut (), acc: *const ()) {
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_528(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_530(_handle, &mut _layout, _acc)
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
        mixin_extents_531(_handle, &mut _layout, _acc)
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
        mixin_extents_532(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_578(
    handle: *const move_counted_handle_int_,
    acc: *const checked_accessor_int_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_int_ = acc;
        mixin_extents_533(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_int_ = acc;
        mixin_extents_534(_handle, &mut _layout, _acc)
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
        mixin_extents_535(_handle, &mut _layout, _acc)
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
        mixin_extents_536(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_579(handle: *const *const (), acc: *const ()) {
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_537(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_538(_handle, &mut _layout, _acc)
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
        mixin_extents_539(_handle, &mut _layout, _acc)
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
        mixin_extents_540(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_580(
    handle: *const move_counted_handle_const_int_,
    acc: *const checked_accessor_const_int_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_int_ = acc;
        mixin_extents_541(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_int_ = acc;
        mixin_extents_542(_handle, &mut _layout, _acc)
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
        mixin_extents_543(_handle, &mut _layout, _acc)
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
        mixin_extents_544(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_581(handle: *const *mut (), acc: *const ()) {
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_545(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_546(_handle, &mut _layout, _acc)
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
        mixin_extents_547(_handle, &mut _layout, _acc)
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
        mixin_extents_548(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_582(
    handle: *const no_default_ctor_handle_double_,
    acc: *const checked_accessor_double_,
) {
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_double_ = acc;
        mixin_extents_549(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_double_ = acc;
        mixin_extents_550(_handle, &mut _layout, _acc)
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
        mixin_extents_551(_handle, &mut _layout, _acc)
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
        mixin_extents_552(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_583(handle: *const *const (), acc: *const ()) {
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_553(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
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
        let _handle: *const *const () = handle;
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
        let _handle: *const *const () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_556(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_584(
    handle: *const move_counted_handle_const_double_,
    acc: *const checked_accessor_const_double_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_double_ = acc;
        mixin_extents_557(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_double_ = acc;
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
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const checked_accessor_const_double_ = acc;
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
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_const_double_ = acc;
        mixin_extents_560(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_585(handle: *const *mut MinimalElementType, acc: *const ()) {
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_561(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
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
        let _handle: *const *mut MinimalElementType = handle;
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
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_564(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_586(
    handle: *const move_counted_handle_MinimalElementType_,
    acc: *const checked_accessor_MinimalElementType_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        mixin_extents_565(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
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
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
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
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        mixin_extents_568(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_587(handle: *const *const MinimalElementType, acc: *const ()) {
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_569(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
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
        let _handle: *const *const MinimalElementType = handle;
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
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_572(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_588(
    handle: *const move_counted_handle_const_MinimalElementType_,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        mixin_extents_573(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
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
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: layout_wrapping_integral_8_ = <layout_wrapping_integral_8_>::default();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
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
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        mixin_extents_576(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_accessor_589() {
    let mut elements: ElementPool_int__1024_ = ElementPool_int__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_int__1024_::destructor);
    (unsafe {
        let mut _handle: *mut () = (unsafe { ElementPool_int__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_577(&mut _handle, &mut _acc)
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
        mixin_layout_578(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_590() {
    let mut elements: ElementPool_const_int__1024_ = ElementPool_const_int__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_const_int__1024_::destructor);
    (unsafe {
        let mut _handle: *const () =
            (unsafe { ElementPool_const_int__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_579(&mut _handle, &mut _acc)
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
        mixin_layout_580(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_591() {
    let mut elements: ElementPool_double__1024_ = ElementPool_double__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_double__1024_::destructor);
    (unsafe {
        let mut _handle: *mut () = (unsafe { ElementPool_double__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_581(&mut _handle, &mut _acc)
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
        mixin_layout_582(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_592() {
    let mut elements: ElementPool_const_double__1024_ = ElementPool_const_double__1024_::new();
    let _dtor_elements = ScopedDestructorUnsafe::new(
        &raw mut elements,
        ElementPool_const_double__1024_::destructor,
    );
    (unsafe {
        let mut _handle: *const () =
            (unsafe { ElementPool_const_double__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_583(&mut _handle, &mut _acc)
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
        mixin_layout_584(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_593() {
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
        mixin_layout_585(&mut _handle, &mut _acc)
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
        mixin_layout_586(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_594() {
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
        mixin_layout_587(&mut _handle, &mut _acc)
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
        mixin_layout_588(&mut _handle, &mut _acc)
    });
}
pub unsafe fn test_595() -> () {
    (unsafe { mixin_accessor_589() });
    (unsafe { mixin_accessor_590() });
    (unsafe { mixin_accessor_591() });
    (unsafe { mixin_accessor_592() });
    (unsafe { mixin_accessor_593() });
    (unsafe { mixin_accessor_594() });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_529));
    const _: () = assert!(
        true,
        "std::is_constructible_v<mds_t, float*, int, int, int>"
    );;
    const _: () = assert!(true, "std::is_constructible_v<mds_t, float*, int, int>");;
    const _: () = assert!(true, "!std::is_constructible_v<mds_t, float*, int>");;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, int, int, int, int>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, int, int, std::dextents<int, 1>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, const float*, int, int>"
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
    (unsafe { test_595() });
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
