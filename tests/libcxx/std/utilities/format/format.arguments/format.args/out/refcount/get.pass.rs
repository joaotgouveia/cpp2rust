extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(80)]
pub struct MultiStringType {
    #[offset(0)]
    #[byte_size(8)]
    pub w_: Ptr<()>,
    #[offset(8)]
    pub wn_: (),
    #[offset(16)]
    #[byte_size(8)]
    pub u8_: Ptr<()>,
    #[offset(24)]
    pub u8n_: (),
    #[offset(32)]
    #[byte_size(8)]
    pub u16_: Ptr<()>,
    #[offset(40)]
    pub u16n_: (),
    #[offset(48)]
    #[byte_size(8)]
    pub u32_: Ptr<()>,
    #[offset(56)]
    pub u32n_: (),
    #[offset(64)]
    #[byte_size(8)]
    pub s_: Ptr<()>,
    #[offset(72)]
    pub sn_: (),
}
impl MultiStringType {
    pub fn new(
        mut w: Ptr<()>,
        mut wn: (),
        mut u8: Ptr<()>,
        mut u8n: (),
        mut u16: Ptr<()>,
        mut u16n: (),
        mut u32: Ptr<()>,
        mut u32n: (),
        mut s: Ptr<()>,
        mut sn: (),
    ) -> Self {
        Self {
            w_: (w).clone(),
            wn_: wn,
            u8_: (u8).clone(),
            u8n_: u8n,
            u16_: (u16).clone(),
            u16n_: u16n,
            u32_: (u32).clone(),
            u32n_: u32n,
            s_: (s).clone(),
            sn_: sn,
        }
    }
}
pub fn test_0(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___bool_> =
        Rc::new(RefCell::new(({ make_format_args_1(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_1({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_2(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___char_> =
        Rc::new(RefCell::new(({ make_format_args_3(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_4(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___char_> =
        Rc::new(RefCell::new(({ make_format_args_3(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_5(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___char_> =
        Rc::new(RefCell::new(({ make_format_args_3(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_6(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___char_> =
        Rc::new(RefCell::new(({ make_format_args_3(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_2({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_7(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___signed_char_,
    > = Rc::new(RefCell::new(({ make_format_args_8(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_3({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_9(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___short_> =
        Rc::new(RefCell::new(({ make_format_args_10(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_4({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_11(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___int_> =
        Rc::new(RefCell::new(({ make_format_args_12(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_5({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_13(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___long_> =
        Rc::new(RefCell::new(({ make_format_args_14(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_6({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_15(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___long_long_> =
        Rc::new(RefCell::new(({ make_format_args_16(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_7({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_17(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_char_,
    > = Rc::new(RefCell::new(({ make_format_args_18(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_8({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_19(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_short_,
    > = Rc::new(RefCell::new(({ make_format_args_20(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_9({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_21(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_int_,
    > = Rc::new(RefCell::new(({ make_format_args_22(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_10({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_23(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_long_,
    > = Rc::new(RefCell::new(({ make_format_args_24(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_11({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_25(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned_long_long_,
    > = Rc::new(RefCell::new(({ make_format_args_26(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_12({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_27(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___float_> =
        Rc::new(RefCell::new(({ make_format_args_28(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_13({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_29(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___double_> =
        Rc::new(RefCell::new(({ make_format_args_30(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_14({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_31(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___long_double_,
    > = Rc::new(RefCell::new(({ make_format_args_32(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_15({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_33(value: Ptr<()>) {
    let value: Value<Ptr<()>> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___const_char_ptr_,
    > = Rc::new(RefCell::new(({ make_format_args_34(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_16({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<Ptr<()>> = Rc::new(RefCell::new((*value.borrow()).clone()));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_35(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___std_string_view_,
    > = Rc::new(RefCell::new(({ make_format_args_36(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_17({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_37(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char___std_string_> =
        Rc::new(RefCell::new(({ make_format_args_38(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_18({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(({ (*value.borrow())() })));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_39(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___std_nullptr_t_,
    > = Rc::new(RefCell::new(({ make_format_args_40(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_19({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        },
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_41(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___bool_> =
        Rc::new(RefCell::new(({ make_format_args_42(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_1({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_43(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___wchar_t_,
    > = Rc::new(RefCell::new(({ make_format_args_44(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_2({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_45(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___char_> =
        Rc::new(RefCell::new(({ make_format_args_46(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_47(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___char_> =
        Rc::new(RefCell::new(({ make_format_args_46(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_48(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___char_> =
        Rc::new(RefCell::new(({ make_format_args_46(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_3({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_49(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___signed_char_,
    > = Rc::new(RefCell::new(({ make_format_args_50(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_4({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_51(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___short_,
    > = Rc::new(RefCell::new(({ make_format_args_52(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_5({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_53(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___int_> =
        Rc::new(RefCell::new(({ make_format_args_54(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_6({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_55(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___long_> =
        Rc::new(RefCell::new(({ make_format_args_56(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_7({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_57(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___long_long_,
    > = Rc::new(RefCell::new(({ make_format_args_58(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_8({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_59(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_char_,
    > = Rc::new(RefCell::new(({ make_format_args_60(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_9({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_61(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_short_,
    > = Rc::new(RefCell::new(({ make_format_args_62(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_10({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_63(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_int_,
    > = Rc::new(RefCell::new(({ make_format_args_64(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_11({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_65(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_long_,
    > = Rc::new(RefCell::new(({ make_format_args_66(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_12({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_67(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned_long_long_,
    > = Rc::new(RefCell::new(({ make_format_args_68(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_13({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_69(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___float_,
    > = Rc::new(RefCell::new(({ make_format_args_70(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_14({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_71(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___double_,
    > = Rc::new(RefCell::new(({ make_format_args_72(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_15({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_73(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___long_double_,
    > = Rc::new(RefCell::new(({ make_format_args_74(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_16({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new((*value.borrow())));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_75(value: Ptr<()>) {
    let value: Value<Ptr<()>> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___const_wchar_t_ptr_,
    > = Rc::new(RefCell::new(({ make_format_args_76(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_17({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<Ptr<()>> = Rc::new(RefCell::new((*value.borrow()).clone()));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_77(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_wstring_view_,
    > = Rc::new(RefCell::new(({ make_format_args_78(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_18({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_79(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_wstring_,
    > = Rc::new(RefCell::new(({ make_format_args_80(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_19({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(({ (*value.borrow())() })));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_81(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___std_nullptr_t_,
    > = Rc::new(RefCell::new(({ make_format_args_82(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_20({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {
            let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        },
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        },
        |mut a: Ptr<()>| {
            {
                unimplemented!();
            }
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            {
                unimplemented!();
            }
        },
        |mut a: ()| {
            {
                unimplemented!();
            }
        }
    )));
    unimplemented!();
}
pub fn test_handle_83(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<std___format_arg_store_std_basic_format_context_char_ptr__char_____int128_> =
        Rc::new(RefCell::new(({ make_format_args_84(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_20({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: Ptr<()>| {
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        }
    )));
    unimplemented!();
}
pub fn test_handle_85(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_char_ptr__char___unsigned___int128_,
    > = Rc::new(RefCell::new(({ make_format_args_86(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_char_ptr__char__::new_21({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |a: std_basic_format_arg_std_basic_format_context_char_ptr__char___handle| {
            let a: Value<std_basic_format_arg_std_basic_format_context_char_ptr__char___handle> =
                Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: Ptr<()>| {
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        }
    )));
    unimplemented!();
}
pub fn test_handle_87(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t_____int128_,
    > = Rc::new(RefCell::new(({ make_format_args_88(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_21({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: Ptr<()>| {
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        }
    )));
    unimplemented!();
}
pub fn test_handle_89(value: ()) {
    let value: Value<()> = Rc::new(RefCell::new(value));
    let store: Value<
        std___format_arg_store_std_basic_format_context_wchar_t_ptr__wchar_t___unsigned___int128_,
    > = Rc::new(RefCell::new(({ make_format_args_90(value.as_pointer()) })));
    let format_args: Value<()> = Rc::new(RefCell::new(
        std_basic_format_args_std_basic_format_context_wchar_t_ptr__wchar_t__::new_22({
            store.as_pointer()
        }),
    ));
    let visitor: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |a: std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle| {
            let a: Value<
                std_basic_format_arg_std_basic_format_context_wchar_t_ptr__wchar_t___handle,
            > = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        },
        |mut a: Ptr<()>| {
            unimplemented!();
        },
        |a: ()| {
            let a: Value<()> = Rc::new(RefCell::new(a));
            unimplemented!();
        },
        |mut a: ()| {
            unimplemented!();
        }
    )));
    unimplemented!();
}
pub fn test_91() {
    let empty: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_0(true) });
    ({ test_0(false) });
    ({ test_2(('a' as ())) });
    ({ test_2(('z' as ())) });
    ({ test_2(('0' as ())) });
    ({ test_2(('9' as ())) });
    {
        ({ test_2(('a' as ())) });
        ({ test_2(('z' as ())) });
        ({ test_2(('0' as ())) });
        ({ test_2(('9' as ())) });
    }
    ({ test_7(unimplemented!()) });
    ({ test_7(0_()) });
    ({ test_7(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_9(0_()) });
    ({ test_9(unimplemented!()) });
    ({ test_9(unimplemented!()) });
    ({ test_11(unimplemented!()) });
    ({ test_11(unimplemented!()) });
    ({ test_11(unimplemented!()) });
    ({ test_11(0_()) });
    ({ test_11(unimplemented!()) });
    ({ test_11(unimplemented!()) });
    ({ test_11(unimplemented!()) });
    ({ test_13(unimplemented!()) });
    ({ test_13(unimplemented!()) });
    ({ test_13(unimplemented!()) });
    ({ test_13(unimplemented!()) });
    ({ test_13(0_()) });
    ({ test_13(unimplemented!()) });
    ({ test_13(unimplemented!()) });
    ({ test_13(unimplemented!()) });
    ({ test_13(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(0_()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_15(unimplemented!()) });
    ({ test_handle_83(0_()) });
    ({ test_17(0_()) });
    ({ test_17(unimplemented!()) });
    ({ test_19(0_()) });
    ({ test_19(unimplemented!()) });
    ({ test_19(unimplemented!()) });
    ({ test_21(0_()) });
    ({ test_21(unimplemented!()) });
    ({ test_21(unimplemented!()) });
    ({ test_21(unimplemented!()) });
    ({ test_23(0_()) });
    ({ test_23(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_23(unimplemented!()) });
    ({ test_25(0_()) });
    ({ test_25(unimplemented!()) });
    ({ test_25(unimplemented!()) });
    ({ test_25(unimplemented!()) });
    ({ test_25(unimplemented!()) });
    ({ test_25(unimplemented!()) });
    ({ test_handle_85(0_()) });
    ({ test_27(-unimplemented!()) });
    ({ test_27(-unimplemented!()) });
    ({ test_27(-0_f64) });
    ({ test_27(0_f64) });
    ({ test_27(unimplemented!()) });
    ({ test_27(unimplemented!()) });
    ({ test_29(-unimplemented!()) });
    ({ test_29(-unimplemented!()) });
    ({ test_29(-0_f64) });
    ({ test_29(0_f64) });
    ({ test_29(unimplemented!()) });
    ({ test_29(unimplemented!()) });
    ({ test_31(-unimplemented!()) });
    ({ test_31(-unimplemented!()) });
    ({ test_31(-0_f64) });
    ({ test_31(0_f64) });
    ({ test_31(unimplemented!()) });
    ({ test_31(unimplemented!()) });
    ({ test_33(unimplemented!()) });
    ({ test_33(unimplemented!()) });
    ({ test_35(unimplemented!()) });
    ({ test_35(({ (*empty.borrow())() })) });
    ({ test_35(({ (*str.borrow())() })) });
    ({ test_37(unimplemented!()) });
    ({ test_37(unimplemented!()) });
    ({ test_37(unimplemented!()) });
    ({ test_39(Default::default()) });
}
pub fn test_92() {
    let empty: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let str: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ test_41(true) });
    ({ test_41(false) });
    ({ test_43(('a' as ())) });
    ({ test_43(('z' as ())) });
    ({ test_43(('0' as ())) });
    ({ test_43(('9' as ())) });
    {
        {
            ({ test_45(('a' as ())) });
            ({ test_45(('z' as ())) });
            ({ test_45(('0' as ())) });
            ({ test_45(('9' as ())) });
        }
    }
    ({ test_49(unimplemented!()) });
    ({ test_49(0_()) });
    ({ test_49(unimplemented!()) });
    ({ test_51(unimplemented!()) });
    ({ test_51(unimplemented!()) });
    ({ test_51(0_()) });
    ({ test_51(unimplemented!()) });
    ({ test_51(unimplemented!()) });
    ({ test_53(unimplemented!()) });
    ({ test_53(unimplemented!()) });
    ({ test_53(unimplemented!()) });
    ({ test_53(0_()) });
    ({ test_53(unimplemented!()) });
    ({ test_53(unimplemented!()) });
    ({ test_53(unimplemented!()) });
    ({ test_55(unimplemented!()) });
    ({ test_55(unimplemented!()) });
    ({ test_55(unimplemented!()) });
    ({ test_55(unimplemented!()) });
    ({ test_55(0_()) });
    ({ test_55(unimplemented!()) });
    ({ test_55(unimplemented!()) });
    ({ test_55(unimplemented!()) });
    ({ test_55(unimplemented!()) });
    ({ test_57(unimplemented!()) });
    ({ test_57(unimplemented!()) });
    ({ test_57(unimplemented!()) });
    ({ test_57(unimplemented!()) });
    ({ test_57(unimplemented!()) });
    ({ test_57(0_()) });
    ({ test_57(unimplemented!()) });
    ({ test_57(unimplemented!()) });
    ({ test_57(unimplemented!()) });
    ({ test_57(unimplemented!()) });
    ({ test_57(unimplemented!()) });
    ({ test_handle_87(0_()) });
    ({ test_59(0_()) });
    ({ test_59(unimplemented!()) });
    ({ test_61(0_()) });
    ({ test_61(unimplemented!()) });
    ({ test_61(unimplemented!()) });
    ({ test_63(0_()) });
    ({ test_63(unimplemented!()) });
    ({ test_63(unimplemented!()) });
    ({ test_63(unimplemented!()) });
    ({ test_65(0_()) });
    ({ test_65(unimplemented!()) });
    ({ test_65(unimplemented!()) });
    ({ test_65(unimplemented!()) });
    ({ test_65(unimplemented!()) });
    ({ test_67(0_()) });
    ({ test_67(unimplemented!()) });
    ({ test_67(unimplemented!()) });
    ({ test_67(unimplemented!()) });
    ({ test_67(unimplemented!()) });
    ({ test_67(unimplemented!()) });
    ({ test_handle_89(0_()) });
    ({ test_69(-unimplemented!()) });
    ({ test_69(-unimplemented!()) });
    ({ test_69(-0_f64) });
    ({ test_69(0_f64) });
    ({ test_69(unimplemented!()) });
    ({ test_69(unimplemented!()) });
    ({ test_71(-unimplemented!()) });
    ({ test_71(-unimplemented!()) });
    ({ test_71(-0_f64) });
    ({ test_71(0_f64) });
    ({ test_71(unimplemented!()) });
    ({ test_71(unimplemented!()) });
    ({ test_73(-unimplemented!()) });
    ({ test_73(-unimplemented!()) });
    ({ test_73(-0_f64) });
    ({ test_73(0_f64) });
    ({ test_73(unimplemented!()) });
    ({ test_73(unimplemented!()) });
    ({ test_75(unimplemented!()) });
    ({ test_75(unimplemented!()) });
    ({ test_77(unimplemented!()) });
    ({ test_77(({ (*empty.borrow())() })) });
    ({ test_77(({ (*str.borrow())() })) });
    ({ test_79(unimplemented!()) });
    ({ test_79(unimplemented!()) });
    ({ test_79(unimplemented!()) });
    ({ test_81(Default::default()) });
}
pub fn test_93() {
    ({ test_91() });
    ({ test_92() });
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
    ({ test_93() });
    return 0_();
}
pub trait MultiStringTypeImpl {
    fn as_ptr_2(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_3(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_4(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_5(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn as_ptr_6(&self, _a0: Ptr<()>) -> Ptr<()>;
    fn length_7(&self, _a0: Ptr<()>) -> ();
    fn length_8(&self, _a0: Ptr<()>) -> ();
    fn length_9(&self, _a0: Ptr<()>) -> ();
    fn length_10(&self, _a0: Ptr<()>) -> ();
    fn length_11(&self, _a0: Ptr<()>) -> ();
    fn to_pconst_12(&self) -> Ptr<()>;
    fn to_pconst_13(&self) -> Ptr<()>;
    fn to_pconst_14(&self) -> Ptr<()>;
    fn to_pconst_15(&self) -> Ptr<()>;
    fn to_pconst_16(&self) -> Ptr<()>;
}
impl MultiStringTypeImpl for Ptr<MultiStringType> {
    fn as_ptr_2(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn as_ptr_3(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn as_ptr_4(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn as_ptr_5(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn as_ptr_6(&self, mut _a0: Ptr<()>) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
    fn length_7(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.sn_);
    }
    fn length_8(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.wn_);
    }
    fn length_9(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u8n_);
    }
    fn length_10(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u16n_);
    }
    fn length_11(&self, mut _a0: Ptr<()>) -> () {
        return (*self).with(|__s| __s.u32n_);
    }
    fn to_pconst_12(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.s_.clone());
    }
    fn to_pconst_13(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.w_.clone());
    }
    fn to_pconst_14(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u8_.clone());
    }
    fn to_pconst_15(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u16_.clone());
    }
    fn to_pconst_16(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.u32_.clone());
    }
}
pub fn __cpp2rust_init_globals() {}
