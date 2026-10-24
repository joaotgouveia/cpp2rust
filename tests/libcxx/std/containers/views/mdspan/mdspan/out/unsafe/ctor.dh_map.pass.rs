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
static mut Wrap_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
pub unsafe fn operator_eq_1(
    lhs: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    rhs: *const layout_wrapping_integral_4__mapping_std_extents_int__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_int__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_int__ {
    pub unsafe fn copy_from(
        other: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    ) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_1(ext: *const (), mut _a1: not_extents_constructible_tag) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn extents(&self) -> *const () {
        return &self.extents_;
    }
}
impl std::cmp::PartialEq for layout_wrapping_integral_4__mapping_std_extents_int__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_1(
                self as *const layout_wrapping_integral_4__mapping_std_extents_int__,
                other as *const layout_wrapping_integral_4__mapping_std_extents_int__,
            )
        }
    }
}
impl std::cmp::Eq for layout_wrapping_integral_4__mapping_std_extents_int__ {}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_int__ {
    fn clone(&self) -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_int__::copy_from(
                self as *const layout_wrapping_integral_4__mapping_std_extents_int__,
            )
        }
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
static mut Wrap_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
pub unsafe fn operator_eq_3(
    lhs: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    rhs: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    pub unsafe fn copy_from(
        other : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    ) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_1(ext: *const (), mut _a1: not_extents_constructible_tag) -> Self {
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
    for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__
{
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_3(self as *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__, other as *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__)
        }
    }
}
impl std::cmp::Eq
    for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__
{
}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    fn clone(&self) -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__)
        }
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
static mut Wrap_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
pub unsafe fn operator_eq_5(
    lhs: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    rhs: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    pub unsafe fn copy_from(
        other: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    ) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn new_1(ext: *const (), mut _a1: not_extents_constructible_tag) -> Self {
        let mut this = Self {
            extents_: unimplemented!(),
        };
        this
    }
    pub unsafe fn extents(&self) -> *const () {
        return &self.extents_;
    }
}
impl std::cmp::PartialEq for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
                self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
                other as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
            )
        }
    }
}
impl std::cmp::Eq for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    fn clone(&self) -> Self {
        unsafe {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::copy_from(
                self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
            )
        }
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
static mut Wrap_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { (4_() as ()) });
pub unsafe fn operator_eq_7(
    lhs : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    rhs : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { pub unsafe fn copy_from ( other : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } }
impl std::cmp::PartialEq for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_7(self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__, other as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__) }}}
impl std::cmp::Eq for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ {}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn clone(&self) -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__) } } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_ : unimplemented!() , } } } }
static mut Wrap_8: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
pub unsafe fn operator_eq_9(
    lhs : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    rhs : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { pub unsafe fn copy_from ( other : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } }
impl std::cmp::PartialEq for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_9(self as *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__, other as *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__) }}}
impl std::cmp::Eq for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ {}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn clone(&self) -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__) } } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_ : unimplemented!() , } } } }
static mut Wrap_10: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
pub unsafe fn operator_eq_11(
    lhs : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    rhs : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
) -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { pub unsafe fn copy_from ( other : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } }
impl std::cmp::PartialEq for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn eq(&self, other: &Self) -> bool { unsafe { operator_eq_11(self as *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__, other as *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__) }}}
impl std::cmp::Eq for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ {}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn clone(&self) -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__) } } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct layout_wrapping_integral_4_ {}
pub unsafe fn construct_mapping_12(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_13(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_14(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_15(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_16(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_17(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
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
pub unsafe fn construct_mapping_24(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__ {
    return layout_wrapping_integral_4__mapping_std_extents_int__::new_1({ &exts }, {
        <not_extents_constructible_tag>::default()
    });
}
pub unsafe fn construct_mapping_25(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}
pub unsafe fn construct_mapping_26(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::new_1({ &exts }, {
        <not_extents_constructible_tag>::default()
    });
}pub unsafe fn construct_mapping_27 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}pub unsafe fn construct_mapping_28 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}pub unsafe fn construct_mapping_29 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
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
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_30));
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
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_31));
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
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_32));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct move_counted_handle_double_ {
    pub ptr: *mut (),
}
impl move_counted_handle_double_ {
    pub unsafe fn move_counter() -> *mut () {
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_33));
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
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_34));
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
        return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_35));
    }
}
static mut c_30: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut c_31: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut c_32: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut c_33: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut c_34: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
static mut c_35: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
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
pub unsafe fn test_mdspan_types_36(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_37(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_38(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_39(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_40(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_41(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_42(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_43(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_44(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_45(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_46(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_47(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_48(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_50(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_55(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_56(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_57(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_58(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_59(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_60(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_61(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_62(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_63(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_64(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_65(
    handle: *const move_counted_handle_int_,
    map: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_66(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_67(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_68(
    handle: *const move_counted_handle_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_69(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_70(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_71(
    handle: *const move_counted_handle_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_72(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_73(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_74(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_75(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_76(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_77(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_78(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_79(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_80(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_81(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_82(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_83(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_84(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_85(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_86(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_87(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_88(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_89(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_int_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_90(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_91(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_92(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_93(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_94(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_95(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_96(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_97(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_98(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_99(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_100(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_101(
    handle: *const move_counted_handle_const_int_,
    map: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_102(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_103(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_104(
    handle: *const move_counted_handle_const_int_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_105(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_106(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_107(
    handle: *const move_counted_handle_const_int_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_108(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_109(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_110(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_111(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_112(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_113(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_114(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_115(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_116(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_117(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_118(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_119(handle: *const *mut (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_120(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_121(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_122(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_123(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_124(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_125(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_126(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_127(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_128(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_129(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_130(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_131(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_132(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_133(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_134(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_135(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_136(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_137(
    handle: *const no_default_ctor_handle_double_,
    map: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_138(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_139(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_140(
    handle: *const no_default_ctor_handle_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_141(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_142(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_143(
    handle: *const no_default_ctor_handle_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_144(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_145(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_146(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_147(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_148(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_149(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_150(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_151(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_152(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_153(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_154(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_155(handle: *const *const (), map: *const (), _a2: *const ()) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_159(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_160(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_161(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_double_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_162(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_163(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_164(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_165(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_166(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_167(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_168(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_169(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_170(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_171(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_172(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_173(
    handle: *const move_counted_handle_const_double_,
    map: *const (),
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_174(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_175(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_176(
    handle: *const move_counted_handle_const_double_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_177(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_178(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_179(
    handle: *const move_counted_handle_const_double_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
) {
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
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_180(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_181(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_182(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_183(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_184(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_185(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_186(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_187(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_188(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_189(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_190(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_191(
    handle: *const *mut MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_192(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_193(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_194(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_195(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_196(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_197(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_198(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_199(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_200(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_201(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_202(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_203(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_204(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_205(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_206(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_207(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_208(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_209(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_210(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_211(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_212(
    handle: *const move_counted_handle_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_213(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_214(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_215(
    handle: *const move_counted_handle_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_216(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_218(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
pub unsafe fn test_mdspan_types_220(
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            (*(unsafe { move_counted_handle_const_MinimalElementType_::move_counter() })) = 0_();
        }
        let mut m: () = unimplemented!();
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map))");;
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
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_235(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_236(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_237(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_238(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_239(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_240(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_241(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_242(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_243(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_244(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_245(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_246(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_247(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_248(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_249(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_250(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn test_mdspan_types_251(
    handle: *const move_counted_handle_const_MinimalElementType_,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_constructible_v<MDS, const H&, const M&>");;
    }
}
pub unsafe fn mixin_extents_252(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_36(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_37(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_38(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_39(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_40(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_41(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_254(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_42(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_43(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_44(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_45(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_46(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_47(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_255(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_24((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_48(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_25 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_49(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_26((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_50(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_27 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_51(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_28 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_52(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_29 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_53(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_256(
    handle: *const move_counted_handle_int_,
    layout: *const (),
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_54(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_55(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_56(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_57(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_58(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_59(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_257(
    handle: *const move_counted_handle_int_,
    layout: *const (),
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_60(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_61(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_62(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_63(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_64(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_65(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_258(
    handle: *const move_counted_handle_int_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_24((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_66(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_25 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_67(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_26((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_68(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_27 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_69(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_28 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_70(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_29 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_71(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_259(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_72(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_73(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_74(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_75(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_76(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_77(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_260(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_78(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_79(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_80(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_81(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_82(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_83(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_261(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_24((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_84(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_25 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_85(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_26((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_86(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_27 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_87(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_28 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_88(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_29 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_89(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_262(
    handle: *const move_counted_handle_const_int_,
    layout: *const (),
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_90(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_91(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_92(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_93(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_94(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_95(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_263(
    handle: *const move_counted_handle_const_int_,
    layout: *const (),
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_96(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_97(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_98(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_99(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_100(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_101(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_264(
    handle: *const move_counted_handle_const_int_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_24((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_102(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_25 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_103(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_26((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_104(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_27 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_105(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_28 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_106(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_29 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_107(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_265(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_108(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_109(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_110(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_111(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_112(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_113(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_266(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_114(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_115(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_116(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_117(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_118(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_119(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_267(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_24((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_120(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_25 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_121(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_26((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_122(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_27 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_123(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_28 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_124(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_29 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_125(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_268(
    handle: *const no_default_ctor_handle_double_,
    layout: *const (),
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_126(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_127(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_128(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_129(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_130(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_131(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_269(
    handle: *const no_default_ctor_handle_double_,
    layout: *const (),
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_132(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_133(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_134(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_135(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_136(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_137(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_270(
    handle: *const no_default_ctor_handle_double_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_24((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_138(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_25 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_139(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_26((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_140(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_27 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_141(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_28 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_142(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_29 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_143(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_271(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_144(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_145(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_146(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_147(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_148(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_149(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_272(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_150(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_151(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_152(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_153(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_154(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_155(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_273(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_24((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_156(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_25 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_157(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_26((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_158(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_27 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_159(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_28 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_160(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_29 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_161(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_274(
    handle: *const move_counted_handle_const_double_,
    layout: *const (),
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_162(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_163(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_164(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_165(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_166(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_167(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_275(
    handle: *const move_counted_handle_const_double_,
    layout: *const (),
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_168(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_169(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_170(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_171(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_172(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_173(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_276(
    handle: *const move_counted_handle_const_double_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_24((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_174(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_25 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_175(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_26((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_176(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_27 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_177(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_28 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_178(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_29 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_179(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_277(
    handle: *const *mut MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_180(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_181(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_182(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_183(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_184(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_185(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_278(
    handle: *const *mut MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_186(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_187(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_188(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_189(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_190(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_191(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_279(
    handle: *const *mut MinimalElementType,
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_24((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_192(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_25 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_193(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_26((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_194(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_27 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_195(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_28 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_196(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_29 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_197(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_280(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_198(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_199(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_200(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_201(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_202(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_203(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_281(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_204(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_205(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_206(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_207(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_208(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_209(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_282(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_24((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_210(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_25 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_211(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_26((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_212(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_27 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_213(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_28 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_214(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_29 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_215(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_283(
    handle: *const *const MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_216(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_217(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_218(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_219(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_220(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_221(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_284(
    handle: *const *const MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_222(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_223(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_224(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_225(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_226(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_227(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_285(
    handle: *const *const MinimalElementType,
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_24((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_228(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_25 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_229(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_26((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_230(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_27 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_231(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_28 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_232(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_29 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_233(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_286(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_234(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_235(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_236(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_237(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_238(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_239(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_287(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_18(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_240(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_19(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_241(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_20(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_242(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_21(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_243(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_22(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_244(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: () = (unsafe { construct_mapping_23(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_245(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_extents_288(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_24((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_246(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_25 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_247(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_26((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_248(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_27 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_249(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_28 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_250(_handle, &mut _map, _arg2)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_29 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_251(_handle, &mut _map, _arg2)
    });
}
pub unsafe fn mixin_layout_289(handle: *const *mut (), acc: *const ()) {
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_252(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_254(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_255(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_290(
    handle: *const move_counted_handle_int_,
    acc: *const checked_accessor_int_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_int_ = acc;
        mixin_extents_256(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_int_ = acc;
        mixin_extents_257(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_int_ = acc;
        mixin_extents_258(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_291(handle: *const *const (), acc: *const ()) {
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_259(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_260(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_261(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_292(
    handle: *const move_counted_handle_const_int_,
    acc: *const checked_accessor_const_int_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_int_ = acc;
        mixin_extents_262(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_int_ = acc;
        mixin_extents_263(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_const_int_ = acc;
        mixin_extents_264(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_293(handle: *const *mut (), acc: *const ()) {
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_265(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_266(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_267(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_294(
    handle: *const no_default_ctor_handle_double_,
    acc: *const checked_accessor_double_,
) {
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_double_ = acc;
        mixin_extents_268(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_double_ = acc;
        mixin_extents_269(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_double_ = acc;
        mixin_extents_270(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_295(handle: *const *const (), acc: *const ()) {
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_271(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_272(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_273(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_296(
    handle: *const move_counted_handle_const_double_,
    acc: *const checked_accessor_const_double_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_double_ = acc;
        mixin_extents_274(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_double_ = acc;
        mixin_extents_275(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_const_double_ = acc;
        mixin_extents_276(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_297(handle: *const *mut MinimalElementType, acc: *const ()) {
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_277(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_278(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_279(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_298(
    handle: *const move_counted_handle_MinimalElementType_,
    acc: *const checked_accessor_MinimalElementType_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        mixin_extents_280(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        mixin_extents_281(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        mixin_extents_282(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_299(handle: *const *const MinimalElementType, acc: *const ()) {
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_283(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_284(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_285(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_300(
    handle: *const move_counted_handle_const_MinimalElementType_,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        mixin_extents_286(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        mixin_extents_287(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        mixin_extents_288(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_accessor_301() {
    let mut elements: ElementPool_int__1024_ = ElementPool_int__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_int__1024_::destructor);
    (unsafe {
        let mut _handle: *mut () = (unsafe { ElementPool_int__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_289(&mut _handle, &mut _acc)
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
        mixin_layout_290(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_302() {
    let mut elements: ElementPool_const_int__1024_ = ElementPool_const_int__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_const_int__1024_::destructor);
    (unsafe {
        let mut _handle: *const () =
            (unsafe { ElementPool_const_int__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_291(&mut _handle, &mut _acc)
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
        mixin_layout_292(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_303() {
    let mut elements: ElementPool_double__1024_ = ElementPool_double__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_double__1024_::destructor);
    (unsafe {
        let mut _handle: *mut () = (unsafe { ElementPool_double__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_293(&mut _handle, &mut _acc)
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
        mixin_layout_294(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_304() {
    let mut elements: ElementPool_const_double__1024_ = ElementPool_const_double__1024_::new();
    let _dtor_elements = ScopedDestructorUnsafe::new(
        &raw mut elements,
        ElementPool_const_double__1024_::destructor,
    );
    (unsafe {
        let mut _handle: *const () =
            (unsafe { ElementPool_const_double__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_295(&mut _handle, &mut _acc)
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
        mixin_layout_296(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_305() {
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
        mixin_layout_297(&mut _handle, &mut _acc)
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
        mixin_layout_298(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_306() {
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
        mixin_layout_299(&mut _handle, &mut _acc)
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
        mixin_layout_300(&mut _handle, &mut _acc)
    });
}
pub unsafe fn test_307() -> () {
    (unsafe { mixin_accessor_301() });
    (unsafe { mixin_accessor_302() });
    (unsafe { mixin_accessor_303() });
    (unsafe { mixin_accessor_304() });
    (unsafe { mixin_accessor_305() });
    (unsafe { mixin_accessor_306() });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_253));
    const _: () = assert!(
        true,
        "std::is_constructible_v<mds_t, float*, mapping_t<std::extents<int, 3, D, D>>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, mapping_t<std::extents<int, D, D>>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, mapping_t<std::extents<int, D, D, D, D>>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, mapping_t<std::extents<int, D, D, D>>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, mapping_t<std::extents<unsigned, 3, D, D>>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, const float*, mapping_t<std::extents<int, 3, D, D>>>"
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
    (unsafe { test_307() });
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
}
