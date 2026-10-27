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
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntConfig_false__false__true__true_ {
    #[offset(0)]
    pub val: (),
}
impl IntConfig_false__false__true__true_ {
    pub fn new(mut val_: ()) -> Self {
        Self { val: val_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntConfig_false__true__true__true_ {
    #[offset(0)]
    pub val: (),
}
impl IntConfig_false__true__true__true_ {
    pub fn new(mut val_: ()) -> Self {
        Self { val: val_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntConfig_false__true__false__true_ {
    #[offset(0)]
    pub val: (),
}
impl IntConfig_false__true__false__true_ {
    pub fn new(mut val_: ()) -> Self {
        Self { val: val_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntConfig_true__false__false__true_ {
    #[offset(0)]
    pub val: (),
}
impl IntConfig_true__false__false__true_ {
    pub fn new(mut val_: ()) -> Self {
        Self { val: val_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntConfig_true__false__true__false_ {
    #[offset(0)]
    pub val: (),
}
impl IntConfig_true__false__true__false_ {
    pub fn new(mut val_: ()) -> Self {
        Self { val: val_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntConfig_true__true__false__true_ {
    #[offset(0)]
    pub val: (),
}
impl IntConfig_true__true__false__true_ {
    pub fn new(mut val_: ()) -> Self {
        Self { val: val_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntConfig_true__true__true__true_ {
    #[offset(0)]
    pub val: (),
}
impl IntConfig_true__true__true__true_ {
    pub fn new(mut val_: ()) -> Self {
        Self { val: val_ }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct not_extents_constructible_tag {}
thread_local!(
    static Wrap_0: Value<()> = Rc::new(RefCell::new(4_()));
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
    static Wrap_1: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(4)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__ {
    pub fn copy_from(
        other: Ptr<
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__,
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
impl Clone
    for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__
{
    fn clone(&self) -> Self {
        let __src: Value<
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__,
        > = Rc::new(RefCell::new(
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__ {
                extents_: self.extents_.clone(),
            },
        ));
        layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__::copy_from(__src.as_pointer())
    }
}
impl Default
    for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__
{
    fn default() -> Self {
        {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_2: Value<()> = Rc::new(RefCell::new(4_()));
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
    static Wrap_3: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
    pub fn copy_from(
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__>,
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
impl Clone for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
    fn clone(&self) -> Self {
        let __src: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__> =
            Rc::new(RefCell::new(
                layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
                    extents_: self.extents_.clone(),
                },
            ));
        layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__::copy_from(
            __src.as_pointer(),
        )
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
    fn default() -> Self {
        {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_4: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(4)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { pub fn copy_from ( other : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  > , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl Clone for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { fn clone(&self) -> Self { let __src: Value<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__> = Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { extents_: self.extents_.clone(), })); layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__::copy_from(__src.as_pointer()) } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_5: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(4)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
    pub fn copy_from(
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__>,
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
impl Clone for layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
    fn clone(&self) -> Self {
        let __src: Value<
            layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__,
        > = Rc::new(RefCell::new(
            layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
                extents_: self.extents_.clone(),
            },
        ));
        layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__::copy_from(
            __src.as_pointer(),
        )
    }
}
impl Default for layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
    fn default() -> Self {
        {
            layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_6: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__ {
    #[offset(0)]
    #[byte_size(1)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__ {
    pub fn copy_from(
        other: Ptr<
            layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__,
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
impl Clone
    for layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__
{
    fn clone(&self) -> Self {
        let __src: Value<
            layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__,
        > = Rc::new(RefCell::new(
            layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__ {
                extents_: self.extents_.clone(),
            },
        ));
        layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__::copy_from(__src.as_pointer())
    }
}
impl Default
    for layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__
{
    fn default() -> Self {
        {
            layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__ {
                extents_: unimplemented!(),
            }
        }
    }
}
thread_local!(
    static Wrap_7: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(2)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ { pub fn copy_from ( other : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__  > , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl Clone for layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ { fn clone(&self) -> Self { let __src: Value<layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__> = Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ { extents_: self.extents_.clone(), })); layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__::copy_from(__src.as_pointer()) } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
thread_local!(
    static Wrap_8: Value<()> = Rc::new(RefCell::new(4_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__
{
    #[offset(0)]
    #[byte_size(8)]
    extents_: (),
}
impl layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ { pub fn copy_from ( other : Ptr< layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__  > , ) -> Self { Self { extents_ : unimplemented!()  , }  } pub fn new_1 ( ext : Ptr< ()  > , mut _a1 : not_extents_constructible_tag  , ) -> Self { Self { extents_ : unimplemented!()  , }  } }
impl Clone for layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ { fn clone(&self) -> Self { let __src: Value<layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__> = Rc::new(RefCell::new(layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ { extents_: self.extents_.clone(), })); layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__::copy_from(__src.as_pointer()) } }
impl Default for layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ { fn default() -> Self { { layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ { extents_ : unimplemented!() , } } } }
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct layout_wrapping_integral_4_ {}
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
pub fn construct_mapping_30(_a0: (), exts: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return unimplemented!();
}
pub fn construct_mapping_31(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_int__::new_1({ exts.as_pointer() }, {
        <not_extents_constructible_tag>::default()
    });
}
pub fn construct_mapping_32(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_33(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__::new_1(
        { exts.as_pointer() },
        { <not_extents_constructible_tag>::default() },
    );
}
pub fn construct_mapping_34(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__::new_1(
        { exts.as_pointer() },
        { <not_extents_constructible_tag>::default() },
    );
}pub fn construct_mapping_35 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_36(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__::new_1(
        { exts.as_pointer() },
        { <not_extents_constructible_tag>::default() },
    );
}
pub fn construct_mapping_37(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__ {
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}pub fn construct_mapping_38 ( mut _a0 : layout_wrapping_integral_4_  , exts : ()  , ) -> layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
pub fn construct_mapping_39(
    mut _a0: layout_wrapping_integral_4_,
    exts: (),
) -> layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__
{
    let exts: Value<()> = Rc::new(RefCell::new(exts));
    return layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__ :: new_1 ( { exts .as_pointer()  } , { <not_extents_constructible_tag  >::default()  } , )    ;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct always_convertible_layout {}
pub fn check_operator_constraints_40(m: (), mut idxs: ()) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_41(m: (), mut idxs: IntType) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_42(m: (), mut idxs_1: (), mut idxs_2: ()) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_43(m: (), mut idxs_1: (), mut idxs_2: IntType) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_44(
    m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_false__true__true__true_,
) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_45(
    m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_false__true__false__true_,
) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_46(
    m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_true__false__true__false_,
) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_47(m: (), idxs: ()) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let idxs: Value<()> = Rc::new(RefCell::new(idxs));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_48(m: (), idxs: ()) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let idxs: Value<()> = Rc::new(RefCell::new(idxs));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_49(m: (), mut idxs: ()) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_50(m: (), mut idxs: IntType) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_51(m: (), mut idxs_1: (), mut idxs_2: ()) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_52(m: (), mut idxs_1: (), mut idxs_2: IntType) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_53(
    m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_false__true__true__true_,
) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_54(
    m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_false__true__false__true_,
) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_55(
    m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_true__false__true__false_,
) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_56(m: (), mut idxs: ()) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_57(m: (), mut idxs: IntType) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_58(m: (), mut idxs_1: (), mut idxs_2: ()) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_59(m: (), mut idxs_1: (), mut idxs_2: IntType) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_60(
    m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_false__true__true__true_,
) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_61(
    m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_false__true__false__true_,
) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_62(
    m: (),
    mut idxs_1: (),
    mut idxs_2: IntConfig_true__false__true__false_,
) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_63(_a0: (), mut _a1: (), mut _a2: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_64(_a0: (), mut _a1: IntType) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_65(_a0: (), mut _a1: IntType) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_66(
    _a0: (),
    mut _a1: (),
    mut _a2: IntConfig_false__false__true__true_,
) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_67(
    _a0: (),
    mut _a1: (),
    mut _a2: IntConfig_true__false__false__true_,
) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_68(_a0: (), _a1: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return false;
}
pub fn check_operator_constraints_69(_a0: (), _a1: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return false;
}
pub fn check_operator_constraints_70(_a0: (), _a1: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return false;
}
pub fn check_operator_constraints_71(_a0: (), _a1: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return false;
}
pub fn check_operator_constraints_72(_a0: (), _a1: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    let _a1: Value<()> = Rc::new(RefCell::new(_a1));
    return false;
}
pub fn check_operator_constraints_73(_a0: (), mut _a1: (), mut _a2: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_74(_a0: (), mut _a1: IntType) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_75(_a0: (), mut _a1: IntType) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_76(
    _a0: (),
    mut _a1: (),
    mut _a2: IntConfig_false__false__true__true_,
) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_77(
    _a0: (),
    mut _a1: (),
    mut _a2: IntConfig_true__false__false__true_,
) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_78(_a0: (), mut _a1: (), mut _a2: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_79(_a0: (), mut _a1: IntType) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_80(_a0: (), mut _a1: IntType) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_81(
    _a0: (),
    mut _a1: (),
    mut _a2: IntConfig_false__false__true__true_,
) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_82(
    _a0: (),
    mut _a1: (),
    mut _a2: IntConfig_true__false__false__true_,
) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn iterate_83(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_84(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_85(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_85(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_86(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_87(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_87(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_88(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_89(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_89(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_90(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_90(mds: (), mut args_1: (), mut args_2: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_91(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_92(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_92(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_93(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_93(mds: (), mut args_1: (), mut args_2: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_94(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_94(mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_95(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_95(mds: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_96(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_97(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_98(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_98(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_99(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_100(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_100(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_101(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_102(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_102(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_103(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_103(mds: (), mut args_1: (), mut args_2: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_104(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_105(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_105(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_106(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_106(mds: (), mut args_1: (), mut args_2: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_107(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_107(mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_108(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_108(mds: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_109(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_110(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_111(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_111(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_112(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_113(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_113(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_114(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_115(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_115(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_116(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_116(mds: (), mut args_1: (), mut args_2: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_117(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_118(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_118(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_119(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_119(mds: (), mut args_1: (), mut args_2: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_120(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_120(mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_121(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_121(mds: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_122(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_123(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_123(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_124(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_124(mds: (), mut args_1: (), mut args_2: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_125(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_125(mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_126(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_126(mds: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_127(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_128(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_128(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_129(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_129(mds: (), mut args_1: (), mut args_2: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_130(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_130(mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_131(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_131(mds: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_132(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_133(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_133(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_134(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_134(mds: (), mut args_1: (), mut args_2: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_135(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_135(mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_136(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_136(mds: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn iterate_137(mds: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_138(unimplemented!(), i) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_138(mds: (), mut args: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_139(unimplemented!(), i, args) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_139(mds: (), mut args_1: (), mut args_2: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_140(unimplemented!(), i, args_1, args_2) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_140(mds: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({ iterate_141(unimplemented!(), i, args_1, args_2, args_3) });
            i.postfix_inc();
        }
    }
}
pub fn iterate_141(mds: (), mut args_1: (), mut args_2: (), mut args_3: (), mut args_4: ()) {
    let mds: Value<()> = Rc::new(RefCell::new(mds));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        let mut ptr1: Ptr<()> = (unimplemented!());
        let mut ptr2: Ptr<()> = (unimplemented!());
        unimplemented!();
        let args_arr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ptr3: Ptr<()> = (unimplemented!());
        unimplemented!();
        let mut ptr4: Ptr<()> = (unimplemented!());
        unimplemented!();
    }
}
pub fn test_iteration_142(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_83(unimplemented!()) });
}
pub fn test_iteration_143(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_84(unimplemented!()) });
}
pub fn test_iteration_144(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_86(unimplemented!()) });
}
pub fn test_iteration_145(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_88(unimplemented!()) });
}
pub fn test_iteration_146(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_91(unimplemented!()) });
}
pub fn test_iteration_147(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_96(unimplemented!()) });
}
pub fn test_iteration_148(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_97(unimplemented!()) });
}
pub fn test_iteration_149(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_99(unimplemented!()) });
}
pub fn test_iteration_150(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_101(unimplemented!()) });
}
pub fn test_iteration_151(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_104(unimplemented!()) });
}
pub fn test_iteration_152(mut m: layout_wrapping_integral_4__mapping_std_extents_int__) {
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_109(unimplemented!()) });
}
pub fn test_iteration_153(
    mut m: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__,
) {
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_110(unimplemented!()) });
}
pub fn test_iteration_154(
    mut m: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__,
) {
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_112(unimplemented!()) });
}
pub fn test_iteration_155(
    mut m: layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__,
) {
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_114(unimplemented!()) });
}
pub fn test_iteration_156(
    mut m : layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__,
) {
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_117(unimplemented!()) });
}
pub fn test_iteration_157(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_122(unimplemented!()) });
}
pub fn test_iteration_158(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_127(unimplemented!()) });
}
pub fn test_iteration_159(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_132(unimplemented!()) });
}
pub fn test_iteration_160(m: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mds: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ iterate_137(unimplemented!()) });
}
pub fn test_layout_161() {
    let mut D: () = dynamic_extent_162.with(|rc| *rc.borrow());
    ({ test_iteration_142(({ construct_mapping_9(unimplemented!(), unimplemented!()) })) });
    ({ test_iteration_143(({ construct_mapping_10(unimplemented!(), unimplemented!()) })) });
    ({ test_iteration_143(({ construct_mapping_10(unimplemented!(), unimplemented!()) })) });
    ({ test_iteration_144(({ construct_mapping_11(unimplemented!(), unimplemented!()) })) });
    ({ test_iteration_145(({ construct_mapping_12(unimplemented!(), unimplemented!()) })) });
    ({ test_iteration_146(({ construct_mapping_13(unimplemented!(), unimplemented!()) })) });
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..1).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    const _: () = assert!(
        true,
        "check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), 0)"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), 0, 0)"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned char, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned char, D, D>(1, 1))), int(0), size_t(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))), unsigned(0), IntType(0))"
    );;
    let mut t: () = true;
    let mut o: () = false;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, o, t, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, t, t, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, t, o, t>(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<t, o, o, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<t, o, t, o>(0))"
    );;
    {
        const _: () = assert!(
            true,
            "!check_operator_constraints(\n        std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), std::array{{IntConfig<o, o, t, t>(0)}})"
        );;
        const _: () = assert!(
            true,
            "!check_operator_constraints(\n        std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), std::array{{IntConfig<o, t, t, t>(0)}})"
        );;
        const _: () = assert!(
            true,
            "!check_operator_constraints(\n        std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), std::array{{IntConfig<t, o, o, t>(0)}})"
        );;
        const _: () = assert!(
            true,
            "!check_operator_constraints(\n        std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), std::array{{IntConfig<t, t, o, t>(0)}})"
        );;
        const _: () = assert!(
            true,
            "check_operator_constraints(\n        std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), std::array{{IntConfig<t, o, t, o>(0)}})"
        );;
        const _: () = assert!(
            true,
            "check_operator_constraints(\n        std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), std::array{{IntConfig<t, t, t, t>(0)}})"
        );;
        {
            let idx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
        }
        {
            let idx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
        }
        {
            let idx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
        }
        {
            let idx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
        }
        {
            let idx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
        }
        {
            let idx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
        }
    }
}
pub fn test_layout_163() {
    let mut D: () = dynamic_extent_162.with(|rc| *rc.borrow());
    ({ test_iteration_147(({ construct_mapping_20(unimplemented!(), unimplemented!()) })) });
    ({ test_iteration_148(({ construct_mapping_21(unimplemented!(), unimplemented!()) })) });
    ({ test_iteration_148(({ construct_mapping_21(unimplemented!(), unimplemented!()) })) });
    ({ test_iteration_149(({ construct_mapping_22(unimplemented!(), unimplemented!()) })) });
    ({ test_iteration_150(({ construct_mapping_23(unimplemented!(), unimplemented!()) })) });
    ({ test_iteration_151(({ construct_mapping_24(unimplemented!(), unimplemented!()) })) });
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..1).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    const _: () = assert!(
        true,
        "check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), 0)"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), 0, 0)"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned char, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned char, D, D>(1, 1))), int(0), size_t(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))), unsigned(0), IntType(0))"
    );;
    let mut t: () = true;
    let mut o: () = false;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, o, t, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, t, t, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, t, o, t>(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<t, o, o, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<t, o, t, o>(0))"
    );;
    {}
}
pub fn test_layout_164() {
    let mut D: () = dynamic_extent_162.with(|rc| *rc.borrow());
    ({
        test_iteration_152(
            ({ construct_mapping_31(<layout_wrapping_integral_4_>::default(), unimplemented!()) }),
        )
    });
    ({
        test_iteration_153(
            ({ construct_mapping_32(<layout_wrapping_integral_4_>::default(), unimplemented!()) }),
        )
    });
    ({
        test_iteration_153(
            ({ construct_mapping_32(<layout_wrapping_integral_4_>::default(), unimplemented!()) }),
        )
    });
    ({
        test_iteration_154(
            ({ construct_mapping_33(<layout_wrapping_integral_4_>::default(), unimplemented!()) }),
        )
    });
    ({
        test_iteration_155(
            ({ construct_mapping_34(<layout_wrapping_integral_4_>::default(), unimplemented!()) }),
        )
    });
    ({
        test_iteration_156(
            ({ construct_mapping_35(<layout_wrapping_integral_4_>::default(), unimplemented!()) }),
        )
    });
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..1).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    const _: () = assert!(
        true,
        "check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), 0)"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), 0, 0)"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(std::mdspan(data, construct_mapping(Layout(), std::extents<int, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned char, D>(1))), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<unsigned char, D, D>(1, 1))), int(0), size_t(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))), unsigned(0), IntType(0))"
    );;
    let mut t: () = true;
    let mut o: () = false;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, o, t, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, t, t, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<o, t, o, t>(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<t, o, o, t>(0))"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(\n      std::mdspan(data, construct_mapping(Layout(), std::extents<int, D, D>(1, 1))),\n      unsigned(0),\n      IntConfig<t, o, t, o>(0))"
    );;
    {}
}
pub fn test_layout_large_165() {
    let mut D: () = dynamic_extent_162.with(|rc| *rc.borrow());
    ({ test_iteration_157(({ construct_mapping_18(unimplemented!(), unimplemented!()) })) });
    ({ test_iteration_158(({ construct_mapping_19(unimplemented!(), unimplemented!()) })) });
}
pub fn test_layout_large_166() {
    let mut D: () = dynamic_extent_162.with(|rc| *rc.borrow());
    ({ test_iteration_159(({ construct_mapping_29(unimplemented!(), unimplemented!()) })) });
    ({ test_iteration_160(({ construct_mapping_30(unimplemented!(), unimplemented!()) })) });
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct NoCopyIndex {
    #[offset(0)]
    pub val: (),
}
impl NoCopyIndex {
    pub fn new(mut v: ()) -> Self {
        Self { val: v }
    }
}
impl Default for NoCopyIndex {
    fn default() -> Self {
        { NoCopyIndex { val: 0_() } }
    }
}
pub fn test_index_cast_167() {
    let data: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let indices: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RValueInt {}
pub fn test_168() -> () {
    ({ test_layout_161() });
    ({ test_layout_163() });
    ({ test_layout_164() });
    let data: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    &(unimplemented!());
    ({ test_index_cast_167() });
    return true;
}
pub fn test_large_169() -> () {
    ({ test_layout_large_165() });
    ({ test_layout_large_166() });
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
    ({ test_168() });
    const _: () = assert!(true, "test()");;
    ({ test_large_169() });
    return 0_();
}
pub trait IntConfig_false__false__true__true_Impl {
    fn to__2(&self) -> () {
        unimplemented!()
    }
    fn to__3(&self) -> () {
        unimplemented!()
    }
}
impl IntConfig_false__false__true__true_Impl for Ptr<IntConfig_false__false__true__true_> {}
pub trait IntConfig_false__true__false__true_Impl {
    fn to__2(&self) -> ();
    fn to__3(&self) -> () {
        unimplemented!()
    }
}
impl IntConfig_false__true__false__true_Impl for Ptr<IntConfig_false__true__false__true_> {
    fn to__2(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait IntConfig_false__true__true__true_Impl {
    fn to__2(&self) -> ();
    fn to__3(&self) -> () {
        unimplemented!()
    }
}
impl IntConfig_false__true__true__true_Impl for Ptr<IntConfig_false__true__true__true_> {
    fn to__2(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait IntConfig_true__false__false__true_Impl {
    fn to__2(&self) -> () {
        unimplemented!()
    }
    fn to__3(&self) -> () {
        unimplemented!()
    }
}
impl IntConfig_true__false__false__true_Impl for Ptr<IntConfig_true__false__false__true_> {}
pub trait IntConfig_true__false__true__false_Impl {
    fn to__2(&self) -> () {
        unimplemented!()
    }
    fn to__3(&self) -> ();
}
impl IntConfig_true__false__true__false_Impl for Ptr<IntConfig_true__false__true__false_> {
    fn to__3(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait IntConfig_true__true__false__true_Impl {
    fn to__2(&self) -> () {
        unimplemented!()
    }
    fn to__3(&self) -> () {
        unimplemented!()
    }
}
impl IntConfig_true__true__false__true_Impl for Ptr<IntConfig_true__true__false__true_> {}
pub trait IntConfig_true__true__true__true_Impl {
    fn to__2(&self) -> () {
        unimplemented!()
    }
    fn to__3(&self) -> ();
}
impl IntConfig_true__true__true__true_Impl for Ptr<IntConfig_true__true__true__true_> {
    fn to__3(&self) -> () {
        return (*self).with(|__s| __s.val);
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
pub trait NoCopyIndexImpl {
    fn to_(&self) -> ();
}
impl NoCopyIndexImpl for Ptr<NoCopyIndex> {
    fn to_(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait RValueIntImpl {
    fn to_(&self) -> ();
}
impl RValueIntImpl for Ptr<RValueInt> {
    fn to_(&self) -> () {
        return 0_();
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
    fn operator_call_11(&self) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_int__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn operator_call_11(&self) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ({
            lambda!(
                Generic,
                {
                    let this_: Value<Ptr<layout_wrapping_integral_4__mapping_std_extents_int__>> =
                        Rc::new(RefCell::new((*self).clone()));
                    let idx_a: Ptr<()> = idx_a.as_pointer();
                },
                |_a0: ()| -> () {
                    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                    let mut res: () = 0_();
                    unimplemented!();
                    return res;
                }
            )
            .spec::<fn(()) -> ()>(0)
            .call(unimplemented!())
        });
    }
}
pub trait layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__Impl {
    fn copy_assign(
        &self,
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__>,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__> {
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
    fn operator_call_11_int(&self, idx: ()) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn operator_call_11_int(&self, mut idx: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ({
            lambda! ( Generic , { let this_ : Value<Ptr< layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 let __rhs = ( (unimplemented!() .read()) + ( ( if ( unimplemented!()  < Wrap_5 .with(|rc| *rc.borrow()) ) { unimplemented!()   } else { Wrap_5 .with(|rc| *rc.borrow())  } ) * res ) )  ;
 res  = __rhs ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( unimplemented!()  , )
        });
    }
}
pub trait layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__  >{
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
    fn operator_call_11_int_int(&self, idx_0: (), idx_1: ()) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__Impl for Ptr<layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn operator_call_11_int_int ( &self , mut idx_0 : ()  , mut idx_1 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( { lambda! ( Generic , { let this_ : Value<Ptr< layout_wrapping_integral_4__mapping_std_extents_int__18446744073709551615__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( ( if ( unimplemented!()  < Wrap_8 .with(|rc| *rc.borrow()) ) { unimplemented!()   } else { Wrap_8 .with(|rc| *rc.borrow())  } ) * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( ( if ( unimplemented!()  < Wrap_8 .with(|rc| *rc.borrow()) ) { unimplemented!()   } else { Wrap_8 .with(|rc| *rc.borrow())  } ) * res ) )  ;
 res  = __rhs } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( unimplemented!()  , ) } )   ;
 } }
pub trait layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  >{
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
    fn operator_call_11_signed_char_signed_char_signed_char_signed_char(
        &self,
        idx_0: (),
        idx_1: (),
        idx_2: (),
        idx_3: (),
    ) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__Impl for Ptr<layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn operator_call_11_signed_char_signed_char_signed_char_signed_char ( &self , mut idx_0 : ()  , mut idx_1 : ()  , mut idx_2 : ()  , mut idx_3 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( { lambda! ( Generic , { let this_ : Value<Ptr< layout_wrapping_integral_4__mapping_std_extents_signed_char__18446744073709551615__18446744073709551615__18446744073709551615__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( ( if ( unimplemented!()  < Wrap_4 .with(|rc| *rc.borrow()) ) { unimplemented!()   } else { Wrap_4 .with(|rc| *rc.borrow())  } ) * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( ( if ( unimplemented!()  < Wrap_4 .with(|rc| *rc.borrow()) ) { unimplemented!()   } else { Wrap_4 .with(|rc| *rc.borrow())  } ) * res ) )  ;
 res  = __rhs ;
 { let __rhs = ( (unimplemented!() .read()) + ( ( if ( unimplemented!()  < Wrap_4 .with(|rc| *rc.borrow()) ) { unimplemented!()   } else { Wrap_4 .with(|rc| *rc.borrow())  } ) * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( ( if ( unimplemented!()  < Wrap_4 .with(|rc| *rc.borrow()) ) { unimplemented!()   } else { Wrap_4 .with(|rc| *rc.borrow())  } ) * res ) )  ;
 res  = __rhs } } } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( unimplemented!()  , ) } )   ;
 } }
pub trait layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__Impl
{
    fn copy_assign(
        &self,
        other: Ptr<
            layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__,
        >,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__>
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
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
}
pub trait layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__Impl
{
    fn copy_assign ( &self , other : Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__  > , ) -> Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__  >{
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
    fn operator_call_11_unsigned_char_unsigned_char(&self, idx_0: (), idx_1: ()) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__Impl for Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__> {
fn extents ( &self , ) -> Ptr< ()  > { return field_ptr!((* self )  , extents_)   ;
 } fn operator_call_11_unsigned_char_unsigned_char ( &self , mut idx_0 : ()  , mut idx_1 : ()  , ) -> ()  { let idx_a : Value<() > = Rc::new(RefCell::new(unimplemented!() )) ;
  ;
 ;
 return ( { lambda! ( Generic , { let this_ : Value<Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_char__18446744073709551615__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 { let __rhs = ( (unimplemented!() .read()) + ( ( if ( unimplemented!()  < Wrap_7 .with(|rc| *rc.borrow()) ) { unimplemented!()   } else { Wrap_7 .with(|rc| *rc.borrow())  } ) * res ) )  ;
 res  = __rhs ;
 let __rhs = ( (unimplemented!() .read()) + ( ( if ( unimplemented!()  < Wrap_7 .with(|rc| *rc.borrow()) ) { unimplemented!()   } else { Wrap_7 .with(|rc| *rc.borrow())  } ) * res ) )  ;
 res  = __rhs } ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( unimplemented!()  , ) } )   ;
 } }
pub trait layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__Impl {
    fn copy_assign(
        &self,
        other: Ptr<
            layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__,
        >,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__>
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
    fn operator_call_11_unsigned_int(&self, idx: ()) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn operator_call_11_unsigned_int(&self, mut idx: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ({
            lambda! ( Generic , { let this_ : Value<Ptr< layout_wrapping_integral_4__mapping_std_extents_unsigned_int__18446744073709551615__  > > = Rc::new(RefCell::new(((* self ) ).clone())) ;
 let idx_a : Ptr< ()  > = idx_a .as_pointer()  ;
 } , | _a0 : ()  , | -> ()  { let _a0 : Value<() >  = Rc::new(RefCell::new(_a0)) ;
 let mut res : ()  = 0_()  ;
  ;
 ;
 let __rhs = ( (unimplemented!() .read()) ) . wrapping_add ( ( ( if ( unimplemented!()  < Wrap_1 .with(|rc| *rc.borrow()) ) { unimplemented!()   } else { Wrap_1 .with(|rc| *rc.borrow())  } ) ) . wrapping_mul ( res ) )  ;
 res  = __rhs ;
 return res   ;
 } )  .spec::<fn(()  ,) -> ()  >(0) .call ( unimplemented!()  , )
        });
    }
}
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
    fn operator_call_11_unsigned_int(&self, idx: ()) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn operator_call_11_unsigned_int(&self, mut idx: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ({
            lambda!(
                Generic,
                {
                    let this_: Value<
                        Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__>,
                    > = Rc::new(RefCell::new((*self).clone()));
                    let idx_a: Ptr<()> = idx_a.as_pointer();
                },
                |_a0: ()| -> () {
                    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                    let mut res: () = 0_();
                    let __rhs = (unimplemented!().read()).wrapping_add(
                        (if (unimplemented!() < Wrap_2.with(|rc| *rc.borrow())) {
                            unimplemented!()
                        } else {
                            Wrap_2.with(|rc| *rc.borrow())
                        })
                        .wrapping_mul(res),
                    );
                    res = __rhs;
                    return res;
                }
            )
            .spec::<fn(()) -> ()>(0)
            .call(unimplemented!())
        });
    }
}
pub trait layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__Impl {
    fn copy_assign(
        &self,
        other: Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__>,
    ) -> Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__> {
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
    fn operator_call_11_unsigned_int_unsigned_int(&self, idx_0: (), idx_1: ()) -> ();
}
impl layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__Impl
    for Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__>
{
    fn extents(&self) -> Ptr<()> {
        return field_ptr!((*self), extents_);
    }
    fn operator_call_11_unsigned_int_unsigned_int(&self, mut idx_0: (), mut idx_1: ()) -> () {
        let idx_a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        return ({
            lambda!(
                Generic,
                {
                    let this_: Value<
                        Ptr<layout_wrapping_integral_4__mapping_std_extents_unsigned_int__7__8__>,
                    > = Rc::new(RefCell::new((*self).clone()));
                    let idx_a: Ptr<()> = idx_a.as_pointer();
                },
                |_a0: ()| -> () {
                    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
                    let mut res: () = 0_();
                    {
                        let __rhs = (unimplemented!().read()).wrapping_add(
                            (if (unimplemented!() < Wrap_3.with(|rc| *rc.borrow())) {
                                unimplemented!()
                            } else {
                                Wrap_3.with(|rc| *rc.borrow())
                            })
                            .wrapping_mul(res),
                        );
                        res = __rhs;
                        let __rhs = (unimplemented!().read()).wrapping_add(
                            (if (unimplemented!() < Wrap_3.with(|rc| *rc.borrow())) {
                                unimplemented!()
                            } else {
                                Wrap_3.with(|rc| *rc.borrow())
                            })
                            .wrapping_mul(res),
                        );
                        res = __rhs
                    };
                    return res;
                }
            )
            .spec::<fn(()) -> ()>(0)
            .call(unimplemented!())
        });
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
}
