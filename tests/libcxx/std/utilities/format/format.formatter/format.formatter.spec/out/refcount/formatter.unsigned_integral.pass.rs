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
pub fn test_4(mut expected: (), fmt: (), mut arg: (), mut offset: ()) {
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
    unimplemented!();
}
pub fn test_5(mut expected: (), fmt: (), mut arg: (), mut offset: ()) {
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
    unimplemented!();
}
pub fn test_6(mut expected: (), fmt: (), mut arg: (), mut offset: ()) {
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
    unimplemented!();
}
pub fn test_7(mut expected: (), fmt: (), mut arg: (), mut offset: ()) {
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
    unimplemented!();
}
pub fn test_8(mut expected: (), fmt: (), mut arg: (), mut offset: ()) {
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
    unimplemented!();
}
pub fn test_9(mut expected: (), fmt: (), mut arg: (), mut offset: ()) {
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
    unimplemented!();
}
pub fn test_10(mut expected: (), fmt: (), mut arg: (), mut offset: ()) {
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
    unimplemented!();
}
pub fn test_11(mut expected: (), fmt: (), mut arg: (), mut offset: ()) {
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
    unimplemented!();
}
pub fn test_12(mut expected: (), fmt: (), mut arg: (), mut offset: ()) {
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
    unimplemented!();
}
pub fn test_13(mut expected: (), fmt: (), mut arg: (), mut offset: ()) {
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
    unimplemented!();
}
pub fn test_14(mut expected: (), fmt: (), mut arg: (), mut offset: ()) {
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
    unimplemented!();
}
pub fn test_15(mut expected: (), fmt: (), mut arg: (), mut offset: ()) {
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
    unimplemented!();
}
pub fn test_termination_condition_16(mut expected: (), mut f: (), mut arg: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_4(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_4(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_17(mut expected: (), mut f: (), mut arg: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_5(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_5(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_18(mut expected: (), mut f: (), mut arg: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_6(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_6(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_19(mut expected: (), mut f: (), mut arg: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_7(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_7(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_20(mut expected: (), mut f: (), mut arg: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_8(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_8(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_21(mut expected: (), mut f: (), mut arg: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_9(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_9(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_22(mut expected: (), mut f: (), mut arg: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_10(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_10(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_23(mut expected: (), mut f: (), mut arg: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_11(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_11(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_24(mut expected: (), mut f: (), mut arg: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_12(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_12(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_25(mut expected: (), mut f: (), mut arg: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_13(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_13(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_26(mut expected: (), mut f: (), mut arg: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_14(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_14(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_27(mut expected: (), mut f: (), mut arg: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    ({ test_15(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_15(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_unsigned_integral_type_28() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_16(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_16(_expected, _f, 255_())
    });
    {}
    {}
    {}
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_16(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_29() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_17(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_17(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_17(_expected, _f, 65535_())
        });
    }
    {}
    {}
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_17(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_30() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_18(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_18(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_18(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_18(_expected, _f, 4294967295_())
        });
    }
    {}
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_18(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_31() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_19(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_19(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_19(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_19(_expected, _f, 4294967295_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_19(_expected, _f, 8446744073709551615_())
        });
    }
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_19(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_32() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_20(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_20(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_20(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_20(_expected, _f, 4294967295_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_20(_expected, _f, 8446744073709551615_())
        });
    }
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_20(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_33() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_21(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_21(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_21(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_21(_expected, _f, 4294967295_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_21(_expected, _f, 8446744073709551615_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_21(_expected, _f, unimplemented!())
        });
    }
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_21(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_34() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_22(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_22(_expected, _f, 255_())
    });
    {}
    {}
    {}
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_22(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_35() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_23(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_23(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_23(_expected, _f, 65535_())
        });
    }
    {}
    {}
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_23(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_36() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_24(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_24(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_24(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_24(_expected, _f, 4294967295_())
        });
    }
    {}
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_24(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_37() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_25(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_25(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_25(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_25(_expected, _f, 4294967295_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_25(_expected, _f, 8446744073709551615_())
        });
    }
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_25(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_38() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_26(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_26(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_26(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_26(_expected, _f, 4294967295_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_26(_expected, _f, 8446744073709551615_())
        });
    }
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_26(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_39() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_27(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_27(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_27(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_27(_expected, _f, 4294967295_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_27(_expected, _f, 8446744073709551615_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_27(_expected, _f, unimplemented!())
        });
    }
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_27(_expected, _f, 255_())
    });
}
pub fn test_all_unsigned_integral_types_40() {
    ({ test_unsigned_integral_type_28() });
    ({ test_unsigned_integral_type_29() });
    ({ test_unsigned_integral_type_30() });
    ({ test_unsigned_integral_type_31() });
    ({ test_unsigned_integral_type_32() });
    ({ test_unsigned_integral_type_33() });
}
pub fn test_all_unsigned_integral_types_41() {
    ({ test_unsigned_integral_type_34() });
    ({ test_unsigned_integral_type_35() });
    ({ test_unsigned_integral_type_36() });
    ({ test_unsigned_integral_type_37() });
    ({ test_unsigned_integral_type_38() });
    ({ test_unsigned_integral_type_39() });
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
    ({ test_all_unsigned_integral_types_40() });
    ({ test_all_unsigned_integral_types_41() });
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
