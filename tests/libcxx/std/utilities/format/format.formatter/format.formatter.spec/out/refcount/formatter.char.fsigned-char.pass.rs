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
pub fn test_eprintf_4(
    mut fmt: Ptr<()>,
    args_1: Ptr<Ptr<()>>,
    args_2: Ptr<Ptr<()>>,
    args_3: Ptr<Ptr<()>>,
    args_4: Ptr<()>,
    args_5: Ptr<Ptr<()>>,
) {
    ({
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            (fmt).clone() as *const libc::c_char,
            (args_1.read()),
            (args_2.read()),
            (args_3.read()),
            (args_4.read()),
            (args_5.read()),
        )
    });
}
pub fn test_eprintf_5(
    mut fmt: Ptr<()>,
    args_1: Ptr<Ptr<()>>,
    args_2: Ptr<Ptr<()>>,
    args_3: Ptr<()>,
) {
    ({
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            (fmt).clone() as *const libc::c_char,
            (args_1.read()),
            (args_2.read()),
            (args_3.read()),
        )
    });
}
pub fn test_eprintf_6(mut fmt: Ptr<()>, args: Ptr<Ptr<()>>) {
    ({
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            (fmt).clone() as *const libc::c_char,
            (args.read()),
        )
    });
}
pub fn test_log_7(condition: Ptr<()>, file: Ptr<()>, line: (), message: Ptr<()>) {
    let condition: Value<Ptr<()>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<()>> = Rc::new(RefCell::new(file));
    let line: Value<()> = Rc::new(RefCell::new(line));
    let message: Value<Ptr<()>> = Rc::new(RefCell::new(message));
    let msg: Value<Ptr<()>> = Rc::new(RefCell::new(if !(*condition.borrow()).is_null() {
        Ptr::<()>::from_string_literal(b"Assertion failure: ")
    } else {
        Ptr::<()>::from_string_literal(b"Unconditional failure:")
    }));
    ({
        test_eprintf_4(
            Ptr::<()>::from_string_literal(b"%s%s %s %d\n%s"),
            msg.as_pointer(),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
            message.as_pointer(),
        )
    });
}
pub fn test_log_8(condition: Ptr<()>, file: Ptr<()>, line: (), functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<()>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<()>> = Rc::new(RefCell::new(file));
    let line: Value<()> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_5(
            Ptr::<()>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_require_9(
    mut condition: (),
    mut condition_str: Ptr<()>,
    mut file: Ptr<()>,
    mut line: (),
    arg: Ptr<FnPtr<fn()>>,
) {
    if condition {
        return;
    }
    ({
        let _condition: Ptr<()> = (condition_str).clone();
        let _file: Ptr<()> = (file).clone();
        let _line: () = line;
        let _functor: Ptr<FnPtr<fn()>> = (arg).clone();
        test_log_8(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub fn test_is_high_surrogate_10(mut value: ()) -> () {
    return (value >= 55296_()) && (value <= 56319_());
}
pub fn test_is_low_surrogate_11(mut value: ()) -> () {
    return (value >= 56320_()) && (value <= 57343_());
}
pub fn test_is_surrogate_12(mut value: ()) -> () {
    return (value >= 55296_()) && (value <= 57343_());
}
pub fn test_is_code_point_13(mut value: ()) -> () {
    return (value <= 1114111_());
}
pub fn test_is_scalar_value_14(mut value: ()) -> () {
    return ({ test_is_code_point_13(value) }) && (!({ test_is_surrogate_12(value) }));
}
thread_local!(
    pub static test_replacement_character_15: Value<()> = Rc::new(RefCell::new((65533 as ())));
);
pub fn test_encode_16(out_it: Ptr<()>, mut value: ()) {
    if (value < 128_()) {
        unimplemented!();
    } else if (value < 2048_()) {
        unimplemented!();
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_encode_17(out_it: Ptr<()>, mut value: ()) {
    if ((value & 4294901760_()) == 0_()) {
        ({
            let _out_it: Ptr<()> = (out_it).clone();
            let _value: () = value;
            test_encode_16(_out_it, _value)
        });
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_transcode_18(first: (), last: (), out_it: ()) -> () {
    let first: Value<()> = Rc::new(RefCell::new(first));
    let last: Value<()> = Rc::new(RefCell::new(last));
    let out_it: Value<()> = Rc::new(RefCell::new(out_it));
    'loop_: while !(unimplemented!()) {
        let mut value: () = (unimplemented!().read());
        if ({ test_is_code_point_13(value) }) {
            ({ test_encode_17(out_it.as_pointer(), value) });
        } else {
            ({
                test_encode_16(
                    out_it.as_pointer(),
                    test_replacement_character_15.with(|rc| *rc.borrow()),
                )
            });
        }
    }
    return (*out_it.borrow_mut()).clone();
}
pub fn operator_shl_19(os: Ptr<()>, loc: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn test_concat_20(stream: Ptr<()>, value: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn test_concat_21(stream: Ptr<()>, value: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn test_concat_22(stream: Ptr<()>, value: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn test_concat_23(stream: Ptr<()>, value: Ptr<()>) -> Ptr<()> {
    let b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if !(unimplemented!()) {
        {}
        ({
            test_transcode_18(
                (*b.borrow()).clone(),
                (*e.borrow()).clone(),
                unimplemented!(),
            )
        });
    }
    return (stream).clone();
}
pub fn test_concat_message_24(
    args_0: Ptr<()>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
    args_4: Ptr<()>,
    args_5: Ptr<()>,
    args_6: Ptr<()>,
) -> () {
    {
        let sstr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            ({
                let _stream: Ptr<()> = sstr.as_pointer();
                let _value: Ptr<()> = (args_0).clone();
                test_concat_22(_stream, _value)
            });
            {
                ({
                    let _stream: Ptr<()> = sstr.as_pointer();
                    let _value: Ptr<()> = (args_1).clone();
                    test_concat_21(_stream, _value)
                });
                {
                    ({
                        let _stream: Ptr<()> = sstr.as_pointer();
                        let _value: Ptr<()> = (args_2).clone();
                        test_concat_22(_stream, _value)
                    });
                    {
                        ({
                            let _stream: Ptr<()> = sstr.as_pointer();
                            let _value: Ptr<()> = (args_3).clone();
                            test_concat_21(_stream, _value)
                        });
                        {
                            ({
                                let _stream: Ptr<()> = sstr.as_pointer();
                                let _value: Ptr<()> = (args_4).clone();
                                test_concat_22(_stream, _value)
                            });
                            {
                                ({
                                    let _stream: Ptr<()> = sstr.as_pointer();
                                    let _value: Ptr<()> = (args_5).clone();
                                    test_concat_21(_stream, _value)
                                });
                                ({
                                    let _stream: Ptr<()> = sstr.as_pointer();
                                    let _value: Ptr<()> = (args_6).clone();
                                    test_concat_20(_stream, _value)
                                })
                            }
                        }
                    }
                }
            }
        };
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub fn test_concat_message_25(
    args_0: Ptr<()>,
    args_1: Ptr<()>,
    args_2: Ptr<()>,
    args_3: Ptr<()>,
    args_4: Ptr<()>,
    args_5: Ptr<()>,
    args_6: Ptr<()>,
) -> () {
    {
        let sstr: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        {
            ({
                let _stream: Ptr<()> = sstr.as_pointer();
                let _value: Ptr<()> = (args_0).clone();
                test_concat_22(_stream, _value)
            });
            {
                ({
                    let _stream: Ptr<()> = sstr.as_pointer();
                    let _value: Ptr<()> = (args_1).clone();
                    test_concat_23(_stream, _value)
                });
                {
                    ({
                        let _stream: Ptr<()> = sstr.as_pointer();
                        let _value: Ptr<()> = (args_2).clone();
                        test_concat_22(_stream, _value)
                    });
                    {
                        ({
                            let _stream: Ptr<()> = sstr.as_pointer();
                            let _value: Ptr<()> = (args_3).clone();
                            test_concat_23(_stream, _value)
                        });
                        {
                            ({
                                let _stream: Ptr<()> = sstr.as_pointer();
                                let _value: Ptr<()> = (args_4).clone();
                                test_concat_22(_stream, _value)
                            });
                            {
                                ({
                                    let _stream: Ptr<()> = sstr.as_pointer();
                                    let _value: Ptr<()> = (args_5).clone();
                                    test_concat_23(_stream, _value)
                                });
                                ({
                                    let _stream: Ptr<()> = sstr.as_pointer();
                                    let _value: Ptr<()> = (args_6).clone();
                                    test_concat_20(_stream, _value)
                                })
                            }
                        }
                    }
                }
            }
        };
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub fn test_26(expected: (), fmt: (), arg: ()) {
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    ({ ((*formatter.borrow()) as std___formatter_char_char_).parse(parse_ctx.as_pointer()) });
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_0((*out.borrow()).clone(), {
                let __tmp_0 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___char_ > = Rc::new(RefCell::new(( { make_format_args_27 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new ( { __tmp_0.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_char_char_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    ({
        let _condition: () = unimplemented!();
        let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
            {
                let fmt: Ptr<()> = fmt.as_pointer();
                let expected: Ptr<()> = expected.as_pointer();
                let result: Ptr<()> = result.as_pointer();
            },
            || {
                ({
                    let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                    test_eprintf_6(Ptr::<()>::from_string_literal(b"%s"), _args.as_pointer())
                });
            }
        )));
        test_require_9(
            _condition,
            Ptr::<()>::from_string_literal(b"result == expected"),
            Ptr::<()>::from_string_literal(b"formatter.char.fsigned-char.pass.cpp"),
            57_(),
            _arg.as_pointer(),
        )
    });
}
pub fn test_28(expected: (), fmt: (), arg: ()) {
    let expected: Value<()> = Rc::new(RefCell::new(expected));
    let fmt: Value<()> = Rc::new(RefCell::new(fmt));
    let arg: Value<()> = Rc::new(RefCell::new(arg));
    let parse_ctx: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let formatter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    ({ ((*formatter.borrow()) as std___formatter_char_wchar_t_).parse(parse_ctx.as_pointer()) });
    let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let format_ctx: Value<()> = Rc::new(RefCell::new(
        ({
            test_format_context_create_2((*out.borrow()).clone(), {
                let __tmp_1 : Value<std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___char_ > = Rc::new(RefCell::new(( { make_format_args_29 ( arg .as_pointer()  , ) } )  ));
                std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new ( { __tmp_1.as_pointer() } , )
            })
        }),
    ));
    ({
        ((*formatter.borrow()) as std___formatter_char_wchar_t_)
            .format((*arg.borrow()), format_ctx.as_pointer())
    });
    ({
        let _condition: () = unimplemented!();
        let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
            {
                let fmt: Ptr<()> = fmt.as_pointer();
                let expected: Ptr<()> = expected.as_pointer();
                let result: Ptr<()> = result.as_pointer();
            },
            || {
                ({
                    let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                    test_eprintf_6(Ptr::<()>::from_string_literal(b"%s"), _args.as_pointer())
                });
            }
        )));
        test_require_30(
            _condition,
            Ptr::<()>::from_string_literal(b"result == expected"),
            Ptr::<()>::from_string_literal(b"formatter.char.fsigned-char.pass.cpp"),
            57_(),
            _arg.as_pointer(),
        )
    });
}
pub fn test_31() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, ('\0' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, ('a' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, (b'\x80' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, (b'\xff' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, ('\0' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, ('a' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, (b'\x80' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, (b'\xff' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, ('\0' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, ('a' as ()))
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            test_26(_expected, _fmt, (b'\x80' as ()))
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            test_26(_expected, _fmt, (b'\xff' as ()))
        });
    }
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -128_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 127_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -128_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 127_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -128_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 127_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -128_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 127_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -128_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 127_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -128_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 127_())
    });
}
pub fn test_32() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, ('\0' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, ('a' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, (b'\x80' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, (b'\xff' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, ('\0' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, ('a' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, (b'\x80' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, (b'\xff' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, ('\0' as ()))
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, ('a' as ()))
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            test_28(_expected, _fmt, (b'\x80' as ()))
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            test_28(_expected, _fmt, (b'\xff' as ()))
        });
    }
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, -128_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, -1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 127_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, -128_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, -1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 127_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, -128_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, -1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 127_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, -128_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, -1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 127_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, -128_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, -1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 127_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, -128_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, -1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 0_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 1_())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_28(_expected, _fmt, 127_())
    });
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
    ({ test_31() });
    ({ test_32() });
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
pub fn __cpp2rust_init_globals() {
    let _ = test_replacement_character_15.with(|_| ());
}
