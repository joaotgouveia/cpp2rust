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
    static Wrap_0: Value<()> = Rc::new(RefCell::new(4_()));
);
pub fn operator_eq_1(
    lhs: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    rhs: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
) -> () {
    return unimplemented!();
}
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
}
impl std::cmp::PartialEq for layout_wrapping_integral_4__mapping_std_extents_int__ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(
                    layout_wrapping_integral_4__mapping_std_extents_int__ {
                        extents_: self.extents_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    layout_wrapping_integral_4__mapping_std_extents_int__ {
                        extents_: other.extents_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for layout_wrapping_integral_4__mapping_std_extents_int__ {}
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
    static Wrap_2: Value<()> = Rc::new(RefCell::new(4_()));
);
pub fn operator_eq_3(
    lhs: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    rhs: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
) -> () {
    return unimplemented!();
}
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
}
impl std::cmp::PartialEq
    for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__
{
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_3(Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ { extents_: self.extents_.clone(), })).as_pointer(), Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ { extents_: other.extents_.clone(), })).as_pointer())
        }
    }
}
impl std::cmp::Eq
    for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__
{
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
    static Wrap_4: Value<()> = Rc::new(RefCell::new(4_()));
);
pub fn operator_eq_5(
    lhs: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    rhs: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
) -> () {
    return unimplemented!();
}
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
}
impl std::cmp::PartialEq for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_5(
                Rc::new(RefCell::new(
                    layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
                        extents_: self.extents_.clone(),
                    },
                ))
                .as_pointer(),
                Rc::new(RefCell::new(
                    layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
                        extents_: other.extents_.clone(),
                    },
                ))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {}
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
    static Wrap_6: Value<()> = Rc::new(RefCell::new((4_() as ())));
);
pub fn operator_eq_7(
    lhs : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    rhs : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
) -> () {
    return unimplemented!();
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(16)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { pub fn copy_from ( other : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  > , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl std::cmp::PartialEq for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_7(Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_: self.extents_.clone(), })).as_pointer(), Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_: other.extents_.clone(), })).as_pointer()) }}}
impl std::cmp::Eq for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ {}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn clone(&self) -> Self { let __src: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__> = Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_: self.extents_.clone(), })); layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__::copy_from(__src.as_pointer()) } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_8: Value<()> = Rc::new(RefCell::new(4_()));
);
pub fn operator_eq_9(
    lhs : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    rhs : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
) -> () {
    return unimplemented!();
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(2)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { pub fn copy_from ( other : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  > , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl std::cmp::PartialEq for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_9(Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_: self.extents_.clone(), })).as_pointer(), Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_: other.extents_.clone(), })).as_pointer()) }}}
impl std::cmp::Eq for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ {}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn clone(&self) -> Self { let __src: Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__> = Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_: self.extents_.clone(), })); layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__::copy_from(__src.as_pointer()) } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_10: Value<()> = Rc::new(RefCell::new(4_()));
);
pub fn operator_eq_11(
    lhs : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    rhs : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
) -> () {
    return unimplemented!();
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(32)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { pub fn copy_from ( other : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  > , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl std::cmp::PartialEq for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn eq(&self, other: &Self) -> bool {  { operator_eq_11(Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_: self.extents_.clone(), })).as_pointer(), Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_: other.extents_.clone(), })).as_pointer()) }}}
impl std::cmp::Eq for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ {}
impl Clone for layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { fn clone(&self) -> Self { let __src: Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__> = Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ { extents_: self.extents_.clone(), })); layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__::copy_from(__src.as_pointer()) } }
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
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_int__::new_1({ exts.as_pointer() }, {
        <not_extents_constructible_tag>::default()
    });
}
pub fn construct_mapping_25(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_26(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::new_1(
        { exts.as_pointer() },
        { <not_extents_constructible_tag>::default() },
    );
}pub fn construct_mapping_27 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_28 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_29 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__{
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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct move_counted_handle_const_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl move_counted_handle_const_int_ {
    pub fn move_from(other: Ptr<move_counted_handle_const_int_>) -> Self {
        let __this: Value<move_counted_handle_const_int_> = Rc::new(RefCell::new(Self {
            ptr: Ptr::<()>::null(),
        }));
        let this: Ptr<move_counted_handle_const_int_> = __this.as_pointer();
        field!(this, ptr).write({ other.with(|__s| __s.ptr.clone()) });
        {
            ({ move_counted_handle_const_int_::move_counter() }).with_mut(|__v| __v.postfix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
    pub fn move_counter() -> Ptr<()> {
        return c_30.with(|v| v.as_pointer());
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct move_counted_handle_int_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<()>,
}
impl move_counted_handle_int_ {
    pub fn move_from(other: Ptr<move_counted_handle_int_>) -> Self {
        let __this: Value<move_counted_handle_int_> = Rc::new(RefCell::new(Self {
            ptr: Ptr::<()>::null(),
        }));
        let this: Ptr<move_counted_handle_int_> = __this.as_pointer();
        field!(this, ptr).write({ other.with(|__s| __s.ptr.clone()) });
        {
            ({ move_counted_handle_int_::move_counter() }).with_mut(|__v| __v.postfix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_1(mut ptr_: Ptr<()>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
    pub fn move_counter() -> Ptr<()> {
        return c_31.with(|v| v.as_pointer());
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
        return c_32.with(|v| v.as_pointer());
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct move_counted_handle_MinimalElementType_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<MinimalElementType>,
}
impl move_counted_handle_MinimalElementType_ {
    pub fn move_from(other: Ptr<move_counted_handle_MinimalElementType_>) -> Self {
        let __this: Value<move_counted_handle_MinimalElementType_> = Rc::new(RefCell::new(Self {
            ptr: Ptr::<MinimalElementType>::null(),
        }));
        let this: Ptr<move_counted_handle_MinimalElementType_> = __this.as_pointer();
        field!(this, ptr).write({ other.with(|__s| __s.ptr.clone()) });
        {
            ({ move_counted_handle_MinimalElementType_::move_counter() })
                .with_mut(|__v| __v.postfix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_1(mut ptr_: Ptr<MinimalElementType>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
    pub fn move_counter() -> Ptr<()> {
        return c_33.with(|v| v.as_pointer());
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, Default)]
#[byte_size(8)]
pub struct move_counted_handle_const_MinimalElementType_ {
    #[offset(0)]
    #[byte_size(8)]
    pub ptr: Ptr<MinimalElementType>,
}
impl move_counted_handle_const_MinimalElementType_ {
    pub fn move_from(other: Ptr<move_counted_handle_const_MinimalElementType_>) -> Self {
        let __this: Value<move_counted_handle_const_MinimalElementType_> =
            Rc::new(RefCell::new(Self {
                ptr: Ptr::<MinimalElementType>::null(),
            }));
        let this: Ptr<move_counted_handle_const_MinimalElementType_> = __this.as_pointer();
        field!(this, ptr).write({ other.with(|__s| __s.ptr.clone()) });
        {
            ({ move_counted_handle_const_MinimalElementType_::move_counter() })
                .with_mut(|__v| __v.postfix_inc());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_1(mut ptr_: Ptr<MinimalElementType>) -> Self {
        Self {
            ptr: (ptr_).clone(),
        }
    }
    pub fn move_counter() -> Ptr<()> {
        return c_34.with(|v| v.as_pointer());
    }
}
thread_local!(
    static c_30: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_31: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_32: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_33: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static c_34: Value<()> = Rc::new(RefCell::new(0_()));
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
pub fn test_mdspan_types_35(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_36(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_37(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_38(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_39(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_40(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_41(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_42(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_43(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_44(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_45(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_46(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_47(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_48(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_49(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_50(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_51(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_52(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_53(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_54(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_55(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_56(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_57(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_58(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_59(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_60(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_61(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_62(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_63(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_64(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_65(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_66(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_67(
    handle: Ptr<move_counted_handle_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_68(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_69(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_70(
    handle: Ptr<move_counted_handle_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_71(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_72(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_73(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_74(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_75(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_76(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_77(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_78(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_79(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_80(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_81(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_82(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_83(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_84(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_85(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_86(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_87(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_88(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_89(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_90(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_91(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_92(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_93(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_94(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_95(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_96(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_97(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_98(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_99(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_100(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_101(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_102(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_103(
    handle: Ptr<move_counted_handle_const_int_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_104(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_105(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_106(
    handle: Ptr<move_counted_handle_const_int_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_107(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_108(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_109(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_110(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_111(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_112(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_113(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_114(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_115(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_116(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_117(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_118(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_119(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_120(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_121(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_122(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_123(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_124(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_125(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_126(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_127(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_128(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_129(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_130(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_131(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_132(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_133(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_134(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_135(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_136(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_137(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_138(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_139(
    handle: Ptr<no_default_ctor_handle_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_140(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_141(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_142(
    handle: Ptr<no_default_ctor_handle_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_143(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_144(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_145(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_146(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_147(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_148(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_149(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_150(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_151(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_152(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_153(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_154(handle: Ptr<Ptr<()>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_155(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_156(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_157(
    handle: Ptr<Ptr<()>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_158(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_159(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_160(
    handle: Ptr<Ptr<()>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_161(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_162(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_163(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_164(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_165(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_166(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_167(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_168(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_169(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_170(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_171(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_172(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_173(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_174(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_175(
    handle: Ptr<move_counted_handle_const_double_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_176(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_177(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_178(
    handle: Ptr<move_counted_handle_const_double_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_179(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_180(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_181(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_182(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_183(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_184(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_185(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_186(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_187(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_188(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_189(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_190(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_191(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_192(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_193(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_194(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_195(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_196(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_197(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_198(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_199(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_200(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_201(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_202(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_203(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_204(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_205(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_206(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_207(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_208(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_209(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_210(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_211(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_212(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_213(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_214(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_215(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_216(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_217(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_218(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_219(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_220(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_221(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_222(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_223(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_224(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_225(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_226(handle: Ptr<Ptr<MinimalElementType>>, map: Ptr<()>, acc: Ptr<()>) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_227(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_228(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_229(
    handle: Ptr<Ptr<MinimalElementType>>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_230(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_231(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_232(
    handle: Ptr<Ptr<MinimalElementType>>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<()>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_233(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_234(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_235(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_236(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_237(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_238(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_239(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_240(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_241(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_242(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_243(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_244(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_245(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_246(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_247(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_248(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_249(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn test_mdspan_types_250(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    map : Ptr< layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__  >,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let m_org: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m_copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {}
    {
        unimplemented!();
    }
    {}
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<MDS> ==\n                (std::is_trivially_move_assignable_v<H> && std::is_trivially_move_assignable_v<M> &&\n                 std::is_trivially_move_assignable_v<A>)"
    );;
}
pub fn mixin_extents_251(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_35(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_36(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_37(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_38(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_39(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_40(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_253(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_41(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_42(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_43(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_44(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_45(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_46(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_254(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_47(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_48(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_49(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_50(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_51(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_52(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_255(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_53(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_54(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_55(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_56(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_57(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_58(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_256(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_59(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_60(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_61(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_62(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_63(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_64(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_257(
    handle: Ptr<move_counted_handle_int_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_int_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_65(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_66(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_67(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_68(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_69(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        test_mdspan_types_70(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_258(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_71(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_72(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_73(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_74(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_75(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_76(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_259(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_77(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_78(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_79(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_80(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_81(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_82(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_260(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_83(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_84(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_85(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_86(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_87(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_88(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_261(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_89(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_90(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_91(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_92(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_93(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_94(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_262(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_95(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_96(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_97(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_98(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_99(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_100(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_263(
    handle: Ptr<move_counted_handle_const_int_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_101(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_102(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_103(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_104(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_105(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        test_mdspan_types_106(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_264(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_107(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_108(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_109(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_110(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_111(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_112(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_265(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_113(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_114(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_115(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_116(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_117(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_118(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_266(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_119(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_120(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_121(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_122(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_123(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_124(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_267(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_125(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_126(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_127(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_128(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_129(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_130(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_268(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_131(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_132(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_133(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_134(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_135(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_136(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_269(
    handle: Ptr<no_default_ctor_handle_double_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_double_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_137(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_138(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_139(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_140(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_141(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        test_mdspan_types_142(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_270(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_143(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_144(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_145(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_146(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_147(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_148(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_271(handle: Ptr<Ptr<()>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_149(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_150(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_151(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_152(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_153(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_154(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_272(
    handle: Ptr<Ptr<()>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_155(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_156(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_157(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_158(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_159(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_160(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_273(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_161(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_162(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_163(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_164(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_165(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_166(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_274(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_167(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_168(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_169(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_170(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_171(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_172(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_275(
    handle: Ptr<move_counted_handle_const_double_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_173(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_174(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_175(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_176(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_177(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        test_mdspan_types_178(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_276(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_179(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_180(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_181(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_182(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_183(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_184(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_277(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_185(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_186(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_187(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_188(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_189(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_190(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_278(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_191(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_192(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_193(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_194(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_195(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_196(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_279(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_197(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_198(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_199(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_200(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_201(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_202(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_280(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_203(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_204(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_205(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_206(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_207(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_208(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_281(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_209(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_210(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_211(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_212(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_213(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        test_mdspan_types_214(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_282(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_215(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_216(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_217(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_218(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_219(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_220(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_283(handle: Ptr<Ptr<MinimalElementType>>, layout: Ptr<()>, acc: Ptr<()>) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_221(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_222(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_223(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_224(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_225(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_226(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_284(
    handle: Ptr<Ptr<MinimalElementType>>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<()>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_227(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_228(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_229(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_230(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_231(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<()> = (acc).clone();
        test_mdspan_types_232(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_285(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_12(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_233(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_13(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_234(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_14(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_235(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_15(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_236(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_16(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_237(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_17(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_238(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_286(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<()>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_18(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_239(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_19(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_240(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_20(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_241(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_21(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_242(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_22(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_243(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<()> = Rc::new(RefCell::new(
            ({ construct_mapping_23(unimplemented!(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_244(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_extents_287(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    layout: Ptr<layout_wrapping_integral_4_>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    let mut D: () = dynamic_extent_252.with(|rc| *rc.borrow());
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_int__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_24((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_245(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<
            layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            ({ construct_mapping_25((*layout.upgrade().deref()).clone(), unimplemented!()) }),
        ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_246(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__> =
            Rc::new(RefCell::new(
                ({ construct_mapping_26((*layout.upgrade().deref()).clone(), unimplemented!()) }),
            ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_247(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_long__18446744073709551615__4__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_27 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_248(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__7__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_28 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_249(_handle, _map.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _map : Value<layout_wrapping_integral_4__mapping_std_extents_long__18446744073709551615__7__18446744073709551615__4__18446744073709551615__18446744073709551615__ > = Rc::new(RefCell::new(( { construct_mapping_29 ( ((*layout.upgrade().deref()) ).clone()  , unimplemented!()  , ) } )  ));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        test_mdspan_types_250(_handle, _map.as_pointer(), _acc)
    });
}
pub fn mixin_layout_288(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<std::layout_left::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_251(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_253(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_assignable_v<layout_wrapping_integral<4>::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_254(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_289(handle: Ptr<move_counted_handle_int_>, acc: Ptr<checked_accessor_int_>) {
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<std::layout_left::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_255(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_256(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_assignable_v<layout_wrapping_integral<4>::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_int_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_int_> = (acc).clone();
        mixin_extents_257(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_290(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<std::layout_left::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_258(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_259(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_assignable_v<layout_wrapping_integral<4>::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_260(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_291(
    handle: Ptr<move_counted_handle_const_int_>,
    acc: Ptr<checked_accessor_const_int_>,
) {
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<std::layout_left::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_261(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_262(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_assignable_v<layout_wrapping_integral<4>::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_int_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_const_int_> = (acc).clone();
        mixin_extents_263(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_292(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<std::layout_left::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_264(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_265(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_assignable_v<layout_wrapping_integral<4>::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_266(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_293(
    handle: Ptr<no_default_ctor_handle_double_>,
    acc: Ptr<checked_accessor_double_>,
) {
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<std::layout_left::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_267(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_268(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_assignable_v<layout_wrapping_integral<4>::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<no_default_ctor_handle_double_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_double_> = (acc).clone();
        mixin_extents_269(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_294(handle: Ptr<Ptr<()>>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<std::layout_left::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_270(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_271(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_assignable_v<layout_wrapping_integral<4>::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<Ptr<()>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_272(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_295(
    handle: Ptr<move_counted_handle_const_double_>,
    acc: Ptr<checked_accessor_const_double_>,
) {
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<std::layout_left::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_273(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_274(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_assignable_v<layout_wrapping_integral<4>::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_double_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_const_double_> = (acc).clone();
        mixin_extents_275(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_296(handle: Ptr<Ptr<MinimalElementType>>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<std::layout_left::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_276(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_277(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_assignable_v<layout_wrapping_integral<4>::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_278(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_297(
    handle: Ptr<move_counted_handle_MinimalElementType_>,
    acc: Ptr<checked_accessor_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<std::layout_left::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_279(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_280(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_assignable_v<layout_wrapping_integral<4>::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_MinimalElementType_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_MinimalElementType_> = (acc).clone();
        mixin_extents_281(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_298(handle: Ptr<Ptr<MinimalElementType>>, acc: Ptr<()>) {
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<std::layout_left::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_282(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_283(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_assignable_v<layout_wrapping_integral<4>::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<Ptr<MinimalElementType>> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<()> = (acc).clone();
        mixin_extents_284(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_layout_299(
    handle: Ptr<move_counted_handle_const_MinimalElementType_>,
    acc: Ptr<checked_accessor_const_MinimalElementType_>,
) {
    const _: () = assert!(
        true,
        "std::is_trivially_move_assignable_v<std::layout_left::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_285(_handle, _layout.as_pointer(), _acc)
    });
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_286(_handle, _layout.as_pointer(), _acc)
    });
    const _: () = assert!(
        true,
        "!std::is_trivially_move_assignable_v<layout_wrapping_integral<4>::mapping<std::extents<int>>>"
    );;
    ({
        let _handle: Ptr<move_counted_handle_const_MinimalElementType_> = (handle).clone();
        let _layout: Value<layout_wrapping_integral_4_> =
            Rc::new(RefCell::new(<layout_wrapping_integral_4_>::default()));
        let _acc: Ptr<checked_accessor_const_MinimalElementType_> = (acc).clone();
        mixin_extents_287(_handle, _layout.as_pointer(), _acc)
    });
}
pub fn mixin_accessor_300() {
    let elements: Value<ElementPool_int__1024_> =
        Rc::new(RefCell::new(ElementPool_int__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    const _: () = assert!(
        true,
        "std::is_trivially_copyable_v<std::default_accessor<T>>"
    );;
    const _: () = assert!(
        true,
        "std::is_trivially_copyable_v<typename std::default_accessor<T>::data_handle_type>"
    );;
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_int__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_288(_handle.as_pointer(), _acc.as_pointer())
    });
    let acc: Value<checked_accessor_int_> =
        Rc::new(RefCell::new(checked_accessor_int_::new({ 1024_() })));
    const _: () = assert!(
        true,
        "noexcept(checked_accessor<T>(acc)) != std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_int_> =
            Rc::new(RefCell::new(move_counted_handle_int_::new_1({
                ({ ElementPool_int__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        mixin_layout_289(_handle.as_pointer(), acc.as_pointer())
    });
}
pub fn mixin_accessor_301() {
    let elements: Value<ElementPool_const_int__1024_> =
        Rc::new(RefCell::new(ElementPool_const_int__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    const _: () = assert!(
        true,
        "std::is_trivially_copyable_v<std::default_accessor<T>>"
    );;
    const _: () = assert!(
        true,
        "std::is_trivially_copyable_v<typename std::default_accessor<T>::data_handle_type>"
    );;
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_const_int__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_290(_handle.as_pointer(), _acc.as_pointer())
    });
    let acc: Value<checked_accessor_const_int_> =
        Rc::new(RefCell::new(checked_accessor_const_int_::new({ 1024_() })));
    const _: () = assert!(
        true,
        "noexcept(checked_accessor<T>(acc)) != std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_const_int_> =
            Rc::new(RefCell::new(move_counted_handle_const_int_::new_1({
                ({ ElementPool_const_int__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        mixin_layout_291(_handle.as_pointer(), acc.as_pointer())
    });
}
pub fn mixin_accessor_302() {
    let elements: Value<ElementPool_double__1024_> =
        Rc::new(RefCell::new(ElementPool_double__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    const _: () = assert!(
        true,
        "std::is_trivially_copyable_v<std::default_accessor<T>>"
    );;
    const _: () = assert!(
        true,
        "std::is_trivially_copyable_v<typename std::default_accessor<T>::data_handle_type>"
    );;
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_double__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_292(_handle.as_pointer(), _acc.as_pointer())
    });
    let acc: Value<checked_accessor_double_> =
        Rc::new(RefCell::new(checked_accessor_double_::new({ 1024_() })));
    const _: () = assert!(
        true,
        "noexcept(checked_accessor<T>(acc)) != std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<no_default_ctor_handle_double_> =
            Rc::new(RefCell::new(no_default_ctor_handle_double_::new_1({
                ({ ElementPool_double__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        mixin_layout_293(_handle.as_pointer(), acc.as_pointer())
    });
}
pub fn mixin_accessor_303() {
    let elements: Value<ElementPool_const_double__1024_> =
        Rc::new(RefCell::new(ElementPool_const_double__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    const _: () = assert!(
        true,
        "std::is_trivially_copyable_v<std::default_accessor<T>>"
    );;
    const _: () = assert!(
        true,
        "std::is_trivially_copyable_v<typename std::default_accessor<T>::data_handle_type>"
    );;
    ({
        let _handle: Value<Ptr<()>> = Rc::new(RefCell::new(
            ({ ElementPool_const_double__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_294(_handle.as_pointer(), _acc.as_pointer())
    });
    let acc: Value<checked_accessor_const_double_> =
        Rc::new(RefCell::new(checked_accessor_const_double_::new_2({
            1024_()
        })));
    const _: () = assert!(
        true,
        "noexcept(checked_accessor<T>(acc)) != std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_const_double_> =
            Rc::new(RefCell::new(move_counted_handle_const_double_::new_1({
                ({ ElementPool_const_double__1024_Impl::get_ptr(&elements.as_pointer()) })
            })));
        mixin_layout_295(_handle.as_pointer(), acc.as_pointer())
    });
}
pub fn mixin_accessor_304() {
    let elements: Value<ElementPool_MinimalElementType__1024_> =
        Rc::new(RefCell::new(ElementPool_MinimalElementType__1024_::new()));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    const _: () = assert!(
        true,
        "std::is_trivially_copyable_v<std::default_accessor<T>>"
    );;
    const _: () = assert!(
        true,
        "std::is_trivially_copyable_v<typename std::default_accessor<T>::data_handle_type>"
    );;
    ({
        let _handle: Value<Ptr<MinimalElementType>> = Rc::new(RefCell::new(
            ({ ElementPool_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_296(_handle.as_pointer(), _acc.as_pointer())
    });
    let acc: Value<checked_accessor_MinimalElementType_> =
        Rc::new(RefCell::new(checked_accessor_MinimalElementType_::new({
            1024_()
        })));
    const _: () = assert!(
        true,
        "noexcept(checked_accessor<T>(acc)) != std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_MinimalElementType_> = Rc::new(RefCell::new(
            move_counted_handle_MinimalElementType_::new_1({
                ({ ElementPool_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) })
            }),
        ));
        mixin_layout_297(_handle.as_pointer(), acc.as_pointer())
    });
}
pub fn mixin_accessor_305() {
    let elements: Value<ElementPool_const_MinimalElementType__1024_> = Rc::new(RefCell::new(
        ElementPool_const_MinimalElementType__1024_::new(),
    ));
    let _dtor_elements = ScopedDestructor::new(&elements, |__p| __p.destructor());
    const _: () = assert!(
        true,
        "std::is_trivially_copyable_v<std::default_accessor<T>>"
    );;
    const _: () = assert!(
        true,
        "std::is_trivially_copyable_v<typename std::default_accessor<T>::data_handle_type>"
    );;
    ({
        let _handle: Value<Ptr<MinimalElementType>> = Rc::new(RefCell::new(
            ({ ElementPool_const_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer()) }),
        ));
        let _acc: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        mixin_layout_298(_handle.as_pointer(), _acc.as_pointer())
    });
    let acc: Value<checked_accessor_const_MinimalElementType_> = Rc::new(RefCell::new(
        checked_accessor_const_MinimalElementType_::new({ 1024_() }),
    ));
    const _: () = assert!(
        true,
        "noexcept(checked_accessor<T>(acc)) != std::is_same_v<T, const double>"
    );;
    ({
        let _handle: Value<move_counted_handle_const_MinimalElementType_> = Rc::new(RefCell::new(
            move_counted_handle_const_MinimalElementType_::new_1({
                ({
                    ElementPool_const_MinimalElementType__1024_Impl::get_ptr(&elements.as_pointer())
                })
            }),
        ));
        mixin_layout_299(_handle.as_pointer(), acc.as_pointer())
    });
}
pub fn test_306() -> () {
    ({ mixin_accessor_300() });
    ({ mixin_accessor_301() });
    ({ mixin_accessor_302() });
    ({ mixin_accessor_303() });
    ({ mixin_accessor_304() });
    ({ mixin_accessor_305() });
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
    ({ test_306() });
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
impl layout_wrapping_integral_4__mapping_std_extents_int__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
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
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
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
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
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
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
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
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
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
}
