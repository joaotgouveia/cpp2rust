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
pub fn test_4(fmt: (), arg: (), expected: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = ({
        ((*formatter.borrow()) as std___formatter_floating_point_char_)
            .parse(parse_ctx.as_pointer())
    });
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_0((*out.borrow()).clone(), {
                let __tmp_0 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___float_ > = Rc::new(RefCell::new(( { make_format_args_5 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_1 ( { __tmp_0.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_floating_point_char_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    if unimplemented!() {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let ___first: Ptr<()> = unimplemented!();
            let ___last: Ptr<()> = { unimplemented!().ptr.clone() };
            (*expected.borrow()).append_75_char_ptr_0(___first, ___last)
        });
    }
    unimplemented!();
}
pub fn test_6(fmt: (), arg: (), expected: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = ({
        ((*formatter.borrow()) as std___formatter_floating_point_char_)
            .parse(parse_ctx.as_pointer())
    });
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_0((*out.borrow()).clone(), {
                let __tmp_1 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___double_ > = Rc::new(RefCell::new(( { make_format_args_7 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_2 ( { __tmp_1.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_floating_point_char_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    if unimplemented!() {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let ___first: Ptr<()> = unimplemented!();
            let ___last: Ptr<()> = { unimplemented!().ptr.clone() };
            (*expected.borrow()).append_75_char_ptr_0(___first, ___last)
        });
    }
    unimplemented!();
}
pub fn test_8(fmt: (), arg: (), expected: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = ({
        ((*formatter.borrow()) as std___formatter_floating_point_char_)
            .parse(parse_ctx.as_pointer())
    });
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_0((*out.borrow()).clone(), {
                let __tmp_2 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___long_double_ > = Rc::new(RefCell::new(( { make_format_args_9 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_3 ( { __tmp_2.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_floating_point_char_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    if unimplemented!() {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let ___first: Ptr<()> = unimplemented!();
            let ___last: Ptr<()> = { unimplemented!().ptr.clone() };
            (*expected.borrow()).append_75_char_ptr_0(___first, ___last)
        });
    }
    unimplemented!();
}
pub fn test_10(fmt: (), arg: (), expected: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = ({
        ((*formatter.borrow()) as std___formatter_floating_point_wchar_t_)
            .parse(parse_ctx.as_pointer())
    });
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_2((*out.borrow()).clone(), {
                let __tmp_3 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___float_ > = Rc::new(RefCell::new(( { make_format_args_11 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_1 ( { __tmp_3.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_floating_point_wchar_t_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    if unimplemented!() {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let ___first: Ptr<()> = unimplemented!();
            let ___last: Ptr<()> = { unimplemented!().ptr.clone() };
            (*expected.borrow()).append_75_char_ptr_0(___first, ___last)
        });
    }
    unimplemented!();
}
pub fn test_12(fmt: (), arg: (), expected: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = ({
        ((*formatter.borrow()) as std___formatter_floating_point_wchar_t_)
            .parse(parse_ctx.as_pointer())
    });
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_2((*out.borrow()).clone(), {
                let __tmp_4 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___double_ > = Rc::new(RefCell::new(( { make_format_args_13 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_2 ( { __tmp_4.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_floating_point_wchar_t_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    if unimplemented!() {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let ___first: Ptr<()> = unimplemented!();
            let ___last: Ptr<()> = { unimplemented!().ptr.clone() };
            (*expected.borrow()).append_75_char_ptr_0(___first, ___last)
        });
    }
    unimplemented!();
}
pub fn test_14(fmt: (), arg: (), expected: (), mut offset: ()) {
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = ({
        ((*formatter.borrow()) as std___formatter_floating_point_wchar_t_)
            .parse(parse_ctx.as_pointer())
    });
    unimplemented!();
    let mut result: () = unimplemented!();
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_2((*out.borrow()).clone(), {
                let __tmp_5 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___long_double_ > = Rc::new(RefCell::new(( { make_format_args_15 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_3 ( { __tmp_5.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_floating_point_wchar_t_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    if unimplemented!() {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let ___first: Ptr<()> = unimplemented!();
            let ___last: Ptr<()> = { unimplemented!().ptr.clone() };
            (*expected.borrow()).append_75_char_ptr_0(___first, ___last)
        });
    }
    unimplemented!();
}
pub fn test_termination_condition_16(f: (), mut arg: (), expected: Option<()>) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let mut expected: () = expected.unwrap_or_else(|| unimplemented!());
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_4(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    ({ test_4(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub fn test_termination_condition_17(f: (), mut arg: (), expected: Option<()>) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let mut expected: () = expected.unwrap_or_else(|| unimplemented!());
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_6(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    ({ test_6(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub fn test_termination_condition_18(f: (), mut arg: (), expected: Option<()>) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let mut expected: () = expected.unwrap_or_else(|| unimplemented!());
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_8(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    ({ test_8(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub fn test_termination_condition_19(f: (), mut arg: (), expected: Option<()>) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let mut expected: () = expected.unwrap_or_else(|| unimplemented!());
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_10(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    ({ test_10(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub fn test_termination_condition_20(f: (), mut arg: (), expected: Option<()>) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let mut expected: () = expected.unwrap_or_else(|| unimplemented!());
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_12(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    ({ test_12(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub fn test_termination_condition_21(f: (), mut arg: (), expected: Option<()>) {
    let f: Value<()> = Rc::new(RefCell::new(f));
    let mut expected: () = expected.unwrap_or_else(|| unimplemented!());
    let fmt: Value<()> = Rc::new(RefCell::new(({ (*f.borrow())() })));
    unimplemented!();
    ({ test_14(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    ({ test_14(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub fn test_hex_lower_case_precision_22(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_hex_lower_case_precision_24(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_hex_lower_case_precision_26(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_hex_lower_case_precision_27(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_hex_upper_case_precision_28(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_hex_upper_case_precision_29(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_hex_upper_case_precision_30(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_hex_upper_case_precision_31(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_scientific_lower_case_precision_32(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_scientific_lower_case_precision_33(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_scientific_lower_case_precision_34(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_scientific_lower_case_precision_35(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_scientific_upper_case_precision_36(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_scientific_upper_case_precision_37(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_scientific_upper_case_precision_38(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_scientific_upper_case_precision_39(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_fixed_lower_case_precision_40(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_fixed_lower_case_precision_41(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_fixed_lower_case_precision_42(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_fixed_lower_case_precision_43(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_fixed_upper_case_precision_44(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_fixed_upper_case_precision_45(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_fixed_upper_case_precision_46(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_fixed_upper_case_precision_47(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_general_lower_case_precision_48(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_general_lower_case_precision_49(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_general_lower_case_precision_50(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_general_lower_case_precision_51(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_general_upper_case_precision_52(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_16(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_general_upper_case_precision_53(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { (end).clone() },
                        None,
                    ),
                ),
            )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_17(
                unimplemented!(),
                value,
                Some(
                    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                        { unimplemented!() },
                        { unimplemented!() },
                        None,
                    ),
                ),
            )
        });
    }
}
pub fn test_general_upper_case_precision_54(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_23(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_19 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_general_upper_case_precision_55(mut value: ()) {
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
    {
        let buffer: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut end_ptr: Ptr<()> = { unimplemented!().ptr.clone() };
        let mut size: () = ((end_ptr).clone() - unimplemented!()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { (end ).clone() } , None , )   ) , )
        });
        let mut unused: () = (unimplemented!() - (end).clone()) as ();
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
        unimplemented!();
        if ({ signbit_25(value) }) {
            unimplemented!().write(('-' as ()));
            unimplemented!().write(('0' as ()));
        }
        ({
            test_termination_condition_20 ( unimplemented!()  , value  , Some ( std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()   } , { unimplemented!()   } , None , )   ) , )
        });
    }
}
pub fn test_value_56(mut value: ()) {
    ({ test_hex_lower_case_precision_22(value) });
    ({ test_hex_upper_case_precision_28(value) });
    ({ test_scientific_lower_case_precision_32(value) });
    ({ test_scientific_upper_case_precision_36(value) });
    ({ test_fixed_lower_case_precision_40(value) });
    ({ test_fixed_upper_case_precision_44(value) });
    ({ test_general_lower_case_precision_48(value) });
    ({ test_general_upper_case_precision_52(value) });
}
pub fn test_value_57(mut value: ()) {
    ({ test_hex_lower_case_precision_24(value) });
    ({ test_hex_upper_case_precision_29(value) });
    ({ test_scientific_lower_case_precision_33(value) });
    ({ test_scientific_upper_case_precision_37(value) });
    ({ test_fixed_lower_case_precision_41(value) });
    ({ test_fixed_upper_case_precision_45(value) });
    ({ test_general_lower_case_precision_49(value) });
    ({ test_general_upper_case_precision_53(value) });
}
pub fn test_value_58(mut value: ()) {
    ({ test_hex_lower_case_precision_26(value) });
    ({ test_hex_upper_case_precision_30(value) });
    ({ test_scientific_lower_case_precision_34(value) });
    ({ test_scientific_upper_case_precision_38(value) });
    ({ test_fixed_lower_case_precision_42(value) });
    ({ test_fixed_upper_case_precision_46(value) });
    ({ test_general_lower_case_precision_50(value) });
    ({ test_general_upper_case_precision_54(value) });
}
pub fn test_value_59(mut value: ()) {
    ({ test_hex_lower_case_precision_27(value) });
    ({ test_hex_upper_case_precision_31(value) });
    ({ test_scientific_lower_case_precision_35(value) });
    ({ test_scientific_upper_case_precision_39(value) });
    ({ test_fixed_lower_case_precision_43(value) });
    ({ test_fixed_upper_case_precision_47(value) });
    ({ test_general_lower_case_precision_51(value) });
    ({ test_general_upper_case_precision_55(value) });
}
pub fn test_special_values_60() {
    ({ test_value_56(-({ std_numeric_limits_float_::max() })) });
    ({ test_value_56(-1_f64) });
    ({ test_value_56(-({ std_numeric_limits_float_::min() })) });
    ({ test_value_56(-({ std_numeric_limits_float_::denorm_min() })) });
    ({ test_value_56(-0_f64) });
    ({ test_value_56(0_f64) });
    ({ test_value_56(({ std_numeric_limits_float_::denorm_min() })) });
    ({ test_value_56(1_f64) });
    ({ test_value_56(({ std_numeric_limits_float_::min() })) });
    ({ test_value_56(({ std_numeric_limits_float_::max() })) });
}
pub fn test_special_values_61() {
    ({ test_value_57(-({ std_numeric_limits_double_::max() })) });
    ({ test_value_57(-1_f64) });
    ({ test_value_57(-({ std_numeric_limits_double_::min() })) });
    ({ test_value_57(-({ std_numeric_limits_double_::denorm_min() })) });
    ({ test_value_57(-0_f64) });
    ({ test_value_57(0_f64) });
    ({ test_value_57(({ std_numeric_limits_double_::denorm_min() })) });
    ({ test_value_57(1_f64) });
    ({ test_value_57(({ std_numeric_limits_double_::min() })) });
    ({ test_value_57(({ std_numeric_limits_double_::max() })) });
}
pub fn test_special_values_62() {
    ({ test_value_58(-({ std_numeric_limits_float_::max() })) });
    ({ test_value_58(-1_f64) });
    ({ test_value_58(-({ std_numeric_limits_float_::min() })) });
    ({ test_value_58(-({ std_numeric_limits_float_::denorm_min() })) });
    ({ test_value_58(-0_f64) });
    ({ test_value_58(0_f64) });
    ({ test_value_58(({ std_numeric_limits_float_::denorm_min() })) });
    ({ test_value_58(1_f64) });
    ({ test_value_58(({ std_numeric_limits_float_::min() })) });
    ({ test_value_58(({ std_numeric_limits_float_::max() })) });
}
pub fn test_special_values_63() {
    ({ test_value_59(-({ std_numeric_limits_double_::max() })) });
    ({ test_value_59(-1_f64) });
    ({ test_value_59(-({ std_numeric_limits_double_::min() })) });
    ({ test_value_59(-({ std_numeric_limits_double_::denorm_min() })) });
    ({ test_value_59(-0_f64) });
    ({ test_value_59(0_f64) });
    ({ test_value_59(({ std_numeric_limits_double_::denorm_min() })) });
    ({ test_value_59(1_f64) });
    ({ test_value_59(({ std_numeric_limits_double_::min() })) });
    ({ test_value_59(({ std_numeric_limits_double_::max() })) });
}
pub fn test_float_type_64() {
    ({
        test_termination_condition_16(
            unimplemented!(),
            -({ std_numeric_limits_float_::max() }),
            None,
        )
    });
    ({
        test_termination_condition_16(
            unimplemented!(),
            -({ std_numeric_limits_float_::min() }),
            None,
        )
    });
    ({ test_termination_condition_16(unimplemented!(), -0_f64, None) });
    ({ test_termination_condition_16(unimplemented!(), 0_f64, None) });
    ({
        test_termination_condition_16(
            unimplemented!(),
            ({ std_numeric_limits_float_::min() }),
            None,
        )
    });
    ({
        test_termination_condition_16(
            unimplemented!(),
            ({ std_numeric_limits_float_::max() }),
            None,
        )
    });
    {}
    {}
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_16(
            _f,
            -({ std_numeric_limits_float_::infinity() }),
            Some(_expected),
        )
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_16(
            _f,
            ({ std_numeric_limits_float_::infinity() }),
            Some(_expected),
        )
    });
    let mut nan: () = ({ std_numeric_limits_float_::quiet_NaN() });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_17(_f, ({ copysign_65(nan, -1_f64) }), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_16(_f, nan, Some(_expected))
    });
    {
        ({ test_special_values_60() });
    }
}
pub fn test_float_type_66() {
    ({
        test_termination_condition_17(
            unimplemented!(),
            -({ std_numeric_limits_float_::max() }),
            None,
        )
    });
    ({
        test_termination_condition_17(
            unimplemented!(),
            -({ std_numeric_limits_float_::min() }),
            None,
        )
    });
    ({ test_termination_condition_17(unimplemented!(), -0_f64, None) });
    ({ test_termination_condition_17(unimplemented!(), 0_f64, None) });
    ({
        test_termination_condition_17(
            unimplemented!(),
            ({ std_numeric_limits_float_::min() }),
            None,
        )
    });
    ({
        test_termination_condition_17(
            unimplemented!(),
            ({ std_numeric_limits_float_::max() }),
            None,
        )
    });
    {
        ({
            test_termination_condition_17(
                unimplemented!(),
                -({ std_numeric_limits_double_::max() }),
                None,
            )
        });
        ({
            test_termination_condition_17(
                unimplemented!(),
                -({ std_numeric_limits_double_::min() }),
                None,
            )
        });
        ({
            test_termination_condition_17(
                unimplemented!(),
                ({ std_numeric_limits_double_::min() }),
                None,
            )
        });
        ({
            test_termination_condition_17(
                unimplemented!(),
                ({ std_numeric_limits_double_::max() }),
                None,
            )
        });
    }
    {}
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_17(
            _f,
            -({ std_numeric_limits_double_::infinity() }),
            Some(_expected),
        )
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_17(
            _f,
            ({ std_numeric_limits_double_::infinity() }),
            Some(_expected),
        )
    });
    let mut nan: () = ({ std_numeric_limits_double_::quiet_NaN() });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_17(_f, ({ copysign_67(nan, -1_f64) }), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_17(_f, nan, Some(_expected))
    });
    {
        ({ test_special_values_61() });
    }
}
pub fn test_float_type_68() {
    ({
        test_termination_condition_18(
            unimplemented!(),
            -({ std_numeric_limits_float_::max() }),
            None,
        )
    });
    ({
        test_termination_condition_18(
            unimplemented!(),
            -({ std_numeric_limits_float_::min() }),
            None,
        )
    });
    ({ test_termination_condition_18(unimplemented!(), -0_f64, None) });
    ({ test_termination_condition_18(unimplemented!(), 0_f64, None) });
    ({
        test_termination_condition_18(
            unimplemented!(),
            ({ std_numeric_limits_float_::min() }),
            None,
        )
    });
    ({
        test_termination_condition_18(
            unimplemented!(),
            ({ std_numeric_limits_float_::max() }),
            None,
        )
    });
    {
        ({
            test_termination_condition_18(
                unimplemented!(),
                -({ std_numeric_limits_double_::max() }),
                None,
            )
        });
        ({
            test_termination_condition_18(
                unimplemented!(),
                -({ std_numeric_limits_double_::min() }),
                None,
            )
        });
        ({
            test_termination_condition_18(
                unimplemented!(),
                ({ std_numeric_limits_double_::min() }),
                None,
            )
        });
        ({
            test_termination_condition_18(
                unimplemented!(),
                ({ std_numeric_limits_double_::max() }),
                None,
            )
        });
    }
    {
        ({
            test_termination_condition_18(
                unimplemented!(),
                -({ std_numeric_limits_long_double_::max() }),
                None,
            )
        });
        ({
            test_termination_condition_18(
                unimplemented!(),
                -({ std_numeric_limits_long_double_::min() }),
                None,
            )
        });
        ({
            test_termination_condition_18(
                unimplemented!(),
                ({ std_numeric_limits_long_double_::min() }),
                None,
            )
        });
        ({
            test_termination_condition_18(
                unimplemented!(),
                ({ std_numeric_limits_long_double_::max() }),
                None,
            )
        });
    }
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_18(
            _f,
            -({ std_numeric_limits_long_double_::infinity() }),
            Some(_expected),
        )
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_18(
            _f,
            ({ std_numeric_limits_long_double_::infinity() }),
            Some(_expected),
        )
    });
    let mut nan: () = ({ std_numeric_limits_long_double_::quiet_NaN() });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_18(_f, ({ copysign_69(nan, -1_f64) }), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_18(_f, nan, Some(_expected))
    });
    {}
}
pub fn test_float_type_70() {
    ({
        test_termination_condition_19(
            unimplemented!(),
            -({ std_numeric_limits_float_::max() }),
            None,
        )
    });
    ({
        test_termination_condition_19(
            unimplemented!(),
            -({ std_numeric_limits_float_::min() }),
            None,
        )
    });
    ({ test_termination_condition_19(unimplemented!(), -0_f64, None) });
    ({ test_termination_condition_19(unimplemented!(), 0_f64, None) });
    ({
        test_termination_condition_19(
            unimplemented!(),
            ({ std_numeric_limits_float_::min() }),
            None,
        )
    });
    ({
        test_termination_condition_19(
            unimplemented!(),
            ({ std_numeric_limits_float_::max() }),
            None,
        )
    });
    {}
    {}
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_19(
            _f,
            -({ std_numeric_limits_float_::infinity() }),
            Some(_expected),
        )
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_19(
            _f,
            ({ std_numeric_limits_float_::infinity() }),
            Some(_expected),
        )
    });
    let mut nan: () = ({ std_numeric_limits_float_::quiet_NaN() });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_20(_f, ({ copysign_65(nan, -1_f64) }), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_19(_f, nan, Some(_expected))
    });
    {
        ({ test_special_values_62() });
    }
}
pub fn test_float_type_71() {
    ({
        test_termination_condition_20(
            unimplemented!(),
            -({ std_numeric_limits_float_::max() }),
            None,
        )
    });
    ({
        test_termination_condition_20(
            unimplemented!(),
            -({ std_numeric_limits_float_::min() }),
            None,
        )
    });
    ({ test_termination_condition_20(unimplemented!(), -0_f64, None) });
    ({ test_termination_condition_20(unimplemented!(), 0_f64, None) });
    ({
        test_termination_condition_20(
            unimplemented!(),
            ({ std_numeric_limits_float_::min() }),
            None,
        )
    });
    ({
        test_termination_condition_20(
            unimplemented!(),
            ({ std_numeric_limits_float_::max() }),
            None,
        )
    });
    {
        ({
            test_termination_condition_20(
                unimplemented!(),
                -({ std_numeric_limits_double_::max() }),
                None,
            )
        });
        ({
            test_termination_condition_20(
                unimplemented!(),
                -({ std_numeric_limits_double_::min() }),
                None,
            )
        });
        ({
            test_termination_condition_20(
                unimplemented!(),
                ({ std_numeric_limits_double_::min() }),
                None,
            )
        });
        ({
            test_termination_condition_20(
                unimplemented!(),
                ({ std_numeric_limits_double_::max() }),
                None,
            )
        });
    }
    {}
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_20(
            _f,
            -({ std_numeric_limits_double_::infinity() }),
            Some(_expected),
        )
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_20(
            _f,
            ({ std_numeric_limits_double_::infinity() }),
            Some(_expected),
        )
    });
    let mut nan: () = ({ std_numeric_limits_double_::quiet_NaN() });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_20(_f, ({ copysign_67(nan, -1_f64) }), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_20(_f, nan, Some(_expected))
    });
    {
        ({ test_special_values_63() });
    }
}
pub fn test_float_type_72() {
    ({
        test_termination_condition_21(
            unimplemented!(),
            -({ std_numeric_limits_float_::max() }),
            None,
        )
    });
    ({
        test_termination_condition_21(
            unimplemented!(),
            -({ std_numeric_limits_float_::min() }),
            None,
        )
    });
    ({ test_termination_condition_21(unimplemented!(), -0_f64, None) });
    ({ test_termination_condition_21(unimplemented!(), 0_f64, None) });
    ({
        test_termination_condition_21(
            unimplemented!(),
            ({ std_numeric_limits_float_::min() }),
            None,
        )
    });
    ({
        test_termination_condition_21(
            unimplemented!(),
            ({ std_numeric_limits_float_::max() }),
            None,
        )
    });
    {
        ({
            test_termination_condition_21(
                unimplemented!(),
                -({ std_numeric_limits_double_::max() }),
                None,
            )
        });
        ({
            test_termination_condition_21(
                unimplemented!(),
                -({ std_numeric_limits_double_::min() }),
                None,
            )
        });
        ({
            test_termination_condition_21(
                unimplemented!(),
                ({ std_numeric_limits_double_::min() }),
                None,
            )
        });
        ({
            test_termination_condition_21(
                unimplemented!(),
                ({ std_numeric_limits_double_::max() }),
                None,
            )
        });
    }
    {
        ({
            test_termination_condition_21(
                unimplemented!(),
                -({ std_numeric_limits_long_double_::max() }),
                None,
            )
        });
        ({
            test_termination_condition_21(
                unimplemented!(),
                -({ std_numeric_limits_long_double_::min() }),
                None,
            )
        });
        ({
            test_termination_condition_21(
                unimplemented!(),
                ({ std_numeric_limits_long_double_::min() }),
                None,
            )
        });
        ({
            test_termination_condition_21(
                unimplemented!(),
                ({ std_numeric_limits_long_double_::max() }),
                None,
            )
        });
    }
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_21(
            _f,
            -({ std_numeric_limits_long_double_::infinity() }),
            Some(_expected),
        )
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_21(
            _f,
            ({ std_numeric_limits_long_double_::infinity() }),
            Some(_expected),
        )
    });
    let mut nan: () = ({ std_numeric_limits_long_double_::quiet_NaN() });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_21(_f, ({ copysign_69(nan, -1_f64) }), Some(_expected))
    });
    ({
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_21(_f, nan, Some(_expected))
    });
    {}
}
pub fn test_all_float_types_73() {
    ({ test_float_type_64() });
    ({ test_float_type_66() });
    ({ test_float_type_68() });
}
pub fn test_all_float_types_74() {
    ({ test_float_type_70() });
    ({ test_float_type_71() });
    ({ test_float_type_72() });
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
    ({ test_all_float_types_73() });
    ({ test_all_float_types_74() });
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
