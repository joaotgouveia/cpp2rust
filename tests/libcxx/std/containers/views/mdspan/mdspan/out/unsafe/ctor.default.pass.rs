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
static mut Wrap_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
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
static mut Wrap_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
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
static mut Wrap_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
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
static mut Wrap_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { (4_() as ()) });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_ : unimplemented!() , } } } }
static mut Wrap_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
#[repr(C)]
#[derive(FnPtrArg)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__
{
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { pub unsafe fn new_1 ( ext : *const () , mut _a1 : not_extents_constructible_tag , ) -> Self { let mut this = Self { extents_ : unimplemented!() , } ;
 this } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn default() -> Self { unsafe { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_ : unimplemented!() , } } } }
static mut Wrap_5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 4_() });
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
pub unsafe fn construct_mapping_6(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_7(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_8(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_9(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_10(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
pub unsafe fn construct_mapping_11(mut _a0: (), mut exts: ()) -> () {
    return unimplemented!();
}
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
pub unsafe fn construct_mapping_18(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__ {
    return layout_wrapping_integral_4__mapping_std_extents_int__::new_1({ &exts }, {
        <not_extents_constructible_tag>::default()
    });
}
pub unsafe fn construct_mapping_19(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}
pub unsafe fn construct_mapping_20(
    mut _a0: layout_wrapping_integral_4_,
    mut exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::new_1({ &exts }, {
        <not_extents_constructible_tag>::default()
    });
}pub unsafe fn construct_mapping_21 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}pub unsafe fn construct_mapping_22 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_1 ( { & exts } , { <not_extents_constructible_tag >::default() } , )   ;
}pub unsafe fn construct_mapping_23 ( mut _a0 : layout_wrapping_integral_4_ , mut exts : () , ) -> layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct move_counted_handle_const_double_ {
    pub ptr: *const (),
}
impl move_counted_handle_const_double_ {
    pub unsafe fn new_1(mut ptr_: *const ()) -> Self {
        let mut this = Self { ptr: ptr_ };
        this
    }
    pub unsafe fn operator_index(&self, mut i: ()) -> *const () {
        return &(*self.ptr.offset((i) as isize));
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
}
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
pub unsafe fn test_mdspan_types_24(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_25(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_26(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_27(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_28(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_29(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_30(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_31(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_32(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_33(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_34(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_35(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_36(
    _a0: *const *mut (),
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_37(
    _a0: *const *mut (),
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_38(
    _a0: *const *mut (),
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_39(
    _a0: *const *mut (),
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_40(
    _a0: *const *mut (),
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_41(
    _a0: *const *mut (),
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_42(
    _a0: *const move_counted_handle_int_,
    _a1: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_43(
    _a0: *const move_counted_handle_int_,
    _a1: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_44(
    _a0: *const move_counted_handle_int_,
    _a1: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_45(
    _a0: *const move_counted_handle_int_,
    _a1: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_46(
    _a0: *const move_counted_handle_int_,
    _a1: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_47(
    _a0: *const move_counted_handle_int_,
    _a1: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_48(
    _a0: *const move_counted_handle_int_,
    _a1: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_49(
    _a0: *const move_counted_handle_int_,
    _a1: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_50(
    _a0: *const move_counted_handle_int_,
    _a1: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_51(
    _a0: *const move_counted_handle_int_,
    _a1: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_52(
    _a0: *const move_counted_handle_int_,
    _a1: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_53(
    _a0: *const move_counted_handle_int_,
    _a1: *const (),
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_54(
    _a0: *const move_counted_handle_int_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_55(
    _a0: *const move_counted_handle_int_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_56(
    _a0: *const move_counted_handle_int_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_57(
    _a0: *const move_counted_handle_int_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_58(
    _a0: *const move_counted_handle_int_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_59(
    _a0: *const move_counted_handle_int_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_60(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_61(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_62(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_63(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_64(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_65(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_66(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_67(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_68(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_69(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_70(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_71(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_72(
    _a0: *const *const (),
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_73(
    _a0: *const *const (),
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_74(
    _a0: *const *const (),
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_75(
    _a0: *const *const (),
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_76(
    _a0: *const *const (),
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_77(
    _a0: *const *const (),
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_78(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_79(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_80(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_81(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_82(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_83(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_84(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_85(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_86(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_87(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_88(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_89(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const (),
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_90(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_91(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_92(
    _a0: *const move_counted_handle_const_int_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_93(
    _a0: *const move_counted_handle_const_int_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_94(
    _a0: *const move_counted_handle_const_int_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_95(
    _a0: *const move_counted_handle_const_int_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_int_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_96(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_97(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_98(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_99(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_100(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_101(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_102(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_103(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_104(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_105(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_106(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_107(_a0: *const *mut (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_108(
    _a0: *const *mut (),
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_109(
    _a0: *const *mut (),
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_110(
    _a0: *const *mut (),
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_111(
    _a0: *const *mut (),
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_112(
    _a0: *const *mut (),
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_113(
    _a0: *const *mut (),
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_114(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_115(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_116(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_117(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_118(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_119(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_120(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_121(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_122(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_123(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_124(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_125(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const (),
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_126(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_127(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_128(
    _a0: *const no_default_ctor_handle_double_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_129(
    _a0: *const no_default_ctor_handle_double_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_130(
    _a0: *const no_default_ctor_handle_double_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_131(
    _a0: *const no_default_ctor_handle_double_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_132(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_133(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_134(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_135(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_136(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_137(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_138(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_139(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_140(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_141(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_142(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_143(_a0: *const *const (), _a1: *const (), _a2: *const ()) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_144(
    _a0: *const *const (),
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_145(
    _a0: *const *const (),
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_146(
    _a0: *const *const (),
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_147(
    _a0: *const *const (),
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_148(
    _a0: *const *const (),
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_149(
    _a0: *const *const (),
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_150(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_151(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
        );;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_152(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_153(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
        );;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_154(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
        );;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_155(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
        );;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_156(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_157(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
        );;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_158(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_159(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
        );;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_160(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
        );;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_161(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const (),
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
        );;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub unsafe fn test_mdspan_types_162(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_163(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_164(
    _a0: *const move_counted_handle_const_double_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_165(
    _a0: *const move_counted_handle_const_double_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_166(
    _a0: *const move_counted_handle_const_double_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_167(
    _a0: *const move_counted_handle_const_double_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_double_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_168(
    _a0: *const *mut MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_169(
    _a0: *const *mut MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_170(
    _a0: *const *mut MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_171(
    _a0: *const *mut MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_172(
    _a0: *const *mut MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_173(
    _a0: *const *mut MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_174(
    _a0: *const *mut MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_175(
    _a0: *const *mut MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_176(
    _a0: *const *mut MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_177(
    _a0: *const *mut MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_178(
    _a0: *const *mut MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_179(
    _a0: *const *mut MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_180(
    _a0: *const *mut MinimalElementType,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_181(
    _a0: *const *mut MinimalElementType,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_182(
    _a0: *const *mut MinimalElementType,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_183(
    _a0: *const *mut MinimalElementType,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_184(
    _a0: *const *mut MinimalElementType,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_185(
    _a0: *const *mut MinimalElementType,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_186(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_187(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_188(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_189(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_190(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_191(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_192(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_193(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_194(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_195(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_196(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_197(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_198(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_199(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_200(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_201(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_202(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_203(
    _a0: *const move_counted_handle_MinimalElementType_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_204(
    _a0: *const *const MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_205(
    _a0: *const *const MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_206(
    _a0: *const *const MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_207(
    _a0: *const *const MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_208(
    _a0: *const *const MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_209(
    _a0: *const *const MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_210(
    _a0: *const *const MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_211(
    _a0: *const *const MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_212(
    _a0: *const *const MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_213(
    _a0: *const *const MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_214(
    _a0: *const *const MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_215(
    _a0: *const *const MinimalElementType,
    _a1: *const (),
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let mut m: () = unimplemented!();
        const _: () = assert!(
            true,
            "noexcept(MDS()) == (noexcept(H()) && noexcept(M()) && noexcept(A()))"
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
}
pub unsafe fn test_mdspan_types_216(
    _a0: *const *const MinimalElementType,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_217(
    _a0: *const *const MinimalElementType,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_218(
    _a0: *const *const MinimalElementType,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_219(
    _a0: *const *const MinimalElementType,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_220(
    _a0: *const *const MinimalElementType,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_221(
    _a0: *const *const MinimalElementType,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const (),
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_222(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_223(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_224(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_225(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_226(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_227(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_228(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_229(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_230(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_231(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_232(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_233(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const (),
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_234(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_int__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_235(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_236(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1: *const layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_237(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_238(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn test_mdspan_types_239(
    _a0: *const move_counted_handle_const_MinimalElementType_,
    _a1 : *const layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__,
    _a2: *const checked_accessor_const_MinimalElementType_,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub unsafe fn mixin_extents_240(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_6(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_24(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_7(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_25(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_8(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_26(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_27(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_28(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_29(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_242(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_30(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_31(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_32(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_33(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_34(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_35(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_243(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_18((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_36(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_19 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_37(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_20((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_38(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_21 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_39(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_22 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_40(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_23 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_41(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_244(
    handle: *const move_counted_handle_int_,
    layout: *const (),
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_6(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_42(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_7(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_43(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_8(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_44(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_45(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_46(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_47(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_245(
    handle: *const move_counted_handle_int_,
    layout: *const (),
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_48(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_49(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_50(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_51(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_52(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_53(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_246(
    handle: *const move_counted_handle_int_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_18((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_54(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_19 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_55(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_20((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_56(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_21 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_57(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_22 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_58(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_int_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_23 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_int_ = acc;
        test_mdspan_types_59(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_247(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_6(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_60(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_7(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_61(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_8(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_62(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_63(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_64(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_65(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_248(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_66(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_67(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_68(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_69(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_70(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_71(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_249(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_18((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_72(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_19 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_73(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_20((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_74(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_21 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_75(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_22 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_76(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_23 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_77(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_250(
    handle: *const move_counted_handle_const_int_,
    layout: *const (),
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_6(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_78(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_7(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_79(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_8(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_80(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_81(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_82(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_83(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_251(
    handle: *const move_counted_handle_const_int_,
    layout: *const (),
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_84(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_85(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_86(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_87(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_88(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_89(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_252(
    handle: *const move_counted_handle_const_int_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_const_int_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_18((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_90(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_19 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_91(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_20((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_92(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_21 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_93(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_22 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_94(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_int_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_23 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_int_ = acc;
        test_mdspan_types_95(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_253(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_6(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_96(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_7(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_97(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_8(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_98(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_99(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_100(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_101(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_254(handle: *const *mut (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_102(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_103(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_104(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_105(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_106(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_107(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_255(
    handle: *const *mut (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_18((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_108(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_19 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_109(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_20((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_110(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_21 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_111(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_22 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_112(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_23 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_113(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_256(
    handle: *const no_default_ctor_handle_double_,
    layout: *const (),
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_6(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_114(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_7(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_115(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_8(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_116(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_117(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_118(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_119(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_257(
    handle: *const no_default_ctor_handle_double_,
    layout: *const (),
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_120(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_121(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_122(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_123(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_124(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_125(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_258(
    handle: *const no_default_ctor_handle_double_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_18((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_126(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_19 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_127(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_20((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_128(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_21 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_129(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_22 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_130(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const no_default_ctor_handle_double_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_23 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_double_ = acc;
        test_mdspan_types_131(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_259(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_6(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_132(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_7(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_133(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_8(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_134(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_135(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_136(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_137(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_260(handle: *const *const (), layout: *const (), acc: *const ()) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_138(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_139(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_140(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_141(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_142(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_143(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_261(
    handle: *const *const (),
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_18((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_144(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_19 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_145(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_20((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_146(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_21 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_147(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_22 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_148(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const () = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_23 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_149(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_262(
    handle: *const move_counted_handle_const_double_,
    layout: *const (),
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_6(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_150(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_7(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_151(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_8(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_152(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_153(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_154(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_155(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_263(
    handle: *const move_counted_handle_const_double_,
    layout: *const (),
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_156(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_157(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_158(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_159(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_160(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_161(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_264(
    handle: *const move_counted_handle_const_double_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_const_double_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_18((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_162(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_19 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_163(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_20((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_164(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_21 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_165(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_22 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_166(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_double_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_23 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_double_ = acc;
        test_mdspan_types_167(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_265(
    handle: *const *mut MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_6(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_168(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_7(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_169(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_8(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_170(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_171(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_172(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_173(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_266(
    handle: *const *mut MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_174(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_175(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_176(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_177(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_178(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_179(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_267(
    handle: *const *mut MinimalElementType,
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_18((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_180(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_19 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_181(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_20((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_182(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_21 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_183(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_22 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_184(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *mut MinimalElementType = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_23 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_185(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_268(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_6(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_186(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_7(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_187(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_8(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_188(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_189(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_190(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_191(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_269(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_192(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_193(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_194(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_195(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_196(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_197(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_270(
    handle: *const move_counted_handle_MinimalElementType_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_18((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_198(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_19 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_199(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_20((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_200(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_21 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_201(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_22 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_202(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_23 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_MinimalElementType_ = acc;
        test_mdspan_types_203(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_271(
    handle: *const *const MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_6(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_204(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_7(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_205(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_8(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_206(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_207(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_208(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_209(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_272(
    handle: *const *const MinimalElementType,
    layout: *const (),
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_210(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_211(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_212(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_213(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_214(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_215(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_273(
    handle: *const *const MinimalElementType,
    layout: *const layout_wrapping_integral_4_,
    acc: *const (),
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_18((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_216(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_19 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_217(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_20((*layout), unimplemented!()) });
        let _arg2: *const () = acc;
        test_mdspan_types_218(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_21 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_219(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_22 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_220(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const *const MinimalElementType = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_23 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const () = acc;
        test_mdspan_types_221(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_274(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_6(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_222(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_7(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_223(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_8(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_224(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_9(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_225(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_10(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_226(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_11(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_227(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_275(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const (),
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_12(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_228(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_13(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_229(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_14(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_230(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_15(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_231(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_16(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_232(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: () = (unsafe { construct_mapping_17(unimplemented!(), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_233(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_extents_276(
    handle: *const move_counted_handle_const_MinimalElementType_,
    layout: *const layout_wrapping_integral_4_,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_int__ =
            (unsafe { construct_mapping_18((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_234(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__  = ( unsafe { construct_mapping_19 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_235(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ =
            (unsafe { construct_mapping_20((*layout), unimplemented!()) });
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_236(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  = ( unsafe { construct_mapping_21 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_237(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  = ( unsafe { construct_mapping_22 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_238(_arg0, &mut _arg1, _arg2)
    });
    (unsafe {
        let _arg0: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _arg1 : layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  = ( unsafe { construct_mapping_23 ( ( * layout ) , unimplemented!() , ) } )  ;
        let _arg2: *const checked_accessor_const_MinimalElementType_ = acc;
        test_mdspan_types_239(_arg0, &mut _arg1, _arg2)
    });
}
pub unsafe fn mixin_layout_277(handle: *const *mut (), acc: *const ()) {
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_240(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_242(_handle, &mut _layout, _acc)
    });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_243(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_278(
    handle: *const move_counted_handle_int_,
    acc: *const checked_accessor_int_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_int_ = acc;
        mixin_extents_244(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_int_ = acc;
        mixin_extents_245(_handle, &mut _layout, _acc)
    });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_int_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_int_ = acc;
        mixin_extents_246(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_279(handle: *const *const (), acc: *const ()) {
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_247(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_248(_handle, &mut _layout, _acc)
    });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_249(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_280(
    handle: *const move_counted_handle_const_int_,
    acc: *const checked_accessor_const_int_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_int_ = acc;
        mixin_extents_250(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_int_ = acc;
        mixin_extents_251(_handle, &mut _layout, _acc)
    });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_const_int_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_const_int_ = acc;
        mixin_extents_252(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_281(handle: *const *mut (), acc: *const ()) {
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_253(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_254(_handle, &mut _layout, _acc)
    });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    (unsafe {
        let _handle: *const *mut () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_255(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_282(
    handle: *const no_default_ctor_handle_double_,
    acc: *const checked_accessor_double_,
) {
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_double_ = acc;
        mixin_extents_256(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_double_ = acc;
        mixin_extents_257(_handle, &mut _layout, _acc)
    });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    (unsafe {
        let _handle: *const no_default_ctor_handle_double_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_double_ = acc;
        mixin_extents_258(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_283(handle: *const *const (), acc: *const ()) {
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
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    (unsafe {
        let _handle: *const *const () = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_261(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_284(
    handle: *const move_counted_handle_const_double_,
    acc: *const checked_accessor_const_double_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_double_ = acc;
        mixin_extents_262(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_double_ = acc;
        mixin_extents_263(_handle, &mut _layout, _acc)
    });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_const_double_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_const_double_ = acc;
        mixin_extents_264(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_285(handle: *const *mut MinimalElementType, acc: *const ()) {
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_265(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_266(_handle, &mut _layout, _acc)
    });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    (unsafe {
        let _handle: *const *mut MinimalElementType = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_267(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_286(
    handle: *const move_counted_handle_MinimalElementType_,
    acc: *const checked_accessor_MinimalElementType_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        mixin_extents_268(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        mixin_extents_269(_handle, &mut _layout, _acc)
    });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_MinimalElementType_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_MinimalElementType_ = acc;
        mixin_extents_270(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_287(handle: *const *const MinimalElementType, acc: *const ()) {
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_271(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const () = acc;
        mixin_extents_272(_handle, &mut _layout, _acc)
    });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    (unsafe {
        let _handle: *const *const MinimalElementType = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const () = acc;
        mixin_extents_273(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_layout_288(
    handle: *const move_counted_handle_const_MinimalElementType_,
    acc: *const checked_accessor_const_MinimalElementType_,
) {
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        mixin_extents_274(_handle, &mut _layout, _acc)
    });
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: () = unimplemented!();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        mixin_extents_275(_handle, &mut _layout, _acc)
    });
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_241));
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    (unsafe {
        let _handle: *const move_counted_handle_const_MinimalElementType_ = handle;
        let mut _layout: layout_wrapping_integral_4_ = <layout_wrapping_integral_4_>::default();
        let _acc: *const checked_accessor_const_MinimalElementType_ = acc;
        mixin_extents_276(_handle, &mut _layout, _acc)
    });
}
pub unsafe fn mixin_accessor_289() {
    let mut elements: ElementPool_int__1024_ = ElementPool_int__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_int__1024_::destructor);
    (unsafe {
        let mut _handle: *mut () = (unsafe { ElementPool_int__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_277(&mut _handle, &mut _acc)
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<typename checked_accessor<T>::data_handle_type> != std::is_same_v<T, double>"
    );;
    (unsafe {
        let mut _handle: move_counted_handle_int_ = move_counted_handle_int_::new_1({
            (unsafe { ElementPool_int__1024_::get_ptr(&mut elements) })
        });
        let mut _acc: checked_accessor_int_ = checked_accessor_int_::new({ 1024_() });
        mixin_layout_278(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_290() {
    let mut elements: ElementPool_const_int__1024_ = ElementPool_const_int__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_const_int__1024_::destructor);
    (unsafe {
        let mut _handle: *const () =
            (unsafe { ElementPool_const_int__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_279(&mut _handle, &mut _acc)
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<typename checked_accessor<T>::data_handle_type> != std::is_same_v<T, double>"
    );;
    (unsafe {
        let mut _handle: move_counted_handle_const_int_ = move_counted_handle_const_int_::new_1({
            (unsafe { ElementPool_const_int__1024_::get_ptr(&mut elements) })
        });
        let mut _acc: checked_accessor_const_int_ = checked_accessor_const_int_::new({ 1024_() });
        mixin_layout_280(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_291() {
    let mut elements: ElementPool_double__1024_ = ElementPool_double__1024_::new();
    let _dtor_elements =
        ScopedDestructorUnsafe::new(&raw mut elements, ElementPool_double__1024_::destructor);
    (unsafe {
        let mut _handle: *mut () = (unsafe { ElementPool_double__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_281(&mut _handle, &mut _acc)
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<typename checked_accessor<T>::data_handle_type> != std::is_same_v<T, double>"
    );;
    (unsafe {
        let mut _handle: no_default_ctor_handle_double_ = no_default_ctor_handle_double_::new_1({
            (unsafe { ElementPool_double__1024_::get_ptr(&mut elements) })
        });
        let mut _acc: checked_accessor_double_ = checked_accessor_double_::new({ 1024_() });
        mixin_layout_282(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_292() {
    let mut elements: ElementPool_const_double__1024_ = ElementPool_const_double__1024_::new();
    let _dtor_elements = ScopedDestructorUnsafe::new(
        &raw mut elements,
        ElementPool_const_double__1024_::destructor,
    );
    (unsafe {
        let mut _handle: *const () =
            (unsafe { ElementPool_const_double__1024_::get_ptr(&mut elements) });
        let mut _acc: () = unimplemented!();
        mixin_layout_283(&mut _handle, &mut _acc)
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<typename checked_accessor<T>::data_handle_type> != std::is_same_v<T, double>"
    );;
    (unsafe {
        let mut _handle: move_counted_handle_const_double_ =
            move_counted_handle_const_double_::new_1({
                (unsafe { ElementPool_const_double__1024_::get_ptr(&mut elements) })
            });
        let mut _acc: checked_accessor_const_double_ =
            checked_accessor_const_double_::new_2({ 1024_() });
        mixin_layout_284(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_293() {
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
        mixin_layout_285(&mut _handle, &mut _acc)
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<typename checked_accessor<T>::data_handle_type> != std::is_same_v<T, double>"
    );;
    (unsafe {
        let mut _handle: move_counted_handle_MinimalElementType_ =
            move_counted_handle_MinimalElementType_::new_1({
                (unsafe { ElementPool_MinimalElementType__1024_::get_ptr(&mut elements) })
            });
        let mut _acc: checked_accessor_MinimalElementType_ =
            checked_accessor_MinimalElementType_::new({ 1024_() });
        mixin_layout_286(&mut _handle, &mut _acc)
    });
}
pub unsafe fn mixin_accessor_294() {
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
        mixin_layout_287(&mut _handle, &mut _acc)
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<typename checked_accessor<T>::data_handle_type> != std::is_same_v<T, double>"
    );;
    (unsafe {
        let mut _handle: move_counted_handle_const_MinimalElementType_ =
            move_counted_handle_const_MinimalElementType_::new_1({
                (unsafe { ElementPool_const_MinimalElementType__1024_::get_ptr(&mut elements) })
            });
        let mut _acc: checked_accessor_const_MinimalElementType_ =
            checked_accessor_const_MinimalElementType_::new({ 1024_() });
        mixin_layout_288(&mut _handle, &mut _acc)
    });
}
pub unsafe fn test_295() -> () {
    (unsafe { mixin_accessor_289() });
    (unsafe { mixin_accessor_290() });
    (unsafe { mixin_accessor_291() });
    (unsafe { mixin_accessor_292() });
    (unsafe { mixin_accessor_293() });
    (unsafe { mixin_accessor_294() });
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
    (unsafe { test_295() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const Wrap_0);
    std::cell::LazyCell::force(&*&raw const Wrap_1);
    std::cell::LazyCell::force(&*&raw const Wrap_2);
    std::cell::LazyCell::force(&*&raw const Wrap_3);
    std::cell::LazyCell::force(&*&raw const Wrap_4);
    std::cell::LazyCell::force(&*&raw const Wrap_5);
}
