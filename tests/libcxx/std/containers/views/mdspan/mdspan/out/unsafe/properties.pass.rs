extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ptrconst_ref__int_ptrconst_ref_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_extents_int__ref__const_std_extents_int__ref_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_layout_left_mapping_std_extents_int___ref__const_std_layout_left_mapping_std_extents_int___ref_
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_default_accessor_int__ref__const_std_default_accessor_int__ref_
{}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_extents_signed_char__18446744073709551615__ref__const_std_extents_signed_char__18446744073709551615__ref_
{}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_signed_char__signed_char_ {}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_char__unsigned_char_ {}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_layout_left_mapping_std_extents_signed_char__18446744073709551615___ref__const_std_layout_left_mapping_std_extents_signed_char__18446744073709551615___ref_
{}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_extents_unsigned_int__7__ref__const_std_extents_unsigned_int__7__ref_
{}
pub type anon_12 = ();
pub const anon_12_value: anon_12 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_layout_left_mapping_std_extents_unsigned_int__7___ref__const_std_layout_left_mapping_std_extents_unsigned_int__7___ref_
{}
pub type anon_13 = ();
pub const anon_13_value: anon_13 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ref__const_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ref_
{}
pub type anon_14 = ();
pub const anon_14_value: anon_14 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_layout_left_mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___ref__const_std_layout_left_mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___ref_
{}
pub type anon_15 = ();
pub const anon_15_value: anon_15 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ref__const_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ref_
{}
pub type anon_16 = ();
pub const anon_16_value: anon_16 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_layout_left_mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615___ref__const_std_layout_left_mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615___ref_
{}
pub type anon_17 = ();
pub const anon_17_value: anon_17 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ref__const_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ref_
{}
pub type anon_18 = ();
pub const anon_18_value: anon_18 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_long__long_ {}
pub type anon_19 = ();
pub const anon_19_value: anon_19 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_layout_left_mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___ref__const_std_layout_left_mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___ref_
{}
pub type anon_20 = ();
pub const anon_20_value: anon_20 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_layout_right_mapping_std_extents_int___ref__const_std_layout_right_mapping_std_extents_int___ref_
{}
pub type anon_21 = ();
pub const anon_21_value: anon_21 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_layout_right_mapping_std_extents_signed_char__18446744073709551615___ref__const_std_layout_right_mapping_std_extents_signed_char__18446744073709551615___ref_
{}
pub type anon_22 = ();
pub const anon_22_value: anon_22 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_layout_right_mapping_std_extents_unsigned_int__7___ref__const_std_layout_right_mapping_std_extents_unsigned_int__7___ref_
{}
pub type anon_23 = ();
pub const anon_23_value: anon_23 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_layout_right_mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___ref__const_std_layout_right_mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___ref_
{}
pub type anon_24 = ();
pub const anon_24_value: anon_24 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_layout_right_mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615___ref__const_std_layout_right_mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615___ref_
{}
pub type anon_25 = ();
pub const anon_25_value: anon_25 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_layout_right_mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___ref__const_std_layout_right_mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___ref_
{}
pub type anon_26 = ();
pub const anon_26_value: anon_26 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_layout_wrapping_integral_4__mapping_std_extents_int___ref__const_layout_wrapping_integral_4__mapping_std_extents_int___ref_
{}
pub type anon_27 = ();
pub const anon_27_value: anon_27 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615___ref__const_layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615___ref_
{}
pub type anon_28 = ();
pub const anon_28_value: anon_28 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7___ref__const_layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7___ref_
{}
pub type anon_29 = ();
pub const anon_29_value: anon_29 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___ref__const_layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___ref_
{}
pub type anon_30 = ();
pub const anon_30_value: anon_30 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615___ref__const_layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615___ref_
{}
pub type anon_31 = ();
pub const anon_31_value: anon_31 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___ref__const_layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___ref_
{}
pub type anon_32 = ();
pub const anon_32_value: anon_32 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ptrconst_ref__const_int_ptrconst_ref_ {}
pub type anon_33 = ();
pub const anon_33_value: anon_33 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_default_accessor_const_int__ref__const_std_default_accessor_const_int__ref_
{}
pub type anon_34 = ();
pub const anon_34_value: anon_34 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_double_ptrconst_ref__double_ptrconst_ref_ {}
pub type anon_35 = ();
pub const anon_35_value: anon_35 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_default_accessor_double__ref__const_std_default_accessor_double__ref_
{}
pub type anon_36 = ();
pub const anon_36_value: anon_36 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_double_ptrconst_ref__const_double_ptrconst_ref_ {}
pub type anon_37 = ();
pub const anon_37_value: anon_37 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_default_accessor_const_double__ref__const_std_default_accessor_const_double__ref_
{}
pub type anon_38 = ();
pub const anon_38_value: anon_38 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_MinimalElementType_ptrconst_ref__MinimalElementType_ptrconst_ref_
{}
pub type anon_39 = ();
pub const anon_39_value: anon_39 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_default_accessor_MinimalElementType__ref__const_std_default_accessor_MinimalElementType__ref_
{}
pub type anon_40 = ();
pub const anon_40_value: anon_40 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_MinimalElementType_ptrconst_ref__const_MinimalElementType_ptrconst_ref_
{}
pub type anon_41 = ();
pub const anon_41_value: anon_41 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_default_accessor_const_MinimalElementType__ref__const_std_default_accessor_const_MinimalElementType__ref_
{}
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
static mut Wrap_42: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
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
    pub unsafe fn is_always_unique() -> () {
        return false;
    }
    pub unsafe fn is_always_exhaustive() -> () {
        return true;
    }
    pub unsafe fn is_always_strided() -> () {
        return false;
    }
    pub unsafe fn is_unique(&self) -> () {
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!()) > (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_42))) {
                return false;
            }
            r.postfix_inc();
        }
        return true;
    }
    pub unsafe fn is_exhaustive() -> () {
        return true;
    }
    pub unsafe fn is_strided(&self) -> () {
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!()) > (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_42))) {
                return false;
            }
            r.postfix_inc();
        }
        return true;
    }
}
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
static mut Wrap_43: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
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
    pub unsafe fn is_always_unique() -> () {
        return false;
    }
    pub unsafe fn is_always_exhaustive() -> () {
        return true;
    }
    pub unsafe fn is_always_strided() -> () {
        return false;
    }
    pub unsafe fn is_unique(&self) -> () {
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!()) > (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_43))) {
                return false;
            }
            r.postfix_inc();
        }
        return true;
    }
    pub unsafe fn is_exhaustive() -> () {
        return true;
    }
    pub unsafe fn is_strided(&self) -> () {
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!()) > (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_43))) {
                return false;
            }
            r.postfix_inc();
        }
        return true;
    }
    pub unsafe fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = (unimplemented!() as ()).wrapping_sub(1_());
        'loop_: while ((i) > (r)) {
            s *= unimplemented!();
            i.postfix_dec();
        }
        return s;
    }
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
static mut Wrap_44: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
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
    pub unsafe fn is_always_unique() -> () {
        return false;
    }
    pub unsafe fn is_always_exhaustive() -> () {
        return true;
    }
    pub unsafe fn is_always_strided() -> () {
        return false;
    }
    pub unsafe fn is_unique(&self) -> () {
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!()) > (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_44))) {
                return false;
            }
            r.postfix_inc();
        }
        return true;
    }
    pub unsafe fn is_exhaustive() -> () {
        return true;
    }
    pub unsafe fn is_strided(&self) -> () {
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            if ((unimplemented!()) > (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_44))) {
                return false;
            }
            r.postfix_inc();
        }
        return true;
    }
    pub unsafe fn stride(&self, mut r: ()) -> () {
        let mut s: () = 1_();
        let mut i: () = (unimplemented!() as ()).wrapping_sub(1_());
        'loop_: while ((i) > (r)) {
            s = (s).wrapping_mul(unimplemented!());
            i.postfix_dec();
        }
        return s;
    }
}
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
static mut Wrap_45: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { (4_() as ()) });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { pub unsafe fn copy_from ( other : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } pub unsafe fn is_always_unique ( ) -> () { return false  ;
 } pub unsafe fn is_always_exhaustive ( ) -> () { return true  ;
 } pub unsafe fn is_always_strided ( ) -> () { return false  ;
 } pub unsafe fn is_unique ( &self , ) -> () { let mut r : () = 0_() ;
  ;
 ;
 'loop_: while ( ( r ) < ( unimplemented!()  ) ) { if ( ( unimplemented!()  ) > ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_45)) ) ) { return false  ;
 } r .postfix_inc() ;
 } return true  ;
 } pub unsafe fn is_exhaustive ( ) -> () { return true  ;
 } pub unsafe fn is_strided ( &self , ) -> () { let mut r : () = 0_() ;
  ;
 ;
 'loop_: while ( ( r ) < ( unimplemented!()  ) ) { if ( ( unimplemented!()  ) > ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_45)) ) ) { return false  ;
 } r .postfix_inc() ;
 } return true  ;
 } pub unsafe fn stride ( &self , mut r : () , ) -> () { let mut s : () = 1_() ;
  ;
 ;
 let mut i : () = ( ( unimplemented!()  as () ) ) . wrapping_sub ( 1_() ) ;
  ;
 ;
 'loop_: while ( ( i ) > ( r ) ) { s = ( s ) . wrapping_mul ( unimplemented!()  ) ;
 i .postfix_dec() ;
 } return s  ;
 } }
impl Clone for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn clone(&self) -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__) } } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_ : unimplemented!() , } } } }
static mut Wrap_46: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { pub unsafe fn copy_from ( other : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } pub unsafe fn is_always_unique ( ) -> () { return false  ;
 } pub unsafe fn is_always_exhaustive ( ) -> () { return true  ;
 } pub unsafe fn is_always_strided ( ) -> () { return false  ;
 } pub unsafe fn is_unique ( &self , ) -> () { let mut r : () = 0_() ;
  ;
 ;
 'loop_: while ( ( r ) < ( unimplemented!()  ) ) { if ( ( unimplemented!()  ) > ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_46)) ) ) { return false  ;
 } r .postfix_inc() ;
 } return true  ;
 } pub unsafe fn is_exhaustive ( ) -> () { return true  ;
 } pub unsafe fn is_strided ( &self , ) -> () { let mut r : () = 0_() ;
  ;
 ;
 'loop_: while ( ( r ) < ( unimplemented!()  ) ) { if ( ( unimplemented!()  ) > ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_46)) ) ) { return false  ;
 } r .postfix_inc() ;
 } return true  ;
 } pub unsafe fn stride ( &self , mut r : () , ) -> () { let mut s : () = 1_() ;
  ;
 ;
 let mut i : () = ( ( unimplemented!()  as () ) ) . wrapping_sub ( 1_() ) ;
  ;
 ;
 'loop_: while ( ( i ) > ( r ) ) { s  *= unimplemented!()   ;
 i .postfix_dec() ;
 } return s  ;
 } }
impl Clone for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn clone(&self) -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__) } } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_ : unimplemented!() , } } } }
static mut Wrap_47: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { pub unsafe fn copy_from ( other : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } pub unsafe fn extents ( &self , ) -> *const () { return & self . extents_   ;
 } pub unsafe fn is_always_unique ( ) -> () { return false  ;
 } pub unsafe fn is_always_exhaustive ( ) -> () { return true  ;
 } pub unsafe fn is_always_strided ( ) -> () { return false  ;
 } pub unsafe fn is_unique ( &self , ) -> () { let mut r : () = 0_() ;
  ;
 ;
 'loop_: while ( ( r ) < ( unimplemented!()  ) ) { if ( ( unimplemented!()  ) > ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_47)) ) ) { return false  ;
 } r .postfix_inc() ;
 } return true  ;
 } pub unsafe fn is_exhaustive ( ) -> () { return true  ;
 } pub unsafe fn is_strided ( &self , ) -> () { let mut r : () = 0_() ;
  ;
 ;
 'loop_: while ( ( r ) < ( unimplemented!()  ) ) { if ( ( unimplemented!()  ) > ( (*std::cell::LazyCell::force_mut(&mut *&raw mut Wrap_47)) ) ) { return false  ;
 } r .postfix_inc() ;
 } return true  ;
 } pub unsafe fn stride ( &self , mut r : () , ) -> () { let mut s : () = 1_() ;
  ;
 ;
 let mut i : () = ( ( unimplemented!()  as () ) ) . wrapping_sub ( 1_() ) ;
  ;
 ;
 'loop_: while ( ( i ) > ( r ) ) { s  *= unimplemented!()   ;
 i .postfix_dec() ;
 } return s  ;
 } }
impl Clone for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn clone(&self) -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__::copy_from(self as *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__) } } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct layout_wrapping_integral_4_ {}
pub unsafe fn construct_mapping_48(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_49(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_50(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_51(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_52(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_53(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_54(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_55(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_56(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_57(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_58(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_59(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_60(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__ {
    return layout_wrapping_integral_4__mapping_std_extents_int__::new_1({ &exts }, {
        <not_extents_constructible_tag>::default()
    });
}
pub unsafe fn construct_mapping_61(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}
pub unsafe fn construct_mapping_62(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::new_1({ &exts }, {
        <not_extents_constructible_tag>::default()
    });
}pub unsafe fn construct_mapping_63 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}pub unsafe fn construct_mapping_64 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}pub unsafe fn construct_mapping_65 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
    return layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct always_convertible_layout {}
pub unsafe fn test_mdspan_types_66(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_67(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_68(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_69(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_70(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_71(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_72(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_73(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_74(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_75(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_76(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_77(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_78(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_79(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_80(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_81(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_82(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_83(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_84(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_85(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_86(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_87(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_88(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_89(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_90(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_91(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_92(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_93(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_94(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_95(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_96(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_97(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_98(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_99(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_100(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_101(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_102(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_103(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_104(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_105(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_106(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_107(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_108(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_109(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_110(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_111(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_112(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_113(handle: *const *mut (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_114(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_115(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_116(
    handle: *const *mut (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_117(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_118(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_119(
    handle: *const *mut (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_120(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_121(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_122(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_123(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_124(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_125(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_126(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_127(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_128(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_129(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_130(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_131(handle: *const *const (), map: *const (), acc: *const ()) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_132(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_133(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_134(
    handle: *const *const (),
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_135(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_136(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_137(
    handle: *const *const (),
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_138(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_139(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_140(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_141(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_142(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_143(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_144(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_145(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_146(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_147(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_148(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_149(
    handle: *const *mut MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_150(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_151(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_152(
    handle: *const *mut MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_153(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_154(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_155(
    handle: *const *mut MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_156(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_157(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_158(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_159(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_160(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_161(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_162(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_163(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_164(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_165(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_166(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_167(
    handle: *const *const MinimalElementType,
    map: *const (),
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_168(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_169(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_170(
    handle: *const *const MinimalElementType,
    map: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        {}
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_171(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_172(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size *= unimplemented!();
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_mdspan_types_173(
    handle: *const *const MinimalElementType,
    map : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    acc: *const (),
) {
    let mut m: () = unimplemented!();
    const _: () = assert!(true, "std::copyable<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_constructible_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_move_assignable_v<MDS>");;
    const _: () = assert!(true, "std::is_nothrow_swappable_v<MDS>");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    {
        let mut size: () = 1_();
        let mut r: () = 0_();
        'loop_: while ((r) < (unimplemented!())) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            size = (size).wrapping_mul(unimplemented!());
            r.postfix_inc();
        }
        unimplemented!();
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "noexcept(MDS::is_always_unique())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_exhaustive())");;
    const _: () = assert!(true, "noexcept(MDS::is_always_strided())");;
    const _: () = assert!(true, "!noexcept(m.is_unique())");;
    const _: () = assert!(true, "!noexcept(m.is_exhaustive())");;
    const _: () = assert!(true, "!noexcept(m.is_strided())");;
    const _: () = assert!(true, "MDS::is_always_unique() == M::is_always_unique()");;
    const _: () = assert!(
        true,
        "MDS::is_always_exhaustive() == M::is_always_exhaustive()"
    );;
    const _: () = assert!(true, "MDS::is_always_strided() == M::is_always_strided()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        if unimplemented!() {
            let mut r: () = 0_();
            'loop_: while ((r) < (unimplemented!())) {
                const _: () = assert!(true, "");;
                const _: () = assert!(true, "!noexcept(m.stride(r))");;
                unimplemented!();
                r.postfix_inc();
            }
        }
    }
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn mixin_extents_174(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_48(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_66(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_49(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_67(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_50(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_68(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_51(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_69(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_52(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_70(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_53(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_71(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_176(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_54(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_72(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_55(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_73(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_56(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_74(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_57(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_75(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_58(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_76(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_59(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_77(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_177(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_60((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_78(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_61 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_79(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_62((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_80(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_63 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_81(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_64 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_82(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_65 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_83(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_178(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_48(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_84(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_49(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_85(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_50(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_86(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_51(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_87(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_52(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_88(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_53(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_89(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_179(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_54(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_90(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_55(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_91(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_56(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_92(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_57(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_93(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_58(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_94(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_59(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_95(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_180(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_60((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_96(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_61 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_97(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_62((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_98(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_63 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_99(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_64 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_100(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_65 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_101(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_181(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_48(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_102(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_49(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_103(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_50(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_104(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_51(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_105(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_52(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_106(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_53(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_107(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_182(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_54(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_108(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_55(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_109(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_56(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_110(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_57(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_111(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_58(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_112(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: () = (unsafe { construct_mapping_59(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_113(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_183(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_60((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_114(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_61 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_115(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_62((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_116(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_63 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_117(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_64 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_118(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_65 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_119(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_184(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_48(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_120(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_49(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_121(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_50(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_122(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_51(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_123(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_52(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_124(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_53(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_125(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_185(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_54(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_126(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_55(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_127(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_56(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_128(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_57(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_129(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_58(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_130(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: () = (unsafe { construct_mapping_59(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_131(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_186(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_60((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_132(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_61 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_133(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_62((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_134(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_63 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_135(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_64 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_136(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_65 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_137(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_187(
    handle: *const *mut MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_48(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_138(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_49(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_139(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_50(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_140(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_51(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_141(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_52(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_142(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_53(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_143(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_188(
    handle: *const *mut MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_54(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_144(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_55(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_145(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_56(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_146(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_57(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_147(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_58(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_148(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_59(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_149(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_189(
    handle: *const *mut MinimalElementType,
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_60((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_150(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_61 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_151(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_62((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_152(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_63 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_153(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_64 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_154(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_65 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_155(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_190(
    handle: *const *const MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_48(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_156(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_49(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_157(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_50(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_158(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_51(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_159(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_52(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_160(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_53(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_161(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_191(
    handle: *const *const MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_54(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_162(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_55(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_163(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_56(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_164(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_57(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_165(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_58(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_166(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: () = (unsafe { construct_mapping_59(unimplemented!(), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_167(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_extents_192(
    handle: *const *const MinimalElementType,
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_175));
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_60((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_168(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_61 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_169(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_62((*layout), unimplemented!()) });
        let _acc: *const () = acc;
        test_mdspan_types_170(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_63 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_171(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_64 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_172(_handle, &mut _map, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _map : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_65 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _acc: *const () = acc;
        test_mdspan_types_173(_handle, &mut _map, _acc)
    });
}
pub unsafe fn mixin_layout_193(handle: *const *mut (), acc: *const ()) {
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_174(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_176(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_177(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_194(handle: *const *const (), acc: *const ()) {
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_178(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_179(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_180(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_195(handle: *const *mut (), acc: *const ()) {
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_181(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_182(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_183(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_196(handle: *const *const (), acc: *const ()) {
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_184(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_185(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_186(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_197(handle: *const *mut MinimalElementType, acc: *const ()) {
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_187(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_188(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_189(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_198(handle: *const *const MinimalElementType, acc: *const ()) {
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_190(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_191(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_192(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_accessor_199() {
    let mut elements: ElementPool_int__1024_ = ElementPool_int__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_int__1024_::destructor);
    (unsafe {
        let mut _handle: *mut () = (unsafe { ElementPool_int__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_193(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_200() {
    let mut elements: ElementPool_const_int__1024_ = ElementPool_const_int__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_const_int__1024_::destructor);
    (unsafe {
        let mut _handle: *const () =
            (unsafe { ElementPool_const_int__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_194(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_201() {
    let mut elements: ElementPool_double__1024_ = ElementPool_double__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_double__1024_::destructor);
    (unsafe {
        let mut _handle: *mut () = (unsafe { ElementPool_double__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_195(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_202() {
    let mut elements: ElementPool_const_double__1024_ = ElementPool_const_double__1024_::new();
    let _dtor_elements = ScopedDestructorUnsafe::new(
        &raw mut elements,
        ElementPool_const_double__1024_::destructor,
    );
    (unsafe {
        let mut _handle: *const () =
            (unsafe { ElementPool_const_double__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_196(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_203() {
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
        mixin_layout_197(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_204() {
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
        mixin_layout_198(&mut _handle, &mut _acc)
    });
}
pub unsafe fn test_205() -> () {
    (unsafe { mixin_accessor_199() });
    (unsafe { mixin_accessor_200() });
    (unsafe { mixin_accessor_201() });
    (unsafe { mixin_accessor_202() });
    (unsafe { mixin_accessor_203() });
    (unsafe { mixin_accessor_204() });
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
    (unsafe { test_205() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const Wrap_42);
    std::cell::LazyCell::force(&*&raw const Wrap_43);
    std::cell::LazyCell::force(&*&raw const Wrap_44);
    std::cell::LazyCell::force(&*&raw const Wrap_45);
    std::cell::LazyCell::force(&*&raw const Wrap_46);
    std::cell::LazyCell::force(&*&raw const Wrap_47);
}
