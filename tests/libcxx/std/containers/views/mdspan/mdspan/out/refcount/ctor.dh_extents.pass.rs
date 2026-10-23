extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
        field!(this, ptr_).write((unimplemented!()).reinterpret_cast::<()>());
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
        field!(this, ptr_).write((unimplemented!()).reinterpret_cast::<()>());
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
        field!(this, ptr_).write((unimplemented!()).reinterpret_cast::<()>());
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
        field!(this, ptr_).write((unimplemented!()).reinterpret_cast::<()>());
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
    static Wrap_0: Value<()> = Rc::new(RefCell::new(8_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_8__mapping_std_extents_int__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_int__ {
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
}
impl Default for layout_wrapping_integral_8__mapping_std_extents_int__ {
    fn default() -> Self {
        {
            layout_wrapping_integral_8__mapping_std_extents_int__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_1: Value<()> = Rc::new(RefCell::new(8_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ {
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
}
impl Default
    for layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__
{
    fn default() -> Self {
        {
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_2: Value<()> = Rc::new(RefCell::new(8_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
}
impl Default for layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    fn default() -> Self {
        {
            layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_3: Value<()> = Rc::new(RefCell::new((8_() as ())));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(16)]
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl Default for layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_4: Value<()> = Rc::new(RefCell::new(8_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(2)]
pub struct layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(2)]
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl Default for layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_5: Value<()> = Rc::new(RefCell::new(8_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(32)]
pub struct layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(32)]
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl Default for layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct layout_wrapping_integral_8_ {}
thread_local!(
    static Wrap_6: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4__mapping_std_extents_int__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_int__ {
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
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
    static Wrap_7: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
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
    static Wrap_8: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    pub fn new_1(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
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
    static Wrap_9: Value<()> = Rc::new(RefCell::new((4_() as ())));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(16)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_10: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(2)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(2)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_11: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(32)]
pub struct layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(32)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4_ {}
pub fn construct_mapping_12(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_13(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_14(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_15(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_16(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_17(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_18(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_19(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_20(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_21(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_22(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_23(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_24(
    mut _a0: layout_wrapping_integral_8_,
    exts: (),
) -> layout_wrapping_integral_8__mapping_std_extents_int__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_8__mapping_std_extents_int__::new_1({ exts.as_pointer() }, {
        <not_extents_constructible_tag>::default()
    });
}
pub fn construct_mapping_25(
    mut _a0: layout_wrapping_integral_8_,
    exts: (),
) -> layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_26(
    mut _a0: layout_wrapping_integral_8_,
    exts: (),
) -> layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__::new_1(
        { exts.as_pointer() },
        { <not_extents_constructible_tag>::default() },
    );
}pub fn construct_mapping_27 ( mut _a0 : layout_wrapping_integral_8_  , exts : ()  , ) -> layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_28 ( mut _a0 : layout_wrapping_integral_8_  , exts : ()  , ) -> layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_29 ( mut _a0 : layout_wrapping_integral_8_  , exts : ()  , ) -> layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_30(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_int__::new_1({ exts.as_pointer() }, {
        <not_extents_constructible_tag>::default()
    });
}
pub fn construct_mapping_31(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_32(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::new_1(
        { exts.as_pointer() },
        { <not_extents_constructible_tag>::default() },
    );
}pub fn construct_mapping_33 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_34 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_35 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_convertible_layout {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct no_default_ctor_handle_double_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl no_default_ctor_handle_double_ {
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct not_const_convertible_handle_unsigned_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl not_const_convertible_handle_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct not_const_convertible_handle_const_unsigned_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl not_const_convertible_handle_const_unsigned_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_const_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl move_counted_handle_const_int_ {
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
    pub fn move_counter() -> Ptr<()> {
        return c_36.with(|v| v.as_pointer());
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl move_counted_handle_int_ {
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
    pub fn move_counter() -> Ptr<()> {
        return c_37.with(|v| v.as_pointer());
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_const_float_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl move_counted_handle_const_float_ {
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct move_counted_handle_const_double_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl move_counted_handle_const_double_ {
    pub fn move_from(other: Ptr<move_counted_handle_const_double_>) -> Self {
        let __this: Value<move_counted_handle_const_double_> = Rc::new(RefCell::new(Self {
            ptr: Ptr::<()>::null(),
        }));
        let this: Ptr<move_counted_handle_const_double_> = __this.as_pointer();
        field!(this, ptr).write({ other.with(|__s| __s.ptr.clone()) });
        {
            ({ move_counted_handle_const_double_::move_counter() })
                .with_mut(|__v| __v.postfix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
    pub fn move_counter() -> Ptr<()> {
        return c_38.with(|v| v.as_pointer());
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_double_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl move_counted_handle_double_ {
    pub fn move_counter() -> Ptr<()> {
        return c_39.with(|v| v.as_pointer());
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_MinimalElementType_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<MinimalElementType>,
}
impl move_counted_handle_MinimalElementType_ {
    pub fn new_1(mut ptr_: Ptr<MinimalElementType>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
    pub fn move_counter() -> Ptr<()> {
        return c_40.with(|v| v.as_pointer());
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_const_MinimalElementType_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<MinimalElementType>,
}
impl move_counted_handle_const_MinimalElementType_ {
    pub fn new_1(mut ptr_: Ptr<MinimalElementType>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
    pub fn move_counter() -> Ptr<()> {
        return c_41.with(|v| v.as_pointer());
    }
}
thread_local!(
    static c_36: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_37: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_38: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_39: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_40: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_41: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct checked_accessor_const_int_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_const_int_ {
    pub fn new(mut N_: ()) -> Self {
        Self { N: N_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct checked_accessor_int_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_int_ {
    pub fn new(mut N_: ()) -> Self {
        Self { N: N_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct checked_accessor_double_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_double_ {
    pub fn new(mut N_: ()) -> Self {
        Self { N: N_ }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(8)]
pub struct checked_accessor_unsigned_int_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_unsigned_int_ {
    pub fn new_1() -> Self {
        Self { N: 0_() }
    }
    pub fn new_2(mut N_: ()) -> Self {
        Self { N: N_ }
    }
    pub fn copy_from(acc: Ptr<checked_accessor_unsigned_int_>) -> Self {
        Self {
            N: acc.with(|__s| __s.N),
        }
    }
}
impl Clone for checked_accessor_unsigned_int_ {
    fn clone(&self) -> Self {
        let __src: Value<checked_accessor_unsigned_int_> =
            Rc::new(RefCell::new(checked_accessor_unsigned_int_ {
                N: self.N.clone(),
            }));
        checked_accessor_unsigned_int_::copy_from(__src.as_pointer())
    }
}
impl Default for checked_accessor_unsigned_int_ {
    fn default() -> Self {
        { checked_accessor_unsigned_int_::new_1() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(8)]
pub struct checked_accessor_const_unsigned_int_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_const_unsigned_int_ {
    pub fn new_1() -> Self {
        Self { N: 0_() }
    }
    pub fn new_2(mut N_: ()) -> Self {
        Self { N: N_ }
    }
    pub fn copy_from(acc: Ptr<checked_accessor_const_unsigned_int_>) -> Self {
        Self {
            N: acc.with(|__s| __s.N),
        }
    }
}
impl Clone for checked_accessor_const_unsigned_int_ {
    fn clone(&self) -> Self {
        let __src: Value<checked_accessor_const_unsigned_int_> =
            Rc::new(RefCell::new(checked_accessor_const_unsigned_int_ {
                N: self.N.clone(),
            }));
        checked_accessor_const_unsigned_int_::copy_from(__src.as_pointer())
    }
}
impl Default for checked_accessor_const_unsigned_int_ {
    fn default() -> Self {
        { checked_accessor_const_unsigned_int_::new_1() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(8)]
pub struct checked_accessor_const_float_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_const_float_ {
    pub fn new_1() -> Self {
        Self { N: 0_() }
    }
    pub fn new_2(mut N_: ()) -> Self {
        Self { N: N_ }
    }
    pub fn copy_from(acc: Ptr<checked_accessor_const_float_>) -> Self {
        Self {
            N: acc.with(|__s| __s.N),
        }
    }
    pub fn new_3(acc: Ptr<checked_accessor_float_>) -> Self {
        Self {
            N: acc.with(|__s| __s.N),
        }
    }
}
impl Clone for checked_accessor_const_float_ {
    fn clone(&self) -> Self {
        let __src: Value<checked_accessor_const_float_> =
            Rc::new(RefCell::new(checked_accessor_const_float_ {
                N: self.N.clone(),
            }));
        checked_accessor_const_float_::copy_from(__src.as_pointer())
    }
}
impl Default for checked_accessor_const_float_ {
    fn default() -> Self {
        { checked_accessor_const_float_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct checked_accessor_float_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_float_ {}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(8)]
pub struct checked_accessor_const_double_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_const_double_ {
    pub fn new_1() -> Self {
        Self { N: 0_() }
    }
    pub fn new_2(mut N_: ()) -> Self {
        Self { N: N_ }
    }
    pub fn copy_from(acc: Ptr<checked_accessor_const_double_>) -> Self {
        Self {
            N: acc.with(|__s| __s.N),
        }
    }
}
impl Clone for checked_accessor_const_double_ {
    fn clone(&self) -> Self {
        let __src: Value<checked_accessor_const_double_> =
            Rc::new(RefCell::new(checked_accessor_const_double_ {
                N: self.N.clone(),
            }));
        checked_accessor_const_double_::copy_from(__src.as_pointer())
    }
}
impl Default for checked_accessor_const_double_ {
    fn default() -> Self {
        { checked_accessor_const_double_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct checked_accessor_MinimalElementType_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_MinimalElementType_ {
    pub fn new(mut N_: ()) -> Self {
        Self { N: N_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct checked_accessor_const_MinimalElementType_ {
    #[offset(0)]
    pub N: (),
}
impl checked_accessor_const_MinimalElementType_ {
    pub fn new(mut N_: ()) -> Self {
        Self { N: N_ }
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
pub fn test_mdspan_types_42(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_43(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_44(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_45(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_46(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_47(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_48(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_49(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_50(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_51(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_52(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_53(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_54(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_55(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_56(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_57(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_58(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_59(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_60(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_61(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_62(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_63(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_64(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_65(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_66(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_67(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_68(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_69(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_70(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_71(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_72(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_73(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_74(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_75(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_76(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_77(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_78(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_79(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_80(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_81(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_82(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_83(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_84(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_85(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_86(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_87(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_88(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_89(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_90(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_91(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_92(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_93(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_94(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_95(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_96(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_97(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_98(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_99(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_100(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_101(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_int_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_102(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_103(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_104(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_105(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_106(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_107(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_108(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_109(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_110(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_111(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_112(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_113(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_114(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_115(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_116(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_117(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_118(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_119(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_120(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_121(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_122(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_123(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_124(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_125(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_126(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_127(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_128(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_129(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_130(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_131(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_132(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_133(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_134(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_135(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_136(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_137(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_138(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_139(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_140(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_141(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_142(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_143(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_144(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_145(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_146(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_147(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_148(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_149(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_150(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_151(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_152(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_153(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_154(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_155(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_156(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_157(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_158(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_159(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_160(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_161(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_162(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_163(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_164(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_165(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_166(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_167(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_168(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_169(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_170(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_171(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_172(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_173(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_174(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_175(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_176(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_177(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_178(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_179(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_180(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_181(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_182(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_183(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_184(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_185(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_186(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_187(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_188(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_189(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_190(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_191(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_192(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_193(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_194(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_195(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_196(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_197(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_198(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_199(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_200(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_201(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_202(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_203(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_204(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_205(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_206(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_207(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_208(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_209(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_210(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_211(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_212(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_213(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_214(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_215(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_216(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_217(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_218(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_219(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_220(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_221(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_double_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {
                unimplemented!();
            }
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {}
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_222(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_223(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_224(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_225(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_226(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_227(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_228(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_229(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_230(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_231(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_232(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_233(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_234(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_235(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_236(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_237(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_238(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_239(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_240(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_241(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_242(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_243(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_244(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_245(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_246(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_247(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_248(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_249(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_250(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_251(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_252(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_253(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_254(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_255(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_256(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_257(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_258(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_259(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_260(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_261(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_262(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_263(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_264(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_265(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_266(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_267(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_268(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_269(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_270(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_271(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_272(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_273(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_274(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_275(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_276(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_277(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_278(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_279(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_280(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_281(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_282(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_283(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_284(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_285(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_286(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_287(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_288(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_289(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_290(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_291(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_292(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_293(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
        }
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            {}
        }
        const _: () = assert!(true, "!noexcept(MDS(handle, map.extents()))");;
        unimplemented!();
        {
            unimplemented!();
        }
        {
            unimplemented!();
        }
        {}
    }
}
pub fn test_mdspan_types_294(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_295(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_296(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_297(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_298(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_299(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_300(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_301(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_302(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_303(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_304(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_305(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_306(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_307(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_308(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_309(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_310(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_311(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_312(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_313(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_314(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_315(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_316(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_317(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_318(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_319(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_320(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_321(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_322(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_323(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_324(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_325(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_326(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_327(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_328(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn test_mdspan_types_329(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, const typename M::extents_type&>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<MDS, const H&, const typename M::extents_type&>"
        );;
    }
}
pub fn mixin_extents_330(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_42(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_43(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_44(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_45(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_46(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_47(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_332(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_48(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_49(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_50(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_51(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_52(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_53(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_333(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_54(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_55(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_56(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_57(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_58(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_59(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_334(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_60(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_61(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_62(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_63(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_64(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_65(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_335(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_66(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_67(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_68(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_69(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_70(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_71(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_336(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_72(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_73(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_74(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_75(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_76(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_77(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_337(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_78(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_79(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_80(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_81(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_82(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_83(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_338(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_84(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_85(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_86(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_87(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_88(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_89(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_339(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_90(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_91(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_92(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_93(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_94(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_95(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_340(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_96(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_97(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_98(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_99(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_100(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_101(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_341(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_102(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_103(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_104(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_105(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_106(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_107(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_342(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_108(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_109(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_110(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_111(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_112(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_113(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_343(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_114(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_115(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_116(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_117(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_118(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_119(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_344(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_120(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_121(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_122(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_123(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_124(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_125(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_345(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_126(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_127(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_128(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_129(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_130(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_131(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_346(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_132(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_133(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_134(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_135(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_136(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_137(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_347(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_138(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_139(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_140(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_141(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_142(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_143(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_348(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_144(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_145(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_146(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_147(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_148(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_149(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_349(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_150(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_151(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_152(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_153(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_154(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_155(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_350(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_156(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_157(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_158(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_159(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_160(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_161(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_351(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_162(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_163(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_164(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_165(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_166(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_167(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_352(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_168(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_169(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_170(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_171(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_172(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_173(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_353(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_174(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_175(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_176(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_177(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_178(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_179(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_354(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_180(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_181(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_182(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_183(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_184(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_185(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_355(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_186(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_187(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_188(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_189(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_190(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_191(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_356(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_192(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_193(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_194(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_195(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_196(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_197(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_357(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_198(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_199(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_200(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_201(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_202(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_203(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_358(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_204(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_205(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_206(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_207(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_208(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_209(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_359(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_210(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_211(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_212(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_213(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_214(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_215(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_360(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_216(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_217(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_218(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_219(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_220(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_221(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_361(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_222(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_223(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_224(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_225(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_226(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_227(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_362(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_228(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_229(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_230(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_231(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_232(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_233(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_363(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_234(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_235(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_236(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_237(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_238(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_239(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_364(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_240(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_241(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_242(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_243(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_244(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_245(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_365(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_246(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_247(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_248(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_249(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_250(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_251(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_366(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_252(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_253(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_254(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_255(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_256(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_257(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_367(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_258(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_259(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_260(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_261(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_262(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_263(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_368(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_264(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_265(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_266(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_267(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_268(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_269(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_369(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_270(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_271(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_272(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_273(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_274(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_275(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_370(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_276(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_277(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_278(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_279(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_280(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_281(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_371(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_282(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_283(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_284(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_285(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_286(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_287(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_372(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_288(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_289(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_290(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_291(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_292(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_293(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_373(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_294(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_295(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_296(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_297(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_298(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_299(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_374(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_300(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_301(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_302(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_303(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_304(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_305(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_375(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_306(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_307(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_308(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_309(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_310(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_311(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_376(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_312(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_313(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_314(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_315(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_316(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_317(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_377(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_318(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_319(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_320(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_321(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_322(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_323(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_378(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_324(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_325(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_326(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_327(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_328(_handle, _map.as_pointer(), _arg2)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_329(_handle, _map.as_pointer(), _arg2)
    });
}
pub fn mixin_layout_379(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_330(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_332(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_8_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_8_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_333(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_334(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_380(handle: Ptr<move_counted_handle_int_>, acc: Ptr<checked_accessor_int_>) {
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_335(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_336(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_8_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_8_>::default()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_337(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_338(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_381(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_339(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_340(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_8_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_8_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_341(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_342(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_382(
    handle: Ptr<move_counted_handle_const_int_>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_343(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_344(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_8_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_8_>::default()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_345(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_346(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_383(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_347(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_348(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_8_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_8_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_349(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_350(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_384(
    handle: Ptr<no_default_ctor_handle_double_>,
    acc: Ptr<checked_accessor_double_>,
) {
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_351(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_352(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_8_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_8_>::default()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_353(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_354(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_385(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_355(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_356(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_8_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_8_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_357(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_358(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_386(
    handle: Ptr<move_counted_handle_const_double_>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_359(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_360(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_8_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_8_>::default()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_361(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_362(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_387(handle: Ptr<Ptr<MinimalElementType>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_363(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_364(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_8_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_8_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_365(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_366(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_388(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_367(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_368(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_8_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_8_>::default()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_369(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_370(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_389(handle: Ptr<Ptr<MinimalElementType>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_371(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_372(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_8_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_8_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_373(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_374(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_390(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_375(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_376(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<8>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_8_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_8_>::default()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_377(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, std::extents<int>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<layout_wrapping_integral<4>::mapping<std::extents<int>>, const std::extents<int>&>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_378(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_accessor_391() {
    let elements: Value<ElementPool_int__1024_> =
        Rc::new(RefCell::new(ElementPool_int__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_int__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_379(_handle.as_pointer(), _acc.as_pointer())
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_int_> =
            Rc::new(RefCell::new(move_counted_handle_int_::new_1({
                ({ ElementPool_int__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        let _acc: Value<checked_accessor_int_> =
            Rc::new(RefCell::new(checked_accessor_int_::new({ 1024_() })));
        mixin_layout_380(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_392() {
    let elements: Value<ElementPool_const_int__1024_> =
        Rc::new(RefCell::new(ElementPool_const_int__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_const_int__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_381(_handle.as_pointer(), _acc.as_pointer())
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_const_int_> =
            Rc::new(RefCell::new(move_counted_handle_const_int_::new_1({
                ({ ElementPool_const_int__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        let _acc: Value<checked_accessor_const_int_> =
            Rc::new(RefCell::new(checked_accessor_const_int_::new({ 1024_() })));
        mixin_layout_382(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_393() {
    let elements: Value<ElementPool_double__1024_> =
        Rc::new(RefCell::new(ElementPool_double__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_double__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_383(_handle.as_pointer(), _acc.as_pointer())
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<no_default_ctor_handle_double_> =
            Rc::new(RefCell::new(no_default_ctor_handle_double_::new_1({
                ({ ElementPool_double__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        let _acc: Value<checked_accessor_double_> =
            Rc::new(RefCell::new(checked_accessor_double_::new({ 1024_() })));
        mixin_layout_384(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_394() {
    let elements: Value<ElementPool_const_double__1024_> =
        Rc::new(RefCell::new(ElementPool_const_double__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_const_double__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_385(_handle.as_pointer(), _acc.as_pointer())
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_const_double_> =
            Rc::new(RefCell::new(move_counted_handle_const_double_::new_1({
                ({ ElementPool_const_double__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        let _acc: Value<checked_accessor_const_double_> =
            Rc::new(RefCell::new(checked_accessor_const_double_::new_2({
                1024_()
            })));
        mixin_layout_386(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_395() {
    let elements: Value<ElementPool_MinimalElementType__1024_> =
        Rc::new(RefCell::new(ElementPool_MinimalElementType__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<MinimalElementType>> = Rc::new(RefCell::new(
            ({ ElementPool_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_387(_handle.as_pointer(), _acc.as_pointer())
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_MinimalElementType_> = Rc::new(RefCell::new(
            move_counted_handle_MinimalElementType_::new_1({
                ({ ElementPool_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) })
            }),
        ));
        let _acc: Value<checked_accessor_MinimalElementType_> =
            Rc::new(RefCell::new(checked_accessor_MinimalElementType_::new({
                1024_()
            })));
        mixin_layout_388(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_396() {
    let elements: Value<ElementPool_const_MinimalElementType__1024_> = Rc::new(RefCell::new(
        ElementPool_const_MinimalElementType__1024_::new(),
    ));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<MinimalElementType>> = Rc::new(RefCell::new(
            ({ ElementPool_const_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_389(_handle.as_pointer(), _acc.as_pointer())
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_const_MinimalElementType_> = Rc::new(RefCell::new(
            move_counted_handle_const_MinimalElementType_::new_1({
                ({
                    ElementPool_const_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer())
                })
            }),
        ));
        let _acc: Value<checked_accessor_const_MinimalElementType_> = Rc::new(RefCell::new(
            checked_accessor_const_MinimalElementType_::new({ 1024_() }),
        ));
        mixin_layout_390(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn test_397() -> () {
    ({ mixin_accessor_391() });
    ({ mixin_accessor_392() });
    ({ mixin_accessor_393() });
    ({ mixin_accessor_394() });
    ({ mixin_accessor_395() });
    ({ mixin_accessor_396() });
    let mut D: () = dynamic_extent_331.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "std::is_constructible_v<mds_t, float*, std::extents<int, 3, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, std::extents<int, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, std::extents<int, D, D, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, std::extents<int, D, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, float*, std::extents<unsigned, 3, D, D>>"
    );;
    const _: () = assert!(
        true,
        "!std::is_constructible_v<mds_t, const float*, std::extents<int, 3, D, D>>"
    );;
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
    ({ test_397() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct move_counted_handle_float_;
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
        return ((*self).with(|__s| __s.ptr_.clone())).reinterpret_cast::<()>();
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
        return ((*self).with(|__s| __s.ptr_.clone())).reinterpret_cast::<()>();
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
        return ((*self).with(|__s| __s.ptr_.clone())).reinterpret_cast::<()>();
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
        return ((*self).with(|__s| __s.ptr_.clone())).reinterpret_cast::<()>();
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
pub trait checked_accessor_MinimalElementType_Impl {
    fn access(
        &self,
        mut p: move_counted_handle_MinimalElementType_,
        mut i: (),
    ) -> Ptr<MinimalElementType> {
        unimplemented!()
    }
    fn offset(
        &self,
        mut p: move_counted_handle_MinimalElementType_,
        mut i: (),
    ) -> move_counted_handle_MinimalElementType_ {
        unimplemented!()
    }
}
impl checked_accessor_MinimalElementType_Impl for Ptr<checked_accessor_MinimalElementType_> {}
pub trait checked_accessor_const_MinimalElementType_Impl {
    fn access(
        &self,
        mut p: move_counted_handle_const_MinimalElementType_,
        mut i: (),
    ) -> Ptr<MinimalElementType> {
        unimplemented!()
    }
    fn offset(
        &self,
        mut p: move_counted_handle_const_MinimalElementType_,
        mut i: (),
    ) -> move_counted_handle_const_MinimalElementType_ {
        unimplemented!()
    }
}
impl checked_accessor_const_MinimalElementType_Impl
    for Ptr<checked_accessor_const_MinimalElementType_>
{
}
pub trait checked_accessor_const_double_Impl {
    fn access(&self, p: move_counted_handle_const_double_, i: ()) -> Ptr<()>;
    fn offset(
        &self,
        p: move_counted_handle_const_double_,
        i: (),
    ) -> move_counted_handle_const_double_;
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_const_double_>,
    ) -> Ptr<checked_accessor_const_double_>;
}
impl checked_accessor_const_double_Impl for Ptr<checked_accessor_const_double_> {
    fn access(&self, p: move_counted_handle_const_double_, mut i: ()) -> Ptr<()> {
        let p: Value<move_counted_handle_const_double_> = Rc::new(RefCell::new(p));
        unimplemented!();
        return ({ move_counted_handle_const_double_Impl::operator_index(&p.as_pointer(), i) });
    }
    fn offset(
        &self,
        mut p: move_counted_handle_const_double_,
        mut i: (),
    ) -> move_counted_handle_const_double_ {
        unimplemented!();
        return move_counted_handle_const_double_::new_1({ p.ptr.offset((i) as isize) });
    }
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_const_double_>,
    ) -> Ptr<checked_accessor_const_double_> {
        field!((*self), N).write({ _a0.with(|__s| __s.N) });
        return (*self).clone();
    }
}
pub trait checked_accessor_const_float_Impl {
    fn access(&self, p: move_counted_handle_const_float_, i: ()) -> Ptr<()>;
    fn offset(
        &self,
        p: move_counted_handle_const_float_,
        i: (),
    ) -> move_counted_handle_const_float_;
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_const_float_>,
    ) -> Ptr<checked_accessor_const_float_>;
}
impl checked_accessor_const_float_Impl for Ptr<checked_accessor_const_float_> {
    fn access(&self, p: move_counted_handle_const_float_, mut i: ()) -> Ptr<()> {
        let p: Value<move_counted_handle_const_float_> = Rc::new(RefCell::new(p));
        unimplemented!();
        return ({ move_counted_handle_const_float_Impl::operator_index(&p.as_pointer(), i) });
    }
    fn offset(
        &self,
        mut p: move_counted_handle_const_float_,
        mut i: (),
    ) -> move_counted_handle_const_float_ {
        unimplemented!();
        return move_counted_handle_const_float_::new_1({ p.ptr.offset((i) as isize) });
    }
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_const_float_>,
    ) -> Ptr<checked_accessor_const_float_> {
        field!((*self), N).write({ _a0.with(|__s| __s.N) });
        return (*self).clone();
    }
}
pub trait checked_accessor_const_int_Impl {
    fn access(&self, mut p: move_counted_handle_const_int_, mut i: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn offset(
        &self,
        mut p: move_counted_handle_const_int_,
        mut i: (),
    ) -> move_counted_handle_const_int_ {
        unimplemented!()
    }
}
impl checked_accessor_const_int_Impl for Ptr<checked_accessor_const_int_> {}
pub trait checked_accessor_const_unsigned_int_Impl {
    fn access(&self, p: not_const_convertible_handle_const_unsigned_int_, i: ()) -> ();
    fn offset(&self, p: not_const_convertible_handle_const_unsigned_int_, i: ()) -> Ptr<()>;
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_const_unsigned_int_>,
    ) -> Ptr<checked_accessor_const_unsigned_int_>;
}
impl checked_accessor_const_unsigned_int_Impl for Ptr<checked_accessor_const_unsigned_int_> {
    fn access(&self, p: not_const_convertible_handle_const_unsigned_int_, mut i: ()) -> () {
        let p: Value<not_const_convertible_handle_const_unsigned_int_> = Rc::new(RefCell::new(p));
        unimplemented!();
        return (({
            not_const_convertible_handle_const_unsigned_int_Impl::operator_index(&p.as_pointer(), i)
        })
        .read());
    }
    fn offset(
        &self,
        mut p: not_const_convertible_handle_const_unsigned_int_,
        mut i: (),
    ) -> Ptr<()> {
        unimplemented!();
        return p.ptr.offset((i) as isize);
    }
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_const_unsigned_int_>,
    ) -> Ptr<checked_accessor_const_unsigned_int_> {
        field!((*self), N).write({ _a0.with(|__s| __s.N) });
        return (*self).clone();
    }
}
pub trait checked_accessor_double_Impl {
    fn access(&self, p: no_default_ctor_handle_double_, i: ()) -> Ptr<()>;
    fn offset(&self, p: no_default_ctor_handle_double_, i: ()) -> no_default_ctor_handle_double_;
}
impl checked_accessor_double_Impl for Ptr<checked_accessor_double_> {
    fn access(&self, mut p: no_default_ctor_handle_double_, mut i: ()) -> Ptr<()> {
        unimplemented!();
        return p.ptr.offset((i) as isize);
    }
    fn offset(
        &self,
        mut p: no_default_ctor_handle_double_,
        mut i: (),
    ) -> no_default_ctor_handle_double_ {
        unimplemented!();
        return no_default_ctor_handle_double_::new_1({ p.ptr.offset((i) as isize) });
    }
}
pub trait checked_accessor_float_Impl {
    fn access(&self, mut p: move_counted_handle_float_, mut i: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn offset(&self, mut p: move_counted_handle_float_, mut i: ()) -> move_counted_handle_float_ {
        unimplemented!()
    }
}
impl checked_accessor_float_Impl for Ptr<checked_accessor_float_> {}
pub trait checked_accessor_int_Impl {
    fn access(&self, mut p: move_counted_handle_int_, mut i: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn offset(&self, mut p: move_counted_handle_int_, mut i: ()) -> move_counted_handle_int_ {
        unimplemented!()
    }
}
impl checked_accessor_int_Impl for Ptr<checked_accessor_int_> {}
pub trait checked_accessor_unsigned_int_Impl {
    fn access(&self, p: not_const_convertible_handle_unsigned_int_, i: ()) -> ();
    fn offset(&self, p: not_const_convertible_handle_unsigned_int_, i: ()) -> Ptr<()>;
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_unsigned_int_>,
    ) -> Ptr<checked_accessor_unsigned_int_>;
}
impl checked_accessor_unsigned_int_Impl for Ptr<checked_accessor_unsigned_int_> {
    fn access(&self, p: not_const_convertible_handle_unsigned_int_, mut i: ()) -> () {
        let p: Value<not_const_convertible_handle_unsigned_int_> = Rc::new(RefCell::new(p));
        unimplemented!();
        return (({
            not_const_convertible_handle_unsigned_int_Impl::operator_index(&p.as_pointer(), i)
        })
        .read());
    }
    fn offset(&self, mut p: not_const_convertible_handle_unsigned_int_, mut i: ()) -> Ptr<()> {
        unimplemented!();
        return p.ptr.offset((i) as isize);
    }
    fn copy_assign(
        &self,
        _a0: Ptr<checked_accessor_unsigned_int_>,
    ) -> Ptr<checked_accessor_unsigned_int_> {
        field!((*self), N).write({ _a0.with(|__s| __s.N) });
        return (*self).clone();
    }
}
pub trait layout_wrapping_integral_4__mapping_std_extents_int__Impl {
    fn copy_assign(
        &self,
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_int__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_int__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>
{
}
pub trait layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl for Ptr<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__> {
}
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
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>
{
}
pub trait layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl for Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__> {
}
pub trait layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__Impl {
    fn copy_assign(
        &self,
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>
{
}
pub trait layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl for Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__> {
}
pub trait layout_wrapping_integral_8__mapping_std_extents_int__Impl {
    fn copy_assign(
        &self,
        other: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    ) -> Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_8__mapping_std_extents_int__Impl
    for Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>
{
}
pub trait layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl for Ptr<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__> {
}
pub trait layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__Impl {
    fn copy_assign(
        &self,
        other: Ptr<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        >,
    ) -> Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>
    {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__Impl
    for Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>
{
}
pub trait layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl for Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__> {
}
pub trait layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__Impl {
    fn copy_assign(
        &self,
        other: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    ) -> Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__Impl
    for Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>
{
}
pub trait layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn required_span_size(&self) -> () {
        unimplemented!()
    }
    fn is_unique(&self) -> () {
        unimplemented!()
    }
    fn is_strided(&self) -> () {
        unimplemented!()
    }
    fn stride(&self, mut r: ()) -> () {
        unimplemented!()
    }
}
impl layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl for Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__> {
}
pub trait move_counted_handle_MinimalElementType_Impl {
    fn operator_index(&self, mut i: ()) -> Ptr<MinimalElementType> {
        unimplemented!()
    }
}
impl move_counted_handle_MinimalElementType_Impl for Ptr<move_counted_handle_MinimalElementType_> {}
pub trait move_counted_handle_const_MinimalElementType_Impl {
    fn operator_index(&self, mut i: ()) -> Ptr<MinimalElementType> {
        unimplemented!()
    }
}
impl move_counted_handle_const_MinimalElementType_Impl
    for Ptr<move_counted_handle_const_MinimalElementType_>
{
}
pub trait move_counted_handle_const_double_Impl {
    fn operator_index(&self, i: ()) -> Ptr<()>;
}
impl move_counted_handle_const_double_Impl for Ptr<move_counted_handle_const_double_> {
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr.clone()).offset((i) as isize);
    }
}
pub trait move_counted_handle_const_float_Impl {
    fn operator_index(&self, i: ()) -> Ptr<()>;
}
impl move_counted_handle_const_float_Impl for Ptr<move_counted_handle_const_float_> {
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr.clone()).offset((i) as isize);
    }
}
pub trait move_counted_handle_const_int_Impl {
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl move_counted_handle_const_int_Impl for Ptr<move_counted_handle_const_int_> {}
pub trait move_counted_handle_double_Impl {
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl move_counted_handle_double_Impl for Ptr<move_counted_handle_double_> {}
pub trait move_counted_handle_int_Impl {
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl move_counted_handle_int_Impl for Ptr<move_counted_handle_int_> {}
pub trait not_const_convertible_handle_const_unsigned_int_Impl {
    fn operator_index(&self, i: ()) -> Ptr<()>;
}
impl not_const_convertible_handle_const_unsigned_int_Impl
    for Ptr<not_const_convertible_handle_const_unsigned_int_>
{
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr.clone()).offset((i) as isize);
    }
}
pub trait not_const_convertible_handle_unsigned_int_Impl {
    fn operator_index(&self, i: ()) -> Ptr<()>;
}
impl not_const_convertible_handle_unsigned_int_Impl
    for Ptr<not_const_convertible_handle_unsigned_int_>
{
    fn operator_index(&self, mut i: ()) -> Ptr<()> {
        return (*self).with(|__s| __s.ptr.clone()).offset((i) as isize);
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = Wrap_0.with(|_| ());
    let _ = Wrap_1.with(|_| ());
    let _ = Wrap_2.with(|_| ());
    let _ = Wrap_3.with(|_| ());
    let _ = Wrap_4.with(|_| ());
    let _ = Wrap_5.with(|_| ());
    let _ = Wrap_6.with(|_| ());
    let _ = Wrap_7.with(|_| ());
    let _ = Wrap_8.with(|_| ());
    let _ = Wrap_9.with(|_| ());
    let _ = Wrap_10.with(|_| ());
    let _ = Wrap_11.with(|_| ());
}
