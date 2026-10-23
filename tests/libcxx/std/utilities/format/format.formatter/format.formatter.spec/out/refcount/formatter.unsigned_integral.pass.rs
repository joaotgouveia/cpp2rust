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
pub fn test_4(mut expected: (), fmt: (), arg: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_0((*out.borrow()).clone(), {
                let __tmp_0 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___unsigned_char_ > = Rc::new(RefCell::new(( { make_format_args_5 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_1 ( { __tmp_0.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_integer_char_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    unimplemented!();
}
pub fn test_6(mut expected: (), fmt: (), arg: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_0((*out.borrow()).clone(), {
                let __tmp_1 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___unsigned_short_ > = Rc::new(RefCell::new(( { make_format_args_7 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_2 ( { __tmp_1.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_integer_char_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    unimplemented!();
}
pub fn test_8(mut expected: (), fmt: (), arg: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_0((*out.borrow()).clone(), {
                let __tmp_2 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___unsigned_int_ > = Rc::new(RefCell::new(( { make_format_args_9 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_3 ( { __tmp_2.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_integer_char_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    unimplemented!();
}
pub fn test_10(mut expected: (), fmt: (), arg: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_0((*out.borrow()).clone(), {
                let __tmp_3 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___unsigned_long_ > = Rc::new(RefCell::new(( { make_format_args_11 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_4 ( { __tmp_3.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_integer_char_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    unimplemented!();
}
pub fn test_12(mut expected: (), fmt: (), arg: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_0((*out.borrow()).clone(), {
                let __tmp_4 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___unsigned_long_long_ > = Rc::new(RefCell::new(( { make_format_args_13 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_5 ( { __tmp_4.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_integer_char_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    unimplemented!();
}
pub fn test_14(mut expected: (), fmt: (), arg: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_0((*out.borrow()).clone(), {
                let __tmp_5 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___unsigned___int128_ > = Rc::new(RefCell::new(( { make_format_args_15 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_6 ( { __tmp_5.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_integer_char_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    unimplemented!();
}
pub fn test_16(mut expected: (), fmt: (), arg: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_2((*out.borrow()).clone(), {
                let __tmp_6 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___unsigned_char_ > = Rc::new(RefCell::new(( { make_format_args_17 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_1 ( { __tmp_6.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_integer_wchar_t_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    unimplemented!();
}
pub fn test_18(mut expected: (), fmt: (), arg: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_2((*out.borrow()).clone(), {
                let __tmp_7 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___unsigned_short_ > = Rc::new(RefCell::new(( { make_format_args_19 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_2 ( { __tmp_7.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_integer_wchar_t_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    unimplemented!();
}
pub fn test_20(mut expected: (), fmt: (), arg: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_2((*out.borrow()).clone(), {
                let __tmp_8 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___unsigned_int_ > = Rc::new(RefCell::new(( { make_format_args_21 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_3 ( { __tmp_8.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_integer_wchar_t_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    unimplemented!();
}
pub fn test_22(mut expected: (), fmt: (), arg: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_2((*out.borrow()).clone(), {
                let __tmp_9 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___unsigned_long_ > = Rc::new(RefCell::new(( { make_format_args_23 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_4 ( { __tmp_9.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_integer_wchar_t_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    unimplemented!();
}
pub fn test_24(mut expected: (), fmt: (), arg: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_2((*out.borrow()).clone(), {
                let __tmp_10 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___unsigned_long_long_ > = Rc::new(RefCell::new(( { make_format_args_25 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_5 ( { __tmp_10.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_integer_wchar_t_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    unimplemented!();
}
pub fn test_26(mut expected: (), fmt: (), arg: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_2((*out.borrow()).clone(), {
                let __tmp_11 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___unsigned___int128_ > = Rc::new(RefCell::new(( { make_format_args_27 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_6 ( { __tmp_11.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_integer_wchar_t_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    unimplemented!();
}
pub fn test_termination_condition_28(mut expected: (), f: (), mut arg: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_4(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_4(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_29(mut expected: (), f: (), mut arg: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_6(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_6(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_30(mut expected: (), f: (), mut arg: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_8(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_8(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_31(mut expected: (), f: (), mut arg: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_10(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_10(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_32(mut expected: (), f: (), mut arg: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_12(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_12(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_33(mut expected: (), f: (), mut arg: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_14(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_14(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_34(mut expected: (), f: (), mut arg: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_16(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_16(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_35(mut expected: (), f: (), mut arg: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_18(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_18(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_36(mut expected: (), f: (), mut arg: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_20(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_20(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_37(mut expected: (), f: (), mut arg: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_22(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_22(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_38(mut expected: (), f: (), mut arg: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_24(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_24(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_termination_condition_39(mut expected: (), f: (), mut arg: ()) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_26(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    ({ test_26(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub fn test_unsigned_integral_type_40() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_28(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_28(_expected, _f, 255_())
    });
    {}
    {}
    {}
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_28(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_41() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_29(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_29(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_29(_expected, _f, 65535_())
        });
    }
    {}
    {}
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_29(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_42() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_30(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_30(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_30(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_30(_expected, _f, 4294967295_())
        });
    }
    {}
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_30(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_43() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_31(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_31(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_31(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_31(_expected, _f, 4294967295_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_31(_expected, _f, 8446744073709551615_())
        });
    }
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_31(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_44() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_32(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_32(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_32(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_32(_expected, _f, 4294967295_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_32(_expected, _f, 8446744073709551615_())
        });
    }
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_32(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_45() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_33(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_33(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_33(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_33(_expected, _f, 4294967295_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_33(_expected, _f, 8446744073709551615_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_33(_expected, _f, unimplemented!())
        });
    }
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_33(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_46() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_34(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_34(_expected, _f, 255_())
    });
    {}
    {}
    {}
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_34(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_47() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_35(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_35(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_35(_expected, _f, 65535_())
        });
    }
    {}
    {}
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_35(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_48() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_36(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_36(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_36(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_36(_expected, _f, 4294967295_())
        });
    }
    {}
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_36(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_49() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_37(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_37(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_37(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_37(_expected, _f, 4294967295_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_37(_expected, _f, 8446744073709551615_())
        });
    }
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_37(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_50() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_38(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_38(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_38(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_38(_expected, _f, 4294967295_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_38(_expected, _f, 8446744073709551615_())
        });
    }
    {}
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_38(_expected, _f, 255_())
    });
}
pub fn test_unsigned_integral_type_51() {
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_39(_expected, _f, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_39(_expected, _f, 255_())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_39(_expected, _f, 65535_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_39(_expected, _f, 4294967295_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_39(_expected, _f, 8446744073709551615_())
        });
    }
    {
        ({
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_39(_expected, _f, unimplemented!())
        });
    }
    ({
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_39(_expected, _f, 255_())
    });
}
pub fn test_all_unsigned_integral_types_52() {
    ({ test_unsigned_integral_type_40() });
    ({ test_unsigned_integral_type_41() });
    ({ test_unsigned_integral_type_42() });
    ({ test_unsigned_integral_type_43() });
    ({ test_unsigned_integral_type_44() });
    ({ test_unsigned_integral_type_45() });
}
pub fn test_all_unsigned_integral_types_53() {
    ({ test_unsigned_integral_type_46() });
    ({ test_unsigned_integral_type_47() });
    ({ test_unsigned_integral_type_48() });
    ({ test_unsigned_integral_type_49() });
    ({ test_unsigned_integral_type_50() });
    ({ test_unsigned_integral_type_51() });
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
    ({ test_all_unsigned_integral_types_52() });
    ({ test_all_unsigned_integral_types_53() });
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
