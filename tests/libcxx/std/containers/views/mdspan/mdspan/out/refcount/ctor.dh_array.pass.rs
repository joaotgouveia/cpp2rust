extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntType {
    #[offset(0)]
    pub val: (),
}
impl IntType {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
impl std::cmp::PartialEq for IntType {
    fn eq(&self, other: &Self) -> bool {
        {
            IntTypeImpl::operator_eq(
                &Rc::new(RefCell::new(IntType {
                    val: self.val.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(IntType {
                    val: other.val.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for IntType {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntTypeNC {
    #[offset(0)]
    pub val: (),
}
impl IntTypeNC {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
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
    static Wrap_0: Value<()> = Rc::new(RefCell::new(8_()));
);
pub fn operator_eq_1(
    lhs: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    rhs: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
) -> () {
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_8__mapping_std_extents_int__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_int__ {
    pub fn new_1(ext: Ptr<()>) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
    pub fn new_2(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
}
impl std::cmp::PartialEq for layout_wrapping_integral_8__mapping_std_extents_int__ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(
                    layout_wrapping_integral_8__mapping_std_extents_int__ {
                        extents_: self.extents_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    layout_wrapping_integral_8__mapping_std_extents_int__ {
                        extents_: other.extents_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for layout_wrapping_integral_8__mapping_std_extents_int__ {}
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
    static Wrap_2: Value<()> = Rc::new(RefCell::new(8_()));
);
pub fn operator_eq_3(
    lhs: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    rhs: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
) -> () {
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ {
    pub fn new_1(ext: Ptr<()>) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
    pub fn new_2(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
}
impl std::cmp::PartialEq
    for layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__
{
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_3(Rc::new(RefCell::new(layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ { extents_: self.extents_.clone(), })).as_pointer(), Rc::new(RefCell::new(layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ { extents_: other.extents_.clone(), })).as_pointer())
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
        {
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_4: Value<()> = Rc::new(RefCell::new(8_()));
);
pub fn operator_eq_5(
    lhs: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    rhs: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
) -> () {
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    pub fn new_1(ext: Ptr<()>) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
    pub fn new_2(ext: Ptr<()>, mut _a1: not_extents_constructible_tag) -> Self {
        Self {
            extents_: unimplemented!(),
        }
    }
}
impl std::cmp::PartialEq for layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_5(
                Rc::new(RefCell::new(
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
                        extents_: self.extents_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
                        extents_: other.extents_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {}
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
    static Wrap_6: Value<()> = Rc::new(RefCell::new((8_() as ())));
);
pub fn operator_eq_7(
    lhs : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    rhs : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
) -> () {
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(16)]
pub struct layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(16)]
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn new_2 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl std::cmp::PartialEq for layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_7(Rc::new(RefCell::new(layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_: self.extents_.clone(), })).as_pointer(), Rc::new(RefCell::new(layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_: other.extents_.clone(), })).as_pointer()) }}}
impl std::cmp::Eq for layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ {}
impl Default for layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_8: Value<()> = Rc::new(RefCell::new(8_()));
);
pub fn operator_eq_9(
    lhs : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    rhs : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
) -> () {
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(2)]
pub struct layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(2)]
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn new_2 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl std::cmp::PartialEq for layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_9(Rc::new(RefCell::new(layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_: self.extents_.clone(), })).as_pointer(), Rc::new(RefCell::new(layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_: other.extents_.clone(), })).as_pointer()) }}}
impl std::cmp::Eq for layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ {}
impl Default for layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_10: Value<()> = Rc::new(RefCell::new(8_()));
);
pub fn operator_eq_11(
    lhs : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    rhs : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
) -> () {
    return unimplemented!();
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(32)]
pub struct layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(32)]
    extents_: (),
}
impl layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { pub fn new_1 ( ext : Ptr< ()  > , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn new_2 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl std::cmp::PartialEq for layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_11(Rc::new(RefCell::new(layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_: self.extents_.clone(), })).as_pointer(), Rc::new(RefCell::new(layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_: other.extents_.clone(), })).as_pointer()) }}}
impl std::cmp::Eq for layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ {}
impl Default for layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct layout_wrapping_integral_8_ {}
thread_local!(
    static Wrap_12: Value<()> = Rc::new(RefCell::new(4_()));
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
    static Wrap_13: Value<()> = Rc::new(RefCell::new(4_()));
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
    static Wrap_14: Value<()> = Rc::new(RefCell::new(4_()));
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
    static Wrap_15: Value<()> = Rc::new(RefCell::new((4_() as ())));
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
    static Wrap_16: Value<()> = Rc::new(RefCell::new(4_()));
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
    static Wrap_17: Value<()> = Rc::new(RefCell::new(4_()));
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
pub fn construct_mapping_24(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_25(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_26(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_27(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_28(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_29(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_30(
    mut _a0: layout_wrapping_integral_8_,
    exts: (),
) -> layout_wrapping_integral_8__mapping_std_extents_int__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_8__mapping_std_extents_int__::new_2({ exts.as_pointer() }, {
        <not_extents_constructible_tag>::default()
    });
}
pub fn construct_mapping_31(
    mut _a0: layout_wrapping_integral_8_,
    exts: (),
) -> layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__ :: new_2 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_32(
    mut _a0: layout_wrapping_integral_8_,
    exts: (),
) -> layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__::new_2(
        { exts.as_pointer() },
        { <not_extents_constructible_tag>::default() },
    );
}pub fn construct_mapping_33 ( mut _a0 : layout_wrapping_integral_8_  , exts : ()  , ) -> layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_2 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_34 ( mut _a0 : layout_wrapping_integral_8_  , exts : ()  , ) -> layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_2 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_35 ( mut _a0 : layout_wrapping_integral_8_  , exts : ()  , ) -> layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ :: new_2 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_36(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_int__::new_1({ exts.as_pointer() }, {
        <not_extents_constructible_tag>::default()
    });
}
pub fn construct_mapping_37(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_38(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::new_1(
        { exts.as_pointer() },
        { <not_extents_constructible_tag>::default() },
    );
}pub fn construct_mapping_39 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_40 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_41 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
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
        return c_42.with(|v| v.as_pointer());
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
        return c_43.with(|v| v.as_pointer());
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
        return c_44.with(|v| v.as_pointer());
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
        return c_45.with(|v| v.as_pointer());
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
        return c_46.with(|v| v.as_pointer());
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
        return c_47.with(|v| v.as_pointer());
    }
}
thread_local!(
    static c_42: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_43: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_44: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_45: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_46: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_47: Value<()> = Rc::new(RefCell::new(0_()));
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
pub fn array_from_extents_48(exts: Ptr<()>, _a1: ()) -> () {
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return unimplemented!();
}
pub fn array_from_extents_49(exts: Ptr<()>, _a1: ()) -> () {
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return unimplemented!();
}
pub fn array_from_extents_50(exts: Ptr<()>, _a1: ()) -> () {
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return unimplemented!();
}
pub fn array_from_extents_51(exts: Ptr<()>, _a1: ()) -> () {
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return unimplemented!();
}
pub fn array_from_extents_52(exts: Ptr<()>, _a1: ()) -> () {
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return unimplemented!();
}
pub fn array_from_extents_53(exts: Ptr<()>, _a1: ()) -> () {
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return unimplemented!();
}
pub fn test_mdspan_ctor_array_54(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_55(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_56(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_57(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_58(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_59(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_60(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_61(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_62(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_63(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_64(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_65(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_66(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_67(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_68(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_69(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_70(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_71(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_72(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_73(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_74(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_75(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_76(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_77(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_78(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_79(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_80(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_81(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_82(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_83(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_84(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_85(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_86(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_87(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_88(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_89(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_90(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_91(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_92(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_93(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_94(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_95(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_96(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_97(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_98(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_99(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_100(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_101(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_102(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_103(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_104(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_105(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_106(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_107(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_108(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_109(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_110(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_111(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_112(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_113(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_int_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_114(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_115(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_116(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_117(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_118(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_119(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_120(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_121(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_122(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_123(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_124(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_125(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_126(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_127(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_128(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_129(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_130(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_131(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_132(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_133(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_134(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_135(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_136(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_137(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_138(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_139(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_140(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_141(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_142(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_143(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_144(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_145(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_146(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_147(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_148(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_149(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_150(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_151(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_152(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_153(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_154(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_155(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_156(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_157(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_158(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_159(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_160(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_161(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_162(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_163(handle: Ptr<Ptr<()>>, map: Ptr<()>, _a2: Ptr<()>, exts: ()) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_164(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_165(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_166(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_167(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_168(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_169(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_170(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_171(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_172(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_173(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_174(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_175(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_176(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_177(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_178(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_179(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_180(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_181(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_182(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_183(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_184(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_185(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_186(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_187(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_188(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_189(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_190(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_191(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_192(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_193(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_194(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_195(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_196(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_197(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_198(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_199(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_200(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_201(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_202(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_203(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<checked_accessor_const_double_>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_double_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_204(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_205(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_206(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_207(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_208(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_209(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_210(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_211(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_212(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_213(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_214(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_215(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_216(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_217(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_218(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_219(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_220(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_221(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_222(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_223(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_224(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_225(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_226(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_227(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_228(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_229(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_230(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_231(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_232(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_233(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_234(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_235(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_236(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_237(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_238(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_239(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_240(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_241(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_242(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_243(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_244(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_245(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_246(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_247(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_248(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_249(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_250(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_251(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_252(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_253(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<()>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_254(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_255(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_256(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_257(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_258(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_259(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_260(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_261(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_262(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_array_263(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    _a2: Ptr<()>,
    exts: (),
) {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    {
        ({ move_counted_handle_const_MinimalElementType_::move_counter() }).write(0_());
    }
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
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
pub fn test_mdspan_ctor_264(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_54(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_54(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_266(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_55(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_55(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_267(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_56(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_57(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_268(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_58(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_59(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_269(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_60(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_61(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_270(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_62(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_63(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_271(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_64(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_64(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_272(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_65(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_65(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_273(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_66(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_67(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_274(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_68(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_69(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_275(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_70(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_71(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_276(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_72(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_73(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_277(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_48(
                    ({ layout_wrapping_integral_8__mapping_std_extents_int__Impl::extents(&map) }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_74(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_74(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_278(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_49(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_75(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_75(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_279(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_50(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_76(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_77(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_280(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_51(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_78(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_79(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_281(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_52(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_80(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_81(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_282(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_53(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_82(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_83(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_283(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_284(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_285(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_286(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_287(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_288(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_289(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_290(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_291(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_292(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_293(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_294(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_295(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_296(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_297(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_298(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_299(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_300(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_301(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_302(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_303(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_304(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_305(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_306(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_307(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_308(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_309(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_310(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_311(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_312(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_int_>,
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
pub fn test_mdspan_ctor_313(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_84(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_84(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_314(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_85(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_85(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_315(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_86(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_87(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_316(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_88(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_89(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_317(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_90(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_91(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_318(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_92(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_93(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_319(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_94(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_94(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_320(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_95(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_95(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_321(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_96(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_97(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_322(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_98(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_99(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_323(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_100(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_101(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_324(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_102(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_103(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_325(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_48(
                    ({ layout_wrapping_integral_8__mapping_std_extents_int__Impl::extents(&map) }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_104(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_104(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_326(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_49(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_105(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_105(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_327(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_50(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_106(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_107(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_328(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_51(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_108(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_109(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_329(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_52(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_110(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_111(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_330(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_53(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_112(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_113(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_331(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_332(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_333(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_334(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_335(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_336(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_337(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_338(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_339(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_340(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_341(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_342(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_343(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_344(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_345(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_346(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_347(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_348(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_349(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_350(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_351(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_352(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_353(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_354(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_355(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_356(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_357(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_358(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_359(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_360(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_int_>,
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
pub fn test_mdspan_ctor_361(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_114(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_114(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_362(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_115(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_115(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_363(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_116(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_117(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_364(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_118(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_119(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_365(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_120(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_121(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_366(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_122(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_123(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_367(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_124(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_124(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_368(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_125(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_125(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_369(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_126(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_127(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_370(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_128(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_129(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_371(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_130(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_131(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_372(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_132(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_133(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_373(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_48(
                    ({ layout_wrapping_integral_8__mapping_std_extents_int__Impl::extents(&map) }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_134(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_134(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_374(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_49(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_135(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_135(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_375(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_50(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_136(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_137(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_376(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_51(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_138(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_139(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_377(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_52(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_140(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_141(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_378(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_53(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_142(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_143(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_379(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_380(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_381(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_382(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_383(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_384(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_385(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_386(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_387(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_388(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_389(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_390(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_391(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_392(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_393(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_394(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_395(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_396(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_397(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_398(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_399(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_400(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_401(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_402(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_403(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_404(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_405(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_406(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_407(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_408(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_double_>,
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
pub fn test_mdspan_ctor_409(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_144(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_144(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_410(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_145(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_145(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_411(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_146(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_147(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_412(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_148(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_149(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_413(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_150(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_151(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_414(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_152(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_153(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_415(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_154(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_154(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_416(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_155(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_155(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_417(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_156(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_157(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_418(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_158(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_159(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_419(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_160(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_161(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_420(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_162(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_163(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_421(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_48(
                    ({ layout_wrapping_integral_8__mapping_std_extents_int__Impl::extents(&map) }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_164(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_164(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_422(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_49(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_165(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_165(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_423(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_50(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_166(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_167(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_424(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_51(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_168(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_169(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_425(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_52(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_170(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_171(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_426(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_53(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_172(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<()>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_173(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_427(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_428(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_429(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_430(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_431(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_432(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_433(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_174(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_174(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_434(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_175(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_175(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_435(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_176(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_177(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_436(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_178(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_179(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_437(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_180(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_181(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_438(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_182(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_183(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_439(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_184(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_184(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_440(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_185(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_185(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_441(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_186(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_187(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_442(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_188(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_189(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_443(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_190(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_191(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_444(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_192(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_193(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_445(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_48(
                    ({ layout_wrapping_integral_8__mapping_std_extents_int__Impl::extents(&map) }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_194(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_194(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_446(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_49(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_195(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_195(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_447(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_50(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_196(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_197(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_448(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_51(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_198(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_199(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_449(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_52(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_200(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_201(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_450(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_53(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_202(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<checked_accessor_const_double_> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_203(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_451(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_const_double_>,
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
pub fn test_mdspan_ctor_452(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_const_double_>,
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
pub fn test_mdspan_ctor_453(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_const_double_>,
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
pub fn test_mdspan_ctor_454(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_double_>,
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
pub fn test_mdspan_ctor_455(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_double_>,
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
pub fn test_mdspan_ctor_456(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_double_>,
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
pub fn test_mdspan_ctor_457(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_204(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_204(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_458(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_205(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_205(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_459(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_206(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_207(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_460(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_208(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_209(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_461(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_210(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_211(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_462(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_212(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_213(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_463(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_214(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_214(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_464(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_215(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_215(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_465(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_216(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_217(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_466(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_218(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_219(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_467(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_220(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_221(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_468(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_222(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_223(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_469(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_48(
                    ({ layout_wrapping_integral_8__mapping_std_extents_int__Impl::extents(&map) }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_224(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_224(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_470(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_49(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_225(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_225(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_471(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_50(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_226(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_227(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_472(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_51(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_228(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_229(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_473(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_52(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_230(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_231(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_474(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_53(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_232(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_233(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_475(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_476(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_477(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_478(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_479(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_480(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_481(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_482(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_483(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_484(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_485(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_486(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_487(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_488(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_489(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_490(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_491(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_492(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_493(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_494(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_495(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_496(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_497(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_498(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_499(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_500(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_501(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_502(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_503(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_504(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_MinimalElementType_>,
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
pub fn test_mdspan_ctor_505(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_234(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_234(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_506(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_235(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_235(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_507(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_236(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_237(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_508(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_238(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_239(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_509(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_240(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_241(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_510(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_242(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_243(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_511(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_48(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_244(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_244(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_512(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_49(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_245(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_245(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_513(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_50(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_246(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_247(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_514(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_51(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_248(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_249(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_515(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_52(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_250(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_251(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_516(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({ array_from_extents_53(unimplemented!(), unimplemented!()) }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_252(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<()> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_253(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_517(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_48(
                    ({ layout_wrapping_integral_8__mapping_std_extents_int__Impl::extents(&map) }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_254(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__> = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_254(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_518(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_49(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_255(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                let __rhs = (unimplemented!().read());
                unimplemented!().write(__rhs);
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<
                layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
            > = (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_255(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_519(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_50(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_256(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
                (map).clone();
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_257(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_520(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_51(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_258(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_259(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_521(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_52(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_260(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_261(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_522(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    const _: () = assert!(
        true,
        "mec == std::is_constructible_v<M, typename M::extents_type>"
    );;
    const _: () = assert!(true, "ac == std::is_default_constructible_v<A>");;
    {
        let exts: Value<()> = Rc::new(RefCell::new(
            ({
                array_from_extents_53(
                    ({
                        layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl :: extents ( &map  , )
                    }),
                    unimplemented!(),
                )
            }),
        ));
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts.borrow()).clone();
            test_mdspan_ctor_array_262(_handle, _map, _arg2, _exts)
        });
        let exts_dynamic: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r_dyn: () = 0_();
        let mut r: () = 0_();
        'loop_: while (r < unimplemented!()) {
            if (unimplemented!() == dynamic_extent_265.with(|rc| *rc.borrow())) {
                unimplemented!().write((unimplemented!().read()));
            }
            r.postfix_inc();
        }
        ({
            let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
            let _map: Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >  = (map ).clone() ;
            let _arg2: Ptr<()> = (acc).clone();
            let _exts: () = (*exts_dynamic.borrow()).clone();
            test_mdspan_ctor_array_263(_handle, _map, _arg2, _exts)
        });
    }
}
pub fn test_mdspan_ctor_523(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_524(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_525(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_526(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_527(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_528(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
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
pub fn test_mdspan_ctor_529(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_530(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_531(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_532(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_533(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_534(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_535(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_536(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_537(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_538(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_539(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_540(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_541(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_542(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_543(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_544(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_545(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_546(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_547(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_548(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_549(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_550(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_551(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn test_mdspan_ctor_552(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
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
pub fn mixin_extents_553(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_264(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_266(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_267(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_268(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_269(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_270(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_554(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_24(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_271(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_25(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_272(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_26(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_273(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_27(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_274(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_28(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_275(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_29(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_276(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_555(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_277(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_278(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_279(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_280(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_281(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_282(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_556(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_36((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_283(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_37((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_284(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_38((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_285(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_39 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_286(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_40 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_287(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_41 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_288(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_557(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_289(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_290(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_291(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_292(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_293(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_294(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_558(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_24(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_295(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_25(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_296(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_26(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_297(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_27(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_298(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_28(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_299(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_29(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_300(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_559(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_301(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_302(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_303(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_304(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_305(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_306(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_560(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_36((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_307(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_37((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_308(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_38((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_309(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_39 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_310(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_40 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_311(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_41 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_ctor_312(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_561(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_313(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_314(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_315(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_316(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_317(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_318(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_562(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_24(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_319(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_25(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_320(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_26(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_321(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_27(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_322(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_28(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_323(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_29(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_324(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_563(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_325(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_326(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_327(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_328(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_329(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_330(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_564(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_36((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_331(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_37((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_332(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_38((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_333(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_39 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_334(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_40 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_335(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_41 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_336(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_565(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_337(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_338(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_339(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_340(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_341(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_342(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_566(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_24(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_343(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_25(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_344(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_26(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_345(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_27(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_346(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_28(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_347(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_29(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_348(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_567(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_349(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_350(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_351(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_352(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_353(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_354(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_568(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_36((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_355(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_37((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_356(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_38((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_357(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_39 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_358(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_40 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_359(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_41 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_ctor_360(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_569(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_361(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_362(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_363(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_364(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_365(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_366(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_570(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_24(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_367(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_25(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_368(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_26(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_369(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_27(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_370(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_28(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_371(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_29(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_372(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_571(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_373(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_374(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_375(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_376(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_377(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_378(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_572(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_36((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_379(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_37((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_380(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_38((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_381(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_39 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_382(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_40 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_383(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_41 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_384(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_573(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_385(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_386(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_387(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_388(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_389(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_390(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_574(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_24(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_391(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_25(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_392(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_26(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_393(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_27(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_394(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_28(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_395(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_29(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_396(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_575(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_397(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_398(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_399(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_400(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_401(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_402(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_576(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_36((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_403(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_37((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_404(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_38((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_405(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_39 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_406(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_40 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_407(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_41 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_ctor_408(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_577(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_409(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_410(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_411(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_412(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_413(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_414(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_578(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_24(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_415(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_25(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_416(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_26(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_417(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_27(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_418(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_28(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_419(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_29(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_420(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_579(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_421(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_422(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_423(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_424(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_425(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_426(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_580(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_36((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_427(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_37((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_428(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_38((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_429(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_39 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_430(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_40 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_431(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_41 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_432(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_581(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_433(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_434(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_435(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_436(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_437(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_438(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_582(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_24(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_439(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_25(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_440(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_26(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_441(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_27(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_442(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_28(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_443(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_29(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_444(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_583(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_445(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_446(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_447(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_448(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_449(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_450(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_584(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_36((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_451(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_37((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_452(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_38((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_453(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_39 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_454(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_40 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_455(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_41 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_ctor_456(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_585(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_457(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_458(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_459(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_460(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_461(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_462(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_586(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_24(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_463(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_25(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_464(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_26(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_465(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_27(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_466(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_28(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_467(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_29(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_468(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_587(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_469(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_470(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_471(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_472(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_473(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_474(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_588(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_36((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_475(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_37((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_476(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_38((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_477(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_39 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_478(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_40 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_479(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_41 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_480(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_589(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_481(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_482(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_483(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_484(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_485(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_486(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_590(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_24(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_487(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_25(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_488(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_26(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_489(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_27(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_490(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_28(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_491(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_29(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_492(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_591(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_493(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_494(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_495(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_496(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_497(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_498(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_592(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_36((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_499(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_37((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_500(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_38((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_501(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_39 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_502(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_40 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_503(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_41 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_504(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_593(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_505(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_506(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_507(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_508(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_509(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_510(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_594(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_24(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_511(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_25(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_512(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_26(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_513(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_27(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_514(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_28(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_515(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_29(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_516(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_595(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_517(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_518(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_519(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_520(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_521(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_522(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_596(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_36((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_523(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_37((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_524(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_38((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_525(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_39 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_526(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_40 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_527(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_41 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_ctor_528(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_597(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_529(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_530(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_531(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_532(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_533(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_534(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_598(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_24(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_535(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_25(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_536(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_26(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_537(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_27(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_538(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_28(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_539(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_29(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_540(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_599(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_8_>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_30((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_541(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_31((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_542(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_32((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_543(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_33 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_544(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_34 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_545(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_35 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_546(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_600(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_36((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_547(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_37((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_548(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_38((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_549(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_39 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_550(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_40 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_551(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_41 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_ctor_552(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_layout_601(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_553(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_554(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_555(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_556(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_602(handle: Ptr<move_counted_handle_int_>, acc: Ptr<checked_accessor_int_>) {
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_557(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_558(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_559(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_560(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_603(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_561(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_562(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_563(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_564(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_604(
    handle: Ptr<move_counted_handle_const_int_>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_565(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_566(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_567(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_568(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_605(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_569(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_570(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_571(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_572(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_606(
    handle: Ptr<no_default_ctor_handle_double_>,
    acc: Ptr<checked_accessor_double_>,
) {
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_573(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_574(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_575(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_576(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_607(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_577(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_578(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_579(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_580(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_608(
    handle: Ptr<move_counted_handle_const_double_>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_581(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_582(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_583(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_584(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_609(handle: Ptr<Ptr<MinimalElementType>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_585(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_586(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_587(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_588(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_610(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_589(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_590(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_591(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_592(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_611(handle: Ptr<Ptr<MinimalElementType>>, acc: Ptr<()>) {
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_593(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_594(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_595(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_596(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_612(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_597(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_598(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_599(_handle, _layout.as_pointer(), _acc)
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
        mixin_extents_600(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_accessor_613() {
    let elements: Value<ElementPool_int__1024_> =
        Rc::new(RefCell::new(ElementPool_int__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_int__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_601(_handle.as_pointer(), _acc.as_pointer())
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
        mixin_layout_602(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_614() {
    let elements: Value<ElementPool_const_int__1024_> =
        Rc::new(RefCell::new(ElementPool_const_int__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_const_int__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_603(_handle.as_pointer(), _acc.as_pointer())
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
        mixin_layout_604(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_615() {
    let elements: Value<ElementPool_double__1024_> =
        Rc::new(RefCell::new(ElementPool_double__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_double__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_605(_handle.as_pointer(), _acc.as_pointer())
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
        mixin_layout_606(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_616() {
    let elements: Value<ElementPool_const_double__1024_> =
        Rc::new(RefCell::new(ElementPool_const_double__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_const_double__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_607(_handle.as_pointer(), _acc.as_pointer())
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
        mixin_layout_608(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_617() {
    let elements: Value<ElementPool_MinimalElementType__1024_> =
        Rc::new(RefCell::new(ElementPool_MinimalElementType__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<MinimalElementType>> = Rc::new(RefCell::new(
            ({ ElementPool_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_609(_handle.as_pointer(), _acc.as_pointer())
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
        mixin_layout_610(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn mixin_accessor_618() {
    let elements: Value<ElementPool_const_MinimalElementType__1024_> = Rc::new(RefCell::new(
        ElementPool_const_MinimalElementType__1024_::new(),
    ));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    ({
        let _handle: Value<Ptr<MinimalElementType>> = Rc::new(RefCell::new(
            ({ ElementPool_const_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_611(_handle.as_pointer(), _acc.as_pointer())
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
        mixin_layout_612(_handle.as_pointer(), _acc.as_pointer())
    });
}
pub fn test_619() -> () {
    ({ mixin_accessor_613() });
    ({ mixin_accessor_614() });
    ({ mixin_accessor_615() });
    ({ mixin_accessor_616() });
    ({ mixin_accessor_617() });
    ({ mixin_accessor_618() });
    let mut D: () = dynamic_extent_265.with(|rc| *rc.borrow());
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
    ({ test_619() });
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
pub trait IntTypeImpl {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> ();
    fn to__4(&self) -> ();
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl IntTypeImpl for Ptr<IntType> {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn to__4(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__6(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait IntTypeNCImpl {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> ();
    fn to__4(&self) -> ();
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl IntTypeNCImpl for Ptr<IntTypeNC> {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn to__4(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__6(&self) -> () {
        return (*self).with(|__s| __s.val);
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
    fn extents(&self) -> Ptr<()>;
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
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
}
pub trait layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_8__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
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
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } }
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
    fn extents(&self) -> Ptr<()>;
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
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
}
pub trait layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_8__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
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
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } }
pub trait layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__Impl {
    fn copy_assign(
        &self,
        other: Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__>,
    ) -> Ptr<layout_wrapping_integral_8__mapping_std_extents_unsigned_int__7__> {
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
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
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
}
pub trait layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_8__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >{
        unimplemented!()
    }
    fn extents(&self) -> Ptr<()>;
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
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } }
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
    let _ = Wrap_2.with(|_| ());
    let _ = Wrap_4.with(|_| ());
    let _ = Wrap_6.with(|_| ());
    let _ = Wrap_8.with(|_| ());
    let _ = Wrap_10.with(|_| ());
    let _ = Wrap_12.with(|_| ());
    let _ = Wrap_13.with(|_| ());
    let _ = Wrap_14.with(|_| ());
    let _ = Wrap_15.with(|_| ());
    let _ = Wrap_16.with(|_| ());
    let _ = Wrap_17.with(|_| ());
}
