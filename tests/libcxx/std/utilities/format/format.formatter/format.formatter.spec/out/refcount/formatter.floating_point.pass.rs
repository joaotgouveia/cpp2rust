extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_format_context_create_0(out_it: (), args: ()) -> () {
    let out_it: Value<()> = Rc::new(RefCell::new(out_it));
    let args: Value<()> = Rc::new(RefCell::new(args));
    return ({
        __format_context_create_1(
            (*out_it.borrow_mut()).clone(),
            (*args.borrow()).clone(),
            None,
        )
    });
}
pub fn test_format_context_create_2(out_it: (), args: ()) -> () {
    let out_it: Value<()> = Rc::new(RefCell::new(out_it));
    let args: Value<()> = Rc::new(RefCell::new(args));
    return ({
        __format_context_create_3(
            (*out_it.borrow_mut()).clone(),
            (*args.borrow()).clone(),
            None,
        )
    });
}
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
pub fn test_4(fmt: (), mut arg: (), mut expected: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({ test_format_context_create_0((*out.borrow()).clone(), unimplemented!()) }),
    ));
    unimplemented!();
    if unimplemented!() {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_5(fmt: (), mut arg: (), mut expected: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({ test_format_context_create_0((*out.borrow()).clone(), unimplemented!()) }),
    ));
    unimplemented!();
    if unimplemented!() {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_6(fmt: (), mut arg: (), mut expected: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({ test_format_context_create_0((*out.borrow()).clone(), unimplemented!()) }),
    ));
    unimplemented!();
    if unimplemented!() {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_7(fmt: (), mut arg: (), mut expected: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({ test_format_context_create_2((*out.borrow()).clone(), unimplemented!()) }),
    ));
    unimplemented!();
    if unimplemented!() {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_8(fmt: (), mut arg: (), mut expected: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({ test_format_context_create_2((*out.borrow()).clone(), unimplemented!()) }),
    ));
    unimplemented!();
    if unimplemented!() {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_9(fmt: (), mut arg: (), mut expected: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({ test_format_context_create_2((*out.borrow()).clone(), unimplemented!()) }),
    ));
    unimplemented!();
    if unimplemented!() {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
    }
    unimplemented!();
}
pub fn test_termination_condition_10(mut f: (), mut arg: (), expected: Option<()>) {
    let mut expected: () = expected.unwrap_or_else(|| unimplemented!());
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_4(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    ({ test_4(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub fn test_termination_condition_11(mut f: (), mut arg: (), expected: Option<()>) {
    let mut expected: () = expected.unwrap_or_else(|| unimplemented!());
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_5(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    ({ test_5(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub fn test_termination_condition_12(mut f: (), mut arg: (), expected: Option<()>) {
    let mut expected: () = expected.unwrap_or_else(|| unimplemented!());
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_6(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    ({ test_6(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub fn test_termination_condition_13(mut f: (), mut arg: (), expected: Option<()>) {
    let mut expected: () = expected.unwrap_or_else(|| unimplemented!());
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_7(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    ({ test_7(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub fn test_termination_condition_14(mut f: (), mut arg: (), expected: Option<()>) {
    let mut expected: () = expected.unwrap_or_else(|| unimplemented!());
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_8(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    ({ test_8(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub fn test_termination_condition_15(mut f: (), mut arg: (), expected: Option<()>) {
    let mut expected: () = expected.unwrap_or_else(|| unimplemented!());
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_9(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    ({ test_9(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub fn test_hex_lower_case_precision_16(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_hex_lower_case_precision_18(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_hex_lower_case_precision_20(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_hex_lower_case_precision_21(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_hex_upper_case_precision_22(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_hex_upper_case_precision_23(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_hex_upper_case_precision_24(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_hex_upper_case_precision_25(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_scientific_lower_case_precision_26(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_scientific_lower_case_precision_27(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_scientific_lower_case_precision_28(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_scientific_lower_case_precision_29(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_scientific_upper_case_precision_30(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_scientific_upper_case_precision_31(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_scientific_upper_case_precision_32(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_scientific_upper_case_precision_33(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_fixed_lower_case_precision_34(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_fixed_lower_case_precision_35(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_fixed_lower_case_precision_36(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_fixed_lower_case_precision_37(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_fixed_upper_case_precision_38(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_fixed_upper_case_precision_39(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_fixed_upper_case_precision_40(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_fixed_upper_case_precision_41(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_general_lower_case_precision_42(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_general_lower_case_precision_43(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_general_lower_case_precision_44(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_general_lower_case_precision_45(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_general_upper_case_precision_46(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_10(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_general_upper_case_precision_47(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_11(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_general_upper_case_precision_48(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_17(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_13(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_general_upper_case_precision_49(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if ({ signbit_19(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({ test_termination_condition_14(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub fn test_value_50(mut value: ()) {
    ({ test_hex_lower_case_precision_16(value) });
    ({ test_hex_upper_case_precision_22(value) });
    ({ test_scientific_lower_case_precision_26(value) });
    ({ test_scientific_upper_case_precision_30(value) });
    ({ test_fixed_lower_case_precision_34(value) });
    ({ test_fixed_upper_case_precision_38(value) });
    ({ test_general_lower_case_precision_42(value) });
    ({ test_general_upper_case_precision_46(value) });
}
pub fn test_value_51(mut value: ()) {
    ({ test_hex_lower_case_precision_18(value) });
    ({ test_hex_upper_case_precision_23(value) });
    ({ test_scientific_lower_case_precision_27(value) });
    ({ test_scientific_upper_case_precision_31(value) });
    ({ test_fixed_lower_case_precision_35(value) });
    ({ test_fixed_upper_case_precision_39(value) });
    ({ test_general_lower_case_precision_43(value) });
    ({ test_general_upper_case_precision_47(value) });
}
pub fn test_value_52(mut value: ()) {
    ({ test_hex_lower_case_precision_20(value) });
    ({ test_hex_upper_case_precision_24(value) });
    ({ test_scientific_lower_case_precision_28(value) });
    ({ test_scientific_upper_case_precision_32(value) });
    ({ test_fixed_lower_case_precision_36(value) });
    ({ test_fixed_upper_case_precision_40(value) });
    ({ test_general_lower_case_precision_44(value) });
    ({ test_general_upper_case_precision_48(value) });
}
pub fn test_value_53(mut value: ()) {
    ({ test_hex_lower_case_precision_21(value) });
    ({ test_hex_upper_case_precision_25(value) });
    ({ test_scientific_lower_case_precision_29(value) });
    ({ test_scientific_upper_case_precision_33(value) });
    ({ test_fixed_lower_case_precision_37(value) });
    ({ test_fixed_upper_case_precision_41(value) });
    ({ test_general_lower_case_precision_45(value) });
    ({ test_general_upper_case_precision_49(value) });
}
pub fn test_special_values_54() {
    ({ test_value_50(-unimplemented!()) });
    ({ test_value_50(-1_f64) });
    ({ test_value_50(-unimplemented!()) });
    ({ test_value_50(-unimplemented!()) });
    ({ test_value_50(-0_f64) });
    ({ test_value_50(0_f64) });
    ({ test_value_50(unimplemented!()) });
    ({ test_value_50(1_f64) });
    ({ test_value_50(unimplemented!()) });
    ({ test_value_50(unimplemented!()) });
}
pub fn test_special_values_55() {
    ({ test_value_51(-unimplemented!()) });
    ({ test_value_51(-1_f64) });
    ({ test_value_51(-unimplemented!()) });
    ({ test_value_51(-unimplemented!()) });
    ({ test_value_51(-0_f64) });
    ({ test_value_51(0_f64) });
    ({ test_value_51(unimplemented!()) });
    ({ test_value_51(1_f64) });
    ({ test_value_51(unimplemented!()) });
    ({ test_value_51(unimplemented!()) });
}
pub fn test_special_values_56() {
    ({ test_value_52(-unimplemented!()) });
    ({ test_value_52(-1_f64) });
    ({ test_value_52(-unimplemented!()) });
    ({ test_value_52(-unimplemented!()) });
    ({ test_value_52(-0_f64) });
    ({ test_value_52(0_f64) });
    ({ test_value_52(unimplemented!()) });
    ({ test_value_52(1_f64) });
    ({ test_value_52(unimplemented!()) });
    ({ test_value_52(unimplemented!()) });
}
pub fn test_special_values_57() {
    ({ test_value_53(-unimplemented!()) });
    ({ test_value_53(-1_f64) });
    ({ test_value_53(-unimplemented!()) });
    ({ test_value_53(-unimplemented!()) });
    ({ test_value_53(-0_f64) });
    ({ test_value_53(0_f64) });
    ({ test_value_53(unimplemented!()) });
    ({ test_value_53(1_f64) });
    ({ test_value_53(unimplemented!()) });
    ({ test_value_53(unimplemented!()) });
}
pub fn test_float_type_58() {
    ({ test_termination_condition_10(unimplemented!(), -unimplemented!(), None) });
    ({ test_termination_condition_10(unimplemented!(), -unimplemented!(), None) });
    ({ test_termination_condition_10(unimplemented!(), -0_f64, None) });
    ({ test_termination_condition_10(unimplemented!(), 0_f64, None) });
    ({ test_termination_condition_10(unimplemented!(), unimplemented!(), None) });
    ({ test_termination_condition_10(unimplemented!(), unimplemented!(), None) });
    {}
    {}
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_10(_f, -unimplemented!(), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_10(_f, unimplemented!(), Some(_expected))
    });
    let mut nan: () = unimplemented!();
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_11(_f, ({ copysign_59(nan, -1_f64) }), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_10(_f, nan, Some(_expected))
    });
    {
        ({ test_special_values_54() });
    }
}
pub fn test_float_type_60() {
    ({ test_termination_condition_11(unimplemented!(), -unimplemented!(), None) });
    ({ test_termination_condition_11(unimplemented!(), -unimplemented!(), None) });
    ({ test_termination_condition_11(unimplemented!(), -0_f64, None) });
    ({ test_termination_condition_11(unimplemented!(), 0_f64, None) });
    ({ test_termination_condition_11(unimplemented!(), unimplemented!(), None) });
    ({ test_termination_condition_11(unimplemented!(), unimplemented!(), None) });
    {
        ({ test_termination_condition_11(unimplemented!(), -unimplemented!(), None) });
        ({ test_termination_condition_11(unimplemented!(), -unimplemented!(), None) });
        ({ test_termination_condition_11(unimplemented!(), unimplemented!(), None) });
        ({ test_termination_condition_11(unimplemented!(), unimplemented!(), None) });
    }
    {}
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_11(_f, -unimplemented!(), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_11(_f, unimplemented!(), Some(_expected))
    });
    let mut nan: () = unimplemented!();
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_11(_f, ({ copysign_61(nan, -1_f64) }), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_11(_f, nan, Some(_expected))
    });
    {
        ({ test_special_values_55() });
    }
}
pub fn test_float_type_62() {
    ({ test_termination_condition_12(unimplemented!(), -unimplemented!(), None) });
    ({ test_termination_condition_12(unimplemented!(), -unimplemented!(), None) });
    ({ test_termination_condition_12(unimplemented!(), -0_f64, None) });
    ({ test_termination_condition_12(unimplemented!(), 0_f64, None) });
    ({ test_termination_condition_12(unimplemented!(), unimplemented!(), None) });
    ({ test_termination_condition_12(unimplemented!(), unimplemented!(), None) });
    {
        ({ test_termination_condition_12(unimplemented!(), -unimplemented!(), None) });
        ({ test_termination_condition_12(unimplemented!(), -unimplemented!(), None) });
        ({ test_termination_condition_12(unimplemented!(), unimplemented!(), None) });
        ({ test_termination_condition_12(unimplemented!(), unimplemented!(), None) });
    }
    {
        ({ test_termination_condition_12(unimplemented!(), -unimplemented!(), None) });
        ({ test_termination_condition_12(unimplemented!(), -unimplemented!(), None) });
        ({ test_termination_condition_12(unimplemented!(), unimplemented!(), None) });
        ({ test_termination_condition_12(unimplemented!(), unimplemented!(), None) });
    }
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_12(_f, -unimplemented!(), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_12(_f, unimplemented!(), Some(_expected))
    });
    let mut nan: () = unimplemented!();
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_12(_f, ({ copysign_63(nan, -1_f64) }), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_12(_f, nan, Some(_expected))
    });
    {}
}
pub fn test_float_type_64() {
    ({ test_termination_condition_13(unimplemented!(), -unimplemented!(), None) });
    ({ test_termination_condition_13(unimplemented!(), -unimplemented!(), None) });
    ({ test_termination_condition_13(unimplemented!(), -0_f64, None) });
    ({ test_termination_condition_13(unimplemented!(), 0_f64, None) });
    ({ test_termination_condition_13(unimplemented!(), unimplemented!(), None) });
    ({ test_termination_condition_13(unimplemented!(), unimplemented!(), None) });
    {}
    {}
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_13(_f, -unimplemented!(), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_13(_f, unimplemented!(), Some(_expected))
    });
    let mut nan: () = unimplemented!();
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_14(_f, ({ copysign_59(nan, -1_f64) }), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_13(_f, nan, Some(_expected))
    });
    {
        ({ test_special_values_56() });
    }
}
pub fn test_float_type_65() {
    ({ test_termination_condition_14(unimplemented!(), -unimplemented!(), None) });
    ({ test_termination_condition_14(unimplemented!(), -unimplemented!(), None) });
    ({ test_termination_condition_14(unimplemented!(), -0_f64, None) });
    ({ test_termination_condition_14(unimplemented!(), 0_f64, None) });
    ({ test_termination_condition_14(unimplemented!(), unimplemented!(), None) });
    ({ test_termination_condition_14(unimplemented!(), unimplemented!(), None) });
    {
        ({ test_termination_condition_14(unimplemented!(), -unimplemented!(), None) });
        ({ test_termination_condition_14(unimplemented!(), -unimplemented!(), None) });
        ({ test_termination_condition_14(unimplemented!(), unimplemented!(), None) });
        ({ test_termination_condition_14(unimplemented!(), unimplemented!(), None) });
    }
    {}
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_14(_f, -unimplemented!(), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_14(_f, unimplemented!(), Some(_expected))
    });
    let mut nan: () = unimplemented!();
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_14(_f, ({ copysign_61(nan, -1_f64) }), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_14(_f, nan, Some(_expected))
    });
    {
        ({ test_special_values_57() });
    }
}
pub fn test_float_type_66() {
    ({ test_termination_condition_15(unimplemented!(), -unimplemented!(), None) });
    ({ test_termination_condition_15(unimplemented!(), -unimplemented!(), None) });
    ({ test_termination_condition_15(unimplemented!(), -0_f64, None) });
    ({ test_termination_condition_15(unimplemented!(), 0_f64, None) });
    ({ test_termination_condition_15(unimplemented!(), unimplemented!(), None) });
    ({ test_termination_condition_15(unimplemented!(), unimplemented!(), None) });
    {
        ({ test_termination_condition_15(unimplemented!(), -unimplemented!(), None) });
        ({ test_termination_condition_15(unimplemented!(), -unimplemented!(), None) });
        ({ test_termination_condition_15(unimplemented!(), unimplemented!(), None) });
        ({ test_termination_condition_15(unimplemented!(), unimplemented!(), None) });
    }
    {
        ({ test_termination_condition_15(unimplemented!(), -unimplemented!(), None) });
        ({ test_termination_condition_15(unimplemented!(), -unimplemented!(), None) });
        ({ test_termination_condition_15(unimplemented!(), unimplemented!(), None) });
        ({ test_termination_condition_15(unimplemented!(), unimplemented!(), None) });
    }
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_15(_f, -unimplemented!(), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_15(_f, unimplemented!(), Some(_expected))
    });
    let mut nan: () = unimplemented!();
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_15(_f, ({ copysign_63(nan, -1_f64) }), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_15(_f, nan, Some(_expected))
    });
    {}
}
pub fn test_all_float_types_67() {
    ({ test_float_type_58() });
    ({ test_float_type_60() });
    ({ test_float_type_62() });
}
pub fn test_all_float_types_68() {
    ({ test_float_type_64() });
    ({ test_float_type_65() });
    ({ test_float_type_66() });
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
    ({ test_all_float_types_67() });
    ({ test_all_float_types_68() });
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
