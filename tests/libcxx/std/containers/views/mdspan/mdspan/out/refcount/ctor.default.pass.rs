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
    static Wrap_0: Value<()> = Rc::new(RefCell::new(4_()));
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
    static Wrap_1: Value<()> = Rc::new(RefCell::new(4_()));
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
    static Wrap_2: Value<()> = Rc::new(RefCell::new(4_()));
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
    static Wrap_3: Value<()> = Rc::new(RefCell::new((4_() as ())));
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
    static Wrap_4: Value<()> = Rc::new(RefCell::new(4_()));
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
    static Wrap_5: Value<()> = Rc::new(RefCell::new(4_()));
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
pub fn construct_mapping_6(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_7(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_8(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_9(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_10(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_11(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
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
pub fn construct_mapping_18(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_int__::new_1({ exts.as_pointer() }, {
        <not_extents_constructible_tag>::default()
    });
}
pub fn construct_mapping_19(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_20(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::new_1(
        { exts.as_pointer() },
        { <not_extents_constructible_tag>::default() },
    );
}pub fn construct_mapping_21 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_22 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_23 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct move_counted_handle_const_double_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl move_counted_handle_const_double_ {
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
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
}
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
pub fn test_mdspan_types_24(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_25(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_26(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_27(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_28(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_29(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_30(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_31(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_32(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_33(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_34(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_35(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_36(
    _a0: Ptr<Ptr<()>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_37(
    _a0: Ptr<Ptr<()>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_38(
    _a0: Ptr<Ptr<()>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_39(
    _a0: Ptr<Ptr<()>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_40(
    _a0: Ptr<Ptr<()>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_41(
    _a0: Ptr<Ptr<()>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_42(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_43(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_44(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_45(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_46(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_47(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_48(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_49(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_50(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_51(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_52(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_53(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_54(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_55(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_56(
    _a0: Ptr<move_counted_handle_int_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_57(
    _a0: Ptr<move_counted_handle_int_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_58(
    _a0: Ptr<move_counted_handle_int_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_59(
    _a0: Ptr<move_counted_handle_int_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_60(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_61(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_62(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_63(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_64(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_65(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_66(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_67(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_68(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_69(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_70(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_71(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_72(
    _a0: Ptr<Ptr<()>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_73(
    _a0: Ptr<Ptr<()>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_74(
    _a0: Ptr<Ptr<()>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_75(
    _a0: Ptr<Ptr<()>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_76(
    _a0: Ptr<Ptr<()>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_77(
    _a0: Ptr<Ptr<()>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_78(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_79(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_80(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_81(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_82(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_83(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_84(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_85(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_86(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_87(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_88(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_89(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_90(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_91(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_92(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_93(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_94(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_95(
    _a0: Ptr<move_counted_handle_const_int_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_96(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_97(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_98(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_99(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_100(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_101(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_102(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_103(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_104(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_105(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_106(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_107(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_108(
    _a0: Ptr<Ptr<()>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_109(
    _a0: Ptr<Ptr<()>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_110(
    _a0: Ptr<Ptr<()>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_111(
    _a0: Ptr<Ptr<()>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_112(
    _a0: Ptr<Ptr<()>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_113(
    _a0: Ptr<Ptr<()>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_114(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_115(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_116(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_117(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_118(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_119(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_120(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_121(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_122(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_123(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_124(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_125(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_126(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_127(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_128(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_129(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_130(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_131(
    _a0: Ptr<no_default_ctor_handle_double_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_132(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_133(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_134(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_135(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_136(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_137(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_138(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_139(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_140(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_141(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_142(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_143(_a0: Ptr<Ptr<()>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_144(
    _a0: Ptr<Ptr<()>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_145(
    _a0: Ptr<Ptr<()>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_146(
    _a0: Ptr<Ptr<()>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_147(
    _a0: Ptr<Ptr<()>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_148(
    _a0: Ptr<Ptr<()>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_149(
    _a0: Ptr<Ptr<()>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_150(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_151(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_152(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_153(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_154(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_155(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_156(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_157(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_158(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_159(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_160(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_161(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_162(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_163(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_164(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_165(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_166(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_167(
    _a0: Ptr<move_counted_handle_const_double_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_168(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_169(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_170(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_171(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_172(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_173(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_174(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_175(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_176(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_177(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_178(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_179(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_180(
    _a0: Ptr<Ptr<MinimalElementType>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_181(
    _a0: Ptr<Ptr<MinimalElementType>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_182(
    _a0: Ptr<Ptr<MinimalElementType>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_183(
    _a0: Ptr<Ptr<MinimalElementType>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_184(
    _a0: Ptr<Ptr<MinimalElementType>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_185(
    _a0: Ptr<Ptr<MinimalElementType>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_186(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_187(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_188(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_189(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_190(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_191(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_192(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_193(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_194(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_195(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_196(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_197(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_198(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_199(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_200(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_201(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_202(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_203(
    _a0: Ptr<move_counted_handle_MinimalElementType_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_204(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_205(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_206(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_207(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_208(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_209(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_210(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_211(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_212(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_213(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_214(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_215(_a0: Ptr<Ptr<MinimalElementType>>, _a1: Ptr<()>, _a2: Ptr<()>) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_types_216(
    _a0: Ptr<Ptr<MinimalElementType>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_217(
    _a0: Ptr<Ptr<MinimalElementType>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_218(
    _a0: Ptr<Ptr<MinimalElementType>>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_219(
    _a0: Ptr<Ptr<MinimalElementType>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_220(
    _a0: Ptr<Ptr<MinimalElementType>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_221(
    _a0: Ptr<Ptr<MinimalElementType>>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_222(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_223(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_224(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_225(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_226(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_227(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_228(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_229(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_230(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_231(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_232(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_233(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<()>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_234(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_235(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_236(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_237(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_238(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn test_mdspan_types_239(
    _a0: Ptr<move_counted_handle_const_MinimalElementType_>,
    _a1 : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(true, "hc == std::is_default_constructible_v<H>");;
    const _: () = assert!(true, "mc == std::is_default_constructible_v<M>");;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        const _: () = assert!(true, "!std::is_default_constructible_v<MDS>");;
    }
}
pub fn mixin_extents_240(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_6(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_24(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_7(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_25(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_8(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_26(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_9(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_27(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_10(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_28(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_11(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_29(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_242(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_30(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_31(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_32(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_33(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_34(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_35(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_243(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_18((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_36(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_19((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_37(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_20((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_38(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_21 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_39(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_22 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_40(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_23 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_41(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_244(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_6(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_42(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_7(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_43(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_8(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_44(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_9(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_45(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_10(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_46(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_11(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_47(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_245(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_48(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_49(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_50(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_51(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_52(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_53(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_246(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_18((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_54(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_19((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_55(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_20((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_56(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_21 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_57(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_22 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_58(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_int_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_23 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_59(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_247(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_6(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_60(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_7(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_61(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_8(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_62(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_9(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_63(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_10(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_64(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_11(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_65(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_248(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_66(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_67(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_68(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_69(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_70(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_71(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_249(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_18((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_72(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_19((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_73(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_20((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_74(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_21 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_75(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_22 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_76(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_23 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_77(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_250(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_6(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_78(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_7(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_79(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_8(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_80(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_9(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_81(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_10(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_82(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_11(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_83(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_251(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_84(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_85(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_86(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_87(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_88(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_89(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_252(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_18((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_90(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_19((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_91(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_20((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_92(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_21 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_93(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_22 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_94(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_23 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_95(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_253(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_6(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_96(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_7(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_97(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_8(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_98(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_9(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_99(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_10(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_100(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_11(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_101(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_254(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_102(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_103(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_104(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_105(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_106(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_107(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_255(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_18((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_108(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_19((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_109(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_20((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_110(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_21 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_111(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_22 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_112(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_23 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_113(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_256(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_6(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_114(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_7(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_115(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_8(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_116(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_9(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_117(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_10(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_118(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_11(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_119(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_257(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_120(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_121(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_122(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_123(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_124(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_125(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_258(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_18((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_126(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_19((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_127(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_20((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_128(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_21 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_129(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_22 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_130(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_23 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_131(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_259(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_6(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_132(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_7(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_133(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_8(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_134(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_9(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_135(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_10(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_136(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_11(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_137(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_260(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_138(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_139(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_140(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_141(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_142(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_143(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_261(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_18((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_144(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_19((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_145(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_20((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_146(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_21 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_147(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_22 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_148(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<()>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_23 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_149(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_262(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_6(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_150(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_7(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_151(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_8(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_152(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_9(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_153(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_10(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_154(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_11(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_155(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_263(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_156(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_157(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_158(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_159(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_160(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_161(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_264(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_18((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_162(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_19((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_163(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_20((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_164(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_21 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_165(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_22 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_166(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_23 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_167(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_265(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_6(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_168(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_7(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_169(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_8(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_170(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_9(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_171(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_10(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_172(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_11(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_173(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_266(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_174(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_175(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_176(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_177(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_178(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_179(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_267(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_18((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_180(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_19((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_181(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_20((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_182(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_21 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_183(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_22 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_184(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_23 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_185(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_268(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_6(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_186(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_7(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_187(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_8(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_188(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_9(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_189(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_10(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_190(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_11(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_191(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_269(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_192(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_193(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_194(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_195(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_196(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_197(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_270(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_18((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_198(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_19((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_199(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_20((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_200(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_21 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_201(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_22 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_202(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_23 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_203(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_271(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_6(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_204(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_7(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_205(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_8(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_206(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_9(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_207(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_10(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_208(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_11(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_209(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_272(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_210(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_211(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_212(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_213(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_214(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_215(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_273(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_18((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_216(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_19((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_217(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_20((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_218(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_21 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_219(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_22 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_220(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_23 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<()> = (acc).clone();
        test_mdspan_types_221(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_274(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_6(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_222(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_7(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_223(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_8(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_224(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_9(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_225(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_10(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_226(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_11(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_227(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_275(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_228(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_229(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_230(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_231(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_232(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_233(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_extents_276(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_18((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_234(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_19((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_235(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_20((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_236(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_21 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_237(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_22 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_238(_arg0, _arg1.as_pointer(), _arg2)
    });
    ({
        let _arg0: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _arg1 : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_23 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _arg2: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_239(_arg0, _arg1.as_pointer(), _arg2)
    });
}
pub fn mixin_layout_277(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_240(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_242(_handle, _layout.as_pointer(), _acc)
    });
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_243(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_278(handle: Ptr<move_counted_handle_int_>, acc: Ptr<checked_accessor_int_>) {
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_244(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_245(_handle, _layout.as_pointer(), _acc)
    });
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_246(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_279(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_247(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_248(_handle, _layout.as_pointer(), _acc)
    });
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_249(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_280(
    handle: Ptr<move_counted_handle_const_int_>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_250(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_251(_handle, _layout.as_pointer(), _acc)
    });
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_252(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_281(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_253(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_254(_handle, _layout.as_pointer(), _acc)
    });
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_255(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_282(
    handle: Ptr<no_default_ctor_handle_double_>,
    acc: Ptr<checked_accessor_double_>,
) {
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_256(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_257(_handle, _layout.as_pointer(), _acc)
    });
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_258(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_283(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_259(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_260(_handle, _layout.as_pointer(), _acc)
    });
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_261(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_284(
    handle: Ptr<move_counted_handle_const_double_>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_262(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_263(_handle, _layout.as_pointer(), _acc)
    });
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_264(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_285(handle: Ptr<Ptr<MinimalElementType>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_265(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_266(_handle, _layout.as_pointer(), _acc)
    });
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_267(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_286(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_268(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_269(_handle, _layout.as_pointer(), _acc)
    });
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_270(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_287(handle: Ptr<Ptr<MinimalElementType>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_271(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_272(_handle, _layout.as_pointer(), _acc)
    });
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_273(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_288(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_274(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_275(_handle, _layout.as_pointer(), _acc)
    });
    let mut D: () = dynamic_extent_241.with(|rc| *rc.borrow());
    const _: () = assert!(
        true,
        "!std::is_default_constructible_v< layout_wrapping_integral<4>::mapping<std::extents<signed char, D>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_276(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_accessor_289() {
    let elements: Value<ElementPool_int__1024_> =
        Rc::new(RefCell::new(ElementPool_int__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_int__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_277(_handle.as_pointer(), _acc.as_pointer())
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<typename checked_accessor<T>::data_handle_type> != std::is_same_v<T, double>"
    );;
    ({
        let _handle: Value<move_counted_handle_int_> =
            Rc::new(RefCell::new(move_counted_handle_int_::new_1({
                ({ ElementPool_int__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        let _acc: Value<checked_accessor_int_> =
            Rc::new(RefCell::new(checked_accessor_int_::new({ 1024_() })));
        mixin_layout_278(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_290() {
    let elements: Value<ElementPool_const_int__1024_> =
        Rc::new(RefCell::new(ElementPool_const_int__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_const_int__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_279(_handle.as_pointer(), _acc.as_pointer())
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<typename checked_accessor<T>::data_handle_type> != std::is_same_v<T, double>"
    );;
    ({
        let _handle: Value<move_counted_handle_const_int_> =
            Rc::new(RefCell::new(move_counted_handle_const_int_::new_1({
                ({ ElementPool_const_int__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        let _acc: Value<checked_accessor_const_int_> =
            Rc::new(RefCell::new(checked_accessor_const_int_::new({ 1024_() })));
        mixin_layout_280(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_291() {
    let elements: Value<ElementPool_double__1024_> =
        Rc::new(RefCell::new(ElementPool_double__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_double__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_281(_handle.as_pointer(), _acc.as_pointer())
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<typename checked_accessor<T>::data_handle_type> != std::is_same_v<T, double>"
    );;
    ({
        let _handle: Value<no_default_ctor_handle_double_> =
            Rc::new(RefCell::new(no_default_ctor_handle_double_::new_1({
                ({ ElementPool_double__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        let _acc: Value<checked_accessor_double_> =
            Rc::new(RefCell::new(checked_accessor_double_::new({ 1024_() })));
        mixin_layout_282(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_292() {
    let elements: Value<ElementPool_const_double__1024_> =
        Rc::new(RefCell::new(ElementPool_const_double__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_const_double__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_283(_handle.as_pointer(), _acc.as_pointer())
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<typename checked_accessor<T>::data_handle_type> != std::is_same_v<T, double>"
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
        mixin_layout_284(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_293() {
    let elements: Value<ElementPool_MinimalElementType__1024_> =
        Rc::new(RefCell::new(ElementPool_MinimalElementType__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<MinimalElementType>> = Rc::new(RefCell::new(
            ({ ElementPool_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_285(_handle.as_pointer(), _acc.as_pointer())
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<typename checked_accessor<T>::data_handle_type> != std::is_same_v<T, double>"
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
        mixin_layout_286(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_294() {
    let elements: Value<ElementPool_const_MinimalElementType__1024_> = Rc::new(RefCell::new(
        ElementPool_const_MinimalElementType__1024_::new(),
    ));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<MinimalElementType>> = Rc::new(RefCell::new(
            ({ ElementPool_const_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_287(_handle.as_pointer(), _acc.as_pointer())
    });
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<checked_accessor<T>> == std::is_same_v<T, const double>"
    );;
    const _: () = assert!(
        true,
        "std::is_default_constructible_v<typename checked_accessor<T>::data_handle_type> != std::is_same_v<T, double>"
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
        mixin_layout_288(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn test_295() -> () {
    ({ mixin_accessor_289() });
    ({ mixin_accessor_290() });
    ({ mixin_accessor_291() });
    ({ mixin_accessor_292() });
    ({ mixin_accessor_293() });
    ({ mixin_accessor_294() });
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
    ({ test_295() });
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
}
