extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptrconst_ref__int_ptrconst_ref_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_extents_int__ref__const_std_extents_int__ref_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_bool__bool_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_layout_left_mapping_std_extents_int___ref__const_std_layout_left_mapping_std_extents_int___ref_
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_default_accessor_int__ref__const_std_default_accessor_int__ref_
{}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_extents_signed_char__18446744073709551615__ref__const_std_extents_signed_char__18446744073709551615__ref_
{}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_signed_char__signed_char_ {}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_unsigned_char__unsigned_char_ {}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_layout_left_mapping_std_extents_signed_char__18446744073709551615___ref__const_std_layout_left_mapping_std_extents_signed_char__18446744073709551615___ref_
{}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_extents_unsigned_int__7__ref__const_std_extents_unsigned_int__7__ref_
{}
pub type anon_12 = ();
pub const anon_12_value: anon_12 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_layout_left_mapping_std_extents_unsigned_int__7___ref__const_std_layout_left_mapping_std_extents_unsigned_int__7___ref_
{}
pub type anon_13 = ();
pub const anon_13_value: anon_13 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ref__const_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ref_
{}
pub type anon_14 = ();
pub const anon_14_value: anon_14 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_layout_left_mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___ref__const_std_layout_left_mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___ref_
{}
pub type anon_15 = ();
pub const anon_15_value: anon_15 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ref__const_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ref_
{}
pub type anon_16 = ();
pub const anon_16_value: anon_16 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_layout_left_mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615___ref__const_std_layout_left_mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615___ref_
{}
pub type anon_17 = ();
pub const anon_17_value: anon_17 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ref__const_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ref_
{}
pub type anon_18 = ();
pub const anon_18_value: anon_18 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long__long_ {}
pub type anon_19 = ();
pub const anon_19_value: anon_19 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_layout_left_mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___ref__const_std_layout_left_mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___ref_
{}
pub type anon_20 = ();
pub const anon_20_value: anon_20 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_layout_right_mapping_std_extents_int___ref__const_std_layout_right_mapping_std_extents_int___ref_
{}
pub type anon_21 = ();
pub const anon_21_value: anon_21 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_layout_right_mapping_std_extents_signed_char__18446744073709551615___ref__const_std_layout_right_mapping_std_extents_signed_char__18446744073709551615___ref_
{}
pub type anon_22 = ();
pub const anon_22_value: anon_22 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_layout_right_mapping_std_extents_unsigned_int__7___ref__const_std_layout_right_mapping_std_extents_unsigned_int__7___ref_
{}
pub type anon_23 = ();
pub const anon_23_value: anon_23 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_layout_right_mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___ref__const_std_layout_right_mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___ref_
{}
pub type anon_24 = ();
pub const anon_24_value: anon_24 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_layout_right_mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615___ref__const_std_layout_right_mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615___ref_
{}
pub type anon_25 = ();
pub const anon_25_value: anon_25 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_layout_right_mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___ref__const_std_layout_right_mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___ref_
{}
pub type anon_26 = ();
pub const anon_26_value: anon_26 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_layout_wrapping_integral_4__mapping_std_extents_int___ref__const_layout_wrapping_integral_4__mapping_std_extents_int___ref_
{}
pub type anon_27 = ();
pub const anon_27_value: anon_27 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615___ref__const_layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615___ref_
{}
pub type anon_28 = ();
pub const anon_28_value: anon_28 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7___ref__const_layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7___ref_
{}
pub type anon_29 = ();
pub const anon_29_value: anon_29 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___ref__const_layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615___ref_
{}
pub type anon_30 = ();
pub const anon_30_value: anon_30 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615___ref__const_layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615___ref_
{}
pub type anon_31 = ();
pub const anon_31_value: anon_31 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___ref__const_layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615___ref_
{}
pub type anon_32 = ();
pub const anon_32_value: anon_32 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_int_ptrconst_ref__const_int_ptrconst_ref_ {}
pub type anon_33 = ();
pub const anon_33_value: anon_33 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_default_accessor_const_int__ref__const_std_default_accessor_const_int__ref_
{}
pub type anon_34 = ();
pub const anon_34_value: anon_34 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_double_ptrconst_ref__double_ptrconst_ref_ {}
pub type anon_35 = ();
pub const anon_35_value: anon_35 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_default_accessor_double__ref__const_std_default_accessor_double__ref_
{}
pub type anon_36 = ();
pub const anon_36_value: anon_36 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_double_ptrconst_ref__const_double_ptrconst_ref_ {}
pub type anon_37 = ();
pub const anon_37_value: anon_37 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_default_accessor_const_double__ref__const_std_default_accessor_const_double__ref_
{}
pub type anon_38 = ();
pub const anon_38_value: anon_38 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_MinimalElementType_ptrconst_ref__MinimalElementType_ptrconst_ref_
{}
pub type anon_39 = ();
pub const anon_39_value: anon_39 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_default_accessor_MinimalElementType__ref__const_std_default_accessor_MinimalElementType__ref_
{}
pub type anon_40 = ();
pub const anon_40_value: anon_40 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_MinimalElementType_ptrconst_ref__const_MinimalElementType_ptrconst_ref_
{}
pub type anon_41 = ();
pub const anon_41_value: anon_41 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_default_accessor_const_MinimalElementType__ref__const_std_default_accessor_const_MinimalElementType__ref_
{}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(4)]
pub struct MinimalElementType {
    #[offset(0)]
    pub val: (),
}
impl MinimalElementType {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_int__1024_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl ElementPool_int__1024_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_int__1024_> = Rc::new(RefCell::new(Self {
            ptr_: Ptr::<()>::null(),
        }));
        let this: Ptr<ElementPool_int__1024_> = __this.as_pointer();
        field!(this, ptr_).write(unimplemented!());
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_int__1024_ {
    fn default() -> Self {
        { ElementPool_int__1024_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_const_int__1024_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl ElementPool_const_int__1024_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_const_int__1024_> = Rc::new(RefCell::new(Self {
            ptr_: Ptr::<()>::null(),
        }));
        let this: Ptr<ElementPool_const_int__1024_> = __this.as_pointer();
        field!(this, ptr_).write(unimplemented!());
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_const_int__1024_ {
    fn default() -> Self {
        { ElementPool_const_int__1024_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_double__1024_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl ElementPool_double__1024_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_double__1024_> = Rc::new(RefCell::new(Self {
            ptr_: Ptr::<()>::null(),
        }));
        let this: Ptr<ElementPool_double__1024_> = __this.as_pointer();
        field!(this, ptr_).write(unimplemented!());
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_double__1024_ {
    fn default() -> Self {
        { ElementPool_double__1024_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_const_double__1024_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl ElementPool_const_double__1024_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_const_double__1024_> = Rc::new(RefCell::new(Self {
            ptr_: Ptr::<()>::null(),
        }));
        let this: Ptr<ElementPool_const_double__1024_> = __this.as_pointer();
        field!(this, ptr_).write(unimplemented!());
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_const_double__1024_ {
    fn default() -> Self {
        { ElementPool_const_double__1024_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_MinimalElementType__1024_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<MinimalElementType>,
}
impl ElementPool_MinimalElementType__1024_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_MinimalElementType__1024_> = Rc::new(RefCell::new(Self {
            ptr_: Ptr::<MinimalElementType>::null(),
        }));
        let this: Ptr<ElementPool_MinimalElementType__1024_> = __this.as_pointer();
        field!(this, ptr_).write(unimplemented!());
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_MinimalElementType__1024_ {
    fn default() -> Self {
        { ElementPool_MinimalElementType__1024_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(8)]
pub struct ElementPool_const_MinimalElementType__1024_ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<MinimalElementType>,
}
impl ElementPool_const_MinimalElementType__1024_ {
    pub fn new() -> Self {
        let __this: Value<ElementPool_const_MinimalElementType__1024_> =
            Rc::new(RefCell::new(Self {
                ptr_: Ptr::<MinimalElementType>::null(),
            }));
        let this: Ptr<ElementPool_const_MinimalElementType__1024_> = __this.as_pointer();
        field!(this, ptr_).write(unimplemented!());
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for ElementPool_const_MinimalElementType__1024_ {
    fn default() -> Self {
        { ElementPool_const_MinimalElementType__1024_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct not_extents_constructible_tag {}
thread_local!(
    static Wrap_42: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4__mapping_std_extents_int__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_int__ {
    pub fn copy_from(other: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
    pub fn is_always_unique() -> () {
        return false;
    }
    pub fn is_always_exhaustive() -> () {
        return true;
    }
    pub fn is_always_strided() -> () {
        return false;
    }
    pub fn is_exhaustive() -> () {
        return true;
    }
}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_int__ {
    fn clone(&self) -> Self {
        let __src: Value<layout_wrapping_integral_4__mapping_std_extents_int__> = Rc::new(
            RefCell::new(layout_wrapping_integral_4__mapping_std_extents_int__ {
                extents_: self.extents_.clone(),
            }),
        );
        layout_wrapping_integral_4__mapping_std_extents_int__::copy_from(__src.as_pointer())
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_int__ {
    fn default() -> Self {
        {
            layout_wrapping_integral_4__mapping_std_extents_int__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_43: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    pub fn copy_from(
        other: Ptr<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        >,
    ) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
    pub fn is_always_unique() -> () {
        return false;
    }
    pub fn is_always_exhaustive() -> () {
        return true;
    }
    pub fn is_always_strided() -> () {
        return false;
    }
    pub fn is_exhaustive() -> () {
        return true;
    }
}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    fn clone(&self) -> Self {
        let __src: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
                extents_: self.extents_.clone(),
            },
        ));
        layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__::copy_from(__src.as_pointer())
    }
}
impl Default
    for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__
{
    fn default() -> Self {
        {
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_44: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    pub fn copy_from(
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    ) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
    pub fn is_always_unique() -> () {
        return false;
    }
    pub fn is_always_exhaustive() -> () {
        return true;
    }
    pub fn is_always_strided() -> () {
        return false;
    }
    pub fn is_exhaustive() -> () {
        return true;
    }
}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    fn clone(&self) -> Self {
        let __src: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
                    extents_: self.extents_.clone(),
                },
            ));
        layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::copy_from(
            __src.as_pointer(),
        )
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    fn default() -> Self {
        {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_45: Value<()> = Rc::new(RefCell::new((4_() as ())));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(16)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { pub fn copy_from ( other : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  > , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn is_always_unique ( ) -> ()  { return false   ;
 } pub fn is_always_exhaustive ( ) -> ()  { return true   ;
 } pub fn is_always_strided ( ) -> ()  { return false   ;
 } pub fn is_exhaustive ( ) -> ()  { return true   ;
 } }
impl Clone for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn clone(&self) -> Self { let __src: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__> = Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_: self.extents_.clone(), })); layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__::copy_from(__src.as_pointer()) } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_46: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(2)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { pub fn copy_from ( other : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  > , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn is_always_unique ( ) -> ()  { return false   ;
 } pub fn is_always_exhaustive ( ) -> ()  { return true   ;
 } pub fn is_always_strided ( ) -> ()  { return false   ;
 } pub fn is_exhaustive ( ) -> ()  { return true   ;
 } }
impl Clone for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn clone(&self) -> Self { let __src: Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__> = Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_: self.extents_.clone(), })); layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__::copy_from(__src.as_pointer()) } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_47: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(32)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { pub fn copy_from ( other : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  > , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn is_always_unique ( ) -> ()  { return false   ;
 } pub fn is_always_exhaustive ( ) -> ()  { return true   ;
 } pub fn is_always_strided ( ) -> ()  { return false   ;
 } pub fn is_exhaustive ( ) -> ()  { return true   ;
 } }
impl Clone for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn clone(&self) -> Self { let __src: Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__> = Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_: self.extents_.clone(), })); layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__::copy_from(__src.as_pointer()) } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4_ {}
pub fn construct_mapping_48(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_49(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_50(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_51(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_52(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_53(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_54(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_55(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_56(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_57(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_58(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_59(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_60(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_int__::new_1({ exts.as_pointer() }, {
        <not_extents_constructible_tag>::default()
    });
}
pub fn construct_mapping_61(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_62(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::new_1(
        { exts.as_pointer() },
        { <not_extents_constructible_tag>::default() },
    );
}pub fn construct_mapping_63 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_64 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_65 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_convertible_layout {}
pub fn test_mdspan_types_66(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_67(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_68(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_69(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_70(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_71(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_72(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_73(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_74(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_75(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_76(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_77(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_78(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_79(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_80(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
pub fn test_mdspan_types_81(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_82(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_83(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_84(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_85(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_86(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_87(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_88(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_89(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_90(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_91(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_92(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_93(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_94(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_95(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_96(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_97(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_98(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
pub fn test_mdspan_types_99(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_100(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_101(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_102(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_103(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_104(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_105(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_106(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_107(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_108(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_109(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_110(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_111(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_112(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_113(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_114(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_115(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_116(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
pub fn test_mdspan_types_117(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_118(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_119(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_120(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_121(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_122(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_123(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_124(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_125(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_126(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_127(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_128(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_129(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_130(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_131(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_132(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_133(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_134(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
pub fn test_mdspan_types_135(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_136(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_137(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_138(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_139(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_140(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_141(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_142(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_143(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_144(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_145(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_146(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_147(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_148(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_149(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_150(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_151(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_152(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
pub fn test_mdspan_types_153(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_154(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_155(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_156(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_157(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_158(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_159(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_160(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_161(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_162(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_163(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_164(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_165(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_166(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_167(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_168(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_169(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_170(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
pub fn test_mdspan_types_171(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_172(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
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
            'loop_: while (r < unimplemented!()) {
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
pub fn test_mdspan_types_173(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
        'loop_: while (r < unimplemented!()) {
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            const _: () = assert!(true, "");;
            const _: () = assert!(true, "");;
            unimplemented!();
            {
                let rhs_0 = (size).wrapping_mul(unimplemented!());
                size = rhs_0
            };
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
            'loop_: while (r < unimplemented!()) {
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
pub fn mixin_extents_174(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_48(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_66(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_49(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_67(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_50(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_68(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_51(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_69(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_52(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_70(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_53(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_71(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_176(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_54(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_72(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_55(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_73(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_56(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_74(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_57(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_75(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_58(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_76(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_59(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_77(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_177(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_60((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_78(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_61((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_79(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_62((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_80(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_63 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_81(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_64 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_82(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_65 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_83(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_178(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_48(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_84(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_49(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_85(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_50(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_86(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_51(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_87(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_52(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_88(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_53(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_89(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_179(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_54(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_90(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_55(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_91(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_56(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_92(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_57(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_93(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_58(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_94(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_59(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_95(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_180(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_60((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_96(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_61((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_97(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_62((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_98(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_63 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_99(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_64 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_100(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_65 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_101(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_181(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_48(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_102(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_49(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_103(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_50(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_104(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_51(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_105(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_52(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_106(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_53(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_107(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_182(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_54(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_108(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_55(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_109(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_56(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_110(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_57(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_111(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_58(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_112(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_59(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_113(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_183(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_60((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_114(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_61((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_115(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_62((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_116(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_63 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_117(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_64 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_118(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_65 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_119(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_184(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_48(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_120(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_49(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_121(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_50(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_122(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_51(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_123(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_52(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_124(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_53(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_125(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_185(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_54(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_126(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_55(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_127(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_56(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_128(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_57(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_129(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_58(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_130(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_59(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_131(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_186(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_60((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_132(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_61((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_133(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_62((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_134(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_63 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_135(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_64 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_136(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_65 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_137(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_187(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_48(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_138(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_49(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_139(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_50(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_140(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_51(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_141(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_52(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_142(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_53(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_143(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_188(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_54(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_144(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_55(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_145(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_56(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_146(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_57(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_147(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_58(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_148(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_59(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_149(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_189(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_60((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_150(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_61((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_151(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_62((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_152(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_63 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_153(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_64 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_154(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_65 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_155(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_190(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_48(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_156(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_49(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_157(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_50(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_158(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_51(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_159(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_52(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_160(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_53(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_161(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_191(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_54(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_162(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_55(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_163(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_56(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_164(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_57(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_165(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_58(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_166(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_59(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_167(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_192(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_175.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_60((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_168(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_61((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_169(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_62((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_170(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_63 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_171(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_64 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_172(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_65 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_173(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_layout_193(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_174(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_176(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_177(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_194(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_178(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_179(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_180(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_195(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_181(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_182(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_183(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_196(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_184(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_185(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_186(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_197(handle: Ptr<Ptr<MinimalElementType>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_187(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_188(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_189(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_198(handle: Ptr<Ptr<MinimalElementType>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_190(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_191(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_192(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_accessor_199() {
    let elements: Value<ElementPool_int__1024_> =
        Rc::new(RefCell::new(ElementPool_int__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_int__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_193(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_200() {
    let elements: Value<ElementPool_const_int__1024_> =
        Rc::new(RefCell::new(ElementPool_const_int__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_const_int__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_194(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_201() {
    let elements: Value<ElementPool_double__1024_> =
        Rc::new(RefCell::new(ElementPool_double__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_double__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_195(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_202() {
    let elements: Value<ElementPool_const_double__1024_> =
        Rc::new(RefCell::new(ElementPool_const_double__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_const_double__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_196(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_203() {
    let elements: Value<ElementPool_MinimalElementType__1024_> =
        Rc::new(RefCell::new(ElementPool_MinimalElementType__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<MinimalElementType>> = Rc::new(RefCell::new(
            ({ ElementPool_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_197(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_204() {
    let elements: Value<ElementPool_const_MinimalElementType__1024_> = Rc::new(RefCell::new(
        ElementPool_const_MinimalElementType__1024_::new(),
    ));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<MinimalElementType>> = Rc::new(RefCell::new(
            ({ ElementPool_const_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_198(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn test_205() -> () {
    ({ mixin_accessor_199() });
    ({ mixin_accessor_200() });
    ({ mixin_accessor_201() });
    ({ mixin_accessor_202() });
    ({ mixin_accessor_203() });
    ({ mixin_accessor_204() });
    return true;
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_205() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait ElementPool_MinimalElementType__1024_Impl {
    fn get_ptr(&self) -> Ptr<MinimalElementType>;
    fn destructor(&self);
}
impl ElementPool_MinimalElementType__1024_Impl for Ptr<ElementPool_MinimalElementType__1024_> {
    fn get_ptr(&self) -> Ptr<MinimalElementType> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait ElementPool_const_MinimalElementType__1024_Impl {
    fn get_ptr(&self) -> Ptr<MinimalElementType>;
    fn destructor(&self);
}
impl ElementPool_const_MinimalElementType__1024_Impl
    for Ptr<ElementPool_const_MinimalElementType__1024_>
{
    fn get_ptr(&self) -> Ptr<MinimalElementType> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait ElementPool_const_double__1024_Impl {
    fn get_ptr(&self) -> Ptr<()>;
    fn destructor(&self);
}
impl ElementPool_const_double__1024_Impl for Ptr<ElementPool_const_double__1024_> {
    fn get_ptr(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait ElementPool_const_int__1024_Impl {
    fn get_ptr(&self) -> Ptr<()>;
    fn destructor(&self);
}
impl ElementPool_const_int__1024_Impl for Ptr<ElementPool_const_int__1024_> {
    fn get_ptr(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait ElementPool_double__1024_Impl {
    fn get_ptr(&self) -> Ptr<()>;
    fn destructor(&self);
}
impl ElementPool_double__1024_Impl for Ptr<ElementPool_double__1024_> {
    fn get_ptr(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait ElementPool_int__1024_Impl {
    fn get_ptr(&self) -> Ptr<()>;
    fn destructor(&self);
}
impl ElementPool_int__1024_Impl for Ptr<ElementPool_int__1024_> {
    fn get_ptr(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr_.clone());
    }
    fn destructor(&self) {
        let mut i: () = 0_();
        'loop_: while (i != 1024_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
pub trait layout_wrapping_integral_4__mapping_std_extents_int__Impl {
    fn copy_assign(
        &self,
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_int__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> ();
    fn is_strided(&self) -> ();
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_int__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn is_unique(&self) -> () {
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() > Wrap_42.with(|rc| *rc.borrow())) {
                return false;
            }
            r.postfix_inc();
        }
        return true;
    }
    fn is_strided(&self) -> () {
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() > Wrap_42.with(|rc| *rc.borrow())) {
                return false;
            }
            r.postfix_inc();
        }
        return true;
    }
}
pub trait layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> ();
    fn is_strided(&self) -> ();
    fn stride(&self, r: ()) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl for Ptr<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn is_unique ( &self , ) -> ()  { let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { if ( unimplemented!()  > Wrap_47 .with(|rc| *rc.borrow()) ) { return false   ;
 } r  . postfix_inc () ;
 } return true   ;
 } fn is_strided ( &self , ) -> ()  { let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { if ( unimplemented!()  > Wrap_47 .with(|rc| *rc.borrow()) ) { return false   ;
 } r  . postfix_inc () ;
 } return true   ;
 } fn stride ( &self , mut r : ()  , ) -> ()  { let s : Value<() > = Rc::new(RefCell::new(1_() )) ;
  ;
 ;
 let mut i : ()  = ( ( unimplemented!()  as () ) ) . wrapping_sub ( 1_() )  ;
  ;
 ;
 'loop_: while ( i > r ) { (*s.borrow_mut())  *= unimplemented!()   ;
 i  . postfix_dec () ;
 } return (*s.borrow())   ;
 } }
pub trait layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__Impl {
    fn copy_assign(
        &self,
        other: Ptr<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        >,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>
    {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> ();
    fn is_strided(&self) -> ();
    fn stride(&self, r: ()) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn is_unique(&self) -> () {
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() > Wrap_43.with(|rc| *rc.borrow())) {
                return false;
            }
            r.postfix_inc();
        }
        return true;
    }
    fn is_strided(&self) -> () {
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() > Wrap_43.with(|rc| *rc.borrow())) {
                return false;
            }
            r.postfix_inc();
        }
        return true;
    }
    fn stride(&self, mut r: ()) -> () {
        let s: Value<()> = Rc::new(RefCell::new(1_()));
        let mut i: () = (unimplemented!() as ()).wrapping_sub(1_());
        'loop_: while (i > r) {
            (*s.borrow_mut()) *= unimplemented!();
            i.postfix_dec();
        }
        return (*s.borrow());
    }
}
pub trait layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> ();
    fn is_strided(&self) -> ();
    fn stride(&self, r: ()) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl for Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn is_unique ( &self , ) -> ()  { let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { if ( unimplemented!()  > Wrap_46 .with(|rc| *rc.borrow()) ) { return false   ;
 } r  . postfix_inc () ;
 } return true   ;
 } fn is_strided ( &self , ) -> ()  { let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { if ( unimplemented!()  > Wrap_46 .with(|rc| *rc.borrow()) ) { return false   ;
 } r  . postfix_inc () ;
 } return true   ;
 } fn stride ( &self , mut r : ()  , ) -> ()  { let s : Value<() > = Rc::new(RefCell::new(1_() )) ;
  ;
 ;
 let mut i : ()  = ( ( unimplemented!()  as () ) ) . wrapping_sub ( 1_() )  ;
  ;
 ;
 'loop_: while ( i > r ) { (*s.borrow_mut())  *= unimplemented!()   ;
 i  . postfix_dec () ;
 } return (*s.borrow())   ;
 } }
pub trait layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__Impl {
    fn copy_assign(
        &self,
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> ();
    fn is_strided(&self) -> ();
    fn stride(&self, r: ()) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn is_unique(&self) -> () {
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() > Wrap_44.with(|rc| *rc.borrow())) {
                return false;
            }
            r.postfix_inc();
        }
        return true;
    }
    fn is_strided(&self) -> () {
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() > Wrap_44.with(|rc| *rc.borrow())) {
                return false;
            }
            r.postfix_inc();
        }
        return true;
    }
    fn stride(&self, mut r: ()) -> () {
        let s: Value<()> = Rc::new(RefCell::new(1_()));
        let mut i: () = (unimplemented!() as ()).wrapping_sub(1_());
        'loop_: while (i > r) {
            {
                let rhs_0 = (*s.borrow()).wrapping_mul(unimplemented!());
                (*s.borrow_mut()) = rhs_0
            };
            i.postfix_dec();
        }
        return (*s.borrow());
    }
}
pub trait layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> ();
    fn is_strided(&self) -> ();
    fn stride(&self, r: ()) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl for Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn is_unique ( &self , ) -> ()  { let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { if ( unimplemented!()  > Wrap_45 .with(|rc| *rc.borrow()) ) { return false   ;
 } r  . postfix_inc () ;
 } return true   ;
 } fn is_strided ( &self , ) -> ()  { let mut r : ()  = 0_()  ;
  ;
 ;
 'loop_: while ( r < unimplemented!()  ) { if ( unimplemented!()  > Wrap_45 .with(|rc| *rc.borrow()) ) { return false   ;
 } r  . postfix_inc () ;
 } return true   ;
 } fn stride ( &self , mut r : ()  , ) -> ()  { let s : Value<() > = Rc::new(RefCell::new(1_() )) ;
  ;
 ;
 let mut i : ()  = ( ( unimplemented!()  as () ) ) . wrapping_sub ( 1_() )  ;
  ;
 ;
 'loop_: while ( i > r ) { { let rhs_0 = ( (*s.borrow())  ) . wrapping_mul ( unimplemented!()  )  ;
 (*s.borrow_mut())  = rhs_0 } ;
 i  . postfix_dec () ;
 } return (*s.borrow())   ;
 } }
pub fn __cpp2rust_init_globals() {
    let _ = Wrap_42.with(|_| ());
    let _ = Wrap_43.with(|_| ());
    let _ = Wrap_44.with(|_| ());
    let _ = Wrap_45.with(|_| ());
    let _ = Wrap_46.with(|_| ());
    let _ = Wrap_47.with(|_| ());
}
