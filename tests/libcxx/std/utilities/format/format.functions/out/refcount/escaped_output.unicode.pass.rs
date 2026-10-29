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
pub fn test_eprintf_0(
    mut fmt: Ptr<()>,
    args_1: Ptr<Ptr<()>>,
    args_2: Ptr<Ptr<()>>,
    args_3: Ptr<Ptr<()>>,
    args_4: Ptr<()>,
    args_5: Ptr<Ptr<()>>,
) {
    unimplemented!();
}
pub fn test_eprintf_1(
    mut fmt: Ptr<()>,
    args_1: Ptr<Ptr<()>>,
    args_2: Ptr<Ptr<()>>,
    args_3: Ptr<()>,
) {
    unimplemented!();
}
pub fn test_eprintf_2(mut fmt: Ptr<()>, args: Ptr<Ptr<()>>) {
    unimplemented!();
}
pub fn test_log_3(condition: Ptr<()>, file: Ptr<()>, line: (), message: Ptr<()>) {
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
        test_eprintf_0(
            Ptr::<()>::from_string_literal(b"%s%s %s %d\n%s"),
            msg.as_pointer(),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
            message.as_pointer(),
        )
    });
}
pub fn test_log_4(condition: Ptr<()>, file: Ptr<()>, line: (), functor: Ptr<FnPtr<fn()>>) {
    let condition: Value<Ptr<()>> = Rc::new(RefCell::new(condition));
    let file: Value<Ptr<()>> = Rc::new(RefCell::new(file));
    let line: Value<()> = Rc::new(RefCell::new(line));
    ({
        test_eprintf_1(
            Ptr::<()>::from_string_literal(b"Assertion failure: %s %s %d\n"),
            condition.as_pointer(),
            file.as_pointer(),
            line.as_pointer(),
        )
    });
    ({ (*functor.upgrade().deref()).call() });
}
pub fn test_require_5(
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
        test_log_4(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub fn test_is_high_surrogate_6(mut value: ()) -> () {
    return (value >= 55296_()) && (value <= 56319_());
}
pub fn test_is_low_surrogate_7(mut value: ()) -> () {
    return (value >= 56320_()) && (value <= 57343_());
}
pub fn test_is_surrogate_8(mut value: ()) -> () {
    return (value >= 55296_()) && (value <= 57343_());
}
pub fn test_is_code_point_9(mut value: ()) -> () {
    return (value <= 1114111_());
}
pub fn test_is_scalar_value_10(mut value: ()) -> () {
    return ({ test_is_code_point_9(value) }) && (!({ test_is_surrogate_8(value) }));
}
thread_local!(
    pub static test_replacement_character_11: Value<()> = Rc::new(RefCell::new((65533 as ())));
);
pub fn test_encode_12(out_it: Ptr<()>, mut value: ()) {
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
pub fn test_encode_13(out_it: Ptr<()>, mut value: ()) {
    if ((value & 4294901760_()) == 0_()) {
        ({
            let _out_it: Ptr<()> = (out_it).clone();
            let _value: () = value;
            test_encode_12(_out_it, _value)
        });
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_transcode_14(first: (), last: (), out_it: ()) -> () {
    let first: Value<()> = Rc::new(RefCell::new(first));
    let last: Value<()> = Rc::new(RefCell::new(last));
    let out_it: Value<()> = Rc::new(RefCell::new(out_it));
    'loop_: while !(unimplemented!()) {
        let mut value: () = (unimplemented!().read());
        if ({ test_is_code_point_9(value) }) {
            ({ test_encode_13(out_it.as_pointer(), value) });
        } else {
            ({
                test_encode_12(
                    out_it.as_pointer(),
                    test_replacement_character_11.with(|rc| *rc.borrow()),
                )
            });
        }
    }
    return unimplemented!();
}
pub fn test_transcode_15(mut first: Ptr<()>, mut last: Ptr<()>, out_it: ()) -> () {
    let out_it: Value<()> = Rc::new(RefCell::new(out_it));
    'loop_: while ({ (first).clone() } != { (last).clone() }) {
        let mut value: () = (first.postfix_inc().read());
        if ({ test_is_code_point_9(value) }) {
            ({ test_encode_13(out_it.as_pointer(), value) });
        } else {
            ({
                test_encode_12(
                    out_it.as_pointer(),
                    test_replacement_character_11.with(|rc| *rc.borrow()),
                )
            });
        }
    }
    return unimplemented!();
}
pub fn operator_shl_16(os: Ptr<()>, loc: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn test_concat_17(stream: Ptr<()>, value: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn test_concat_18(stream: Ptr<()>, value: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn test_concat_19(stream: Ptr<()>, value: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn test_concat_20(stream: Ptr<()>, value: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn test_concat_21(stream: Ptr<()>, value: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn test_concat_22(stream: Ptr<()>, value: Ptr<()>) -> Ptr<()> {
    let b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if !(unimplemented!()) {
        {}
        ({ test_transcode_14(unimplemented!(), unimplemented!(), unimplemented!()) });
    }
    return (stream).clone();
}
pub fn test_concat_23(stream: Ptr<()>, value: Ptr<()>) -> Ptr<()> {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ({ (b).clone() } != { (e).clone() }) {
        {}
        ({ test_transcode_15((b).clone(), (e).clone(), unimplemented!()) });
    }
    return (stream).clone();
}
pub fn test_concat_24(stream: Ptr<()>, value: Ptr<()>) -> Ptr<()> {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ({ (b).clone() } != { (e).clone() }) {
        {}
        ({ test_transcode_15((b).clone(), (e).clone(), unimplemented!()) });
    }
    return (stream).clone();
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
                test_concat_19(_stream, _value)
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
                        test_concat_19(_stream, _value)
                    });
                    {
                        ({
                            let _stream: Ptr<()> = sstr.as_pointer();
                            let _value: Ptr<()> = (args_3).clone();
                            test_concat_20(_stream, _value)
                        });
                        {
                            ({
                                let _stream: Ptr<()> = sstr.as_pointer();
                                let _value: Ptr<()> = (args_4).clone();
                                test_concat_19(_stream, _value)
                            });
                            {
                                ({
                                    let _stream: Ptr<()> = sstr.as_pointer();
                                    let _value: Ptr<()> = (args_5).clone();
                                    test_concat_18(_stream, _value)
                                });
                                ({
                                    let _stream: Ptr<()> = sstr.as_pointer();
                                    let _value: Ptr<()> = (args_6).clone();
                                    test_concat_17(_stream, _value)
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
pub fn test_concat_message_26(
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
                test_concat_19(_stream, _value)
            });
            {
                ({
                    let _stream: Ptr<()> = sstr.as_pointer();
                    let _value: Ptr<()> = (args_1).clone();
                    test_concat_24(_stream, _value)
                });
                {
                    ({
                        let _stream: Ptr<()> = sstr.as_pointer();
                        let _value: Ptr<()> = (args_2).clone();
                        test_concat_19(_stream, _value)
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
                                test_concat_19(_stream, _value)
                            });
                            {
                                ({
                                    let _stream: Ptr<()> = sstr.as_pointer();
                                    let _value: Ptr<()> = (args_5).clone();
                                    test_concat_22(_stream, _value)
                                });
                                ({
                                    let _stream: Ptr<()> = sstr.as_pointer();
                                    let _value: Ptr<()> = (args_6).clone();
                                    test_concat_17(_stream, _value)
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
thread_local!(
    pub static test_format_27: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_5(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_28(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_29(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_30(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_31(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_32(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args_2: Ptr<()>, args_3: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_33(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_34(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_35(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args_2: Ptr<()>, args_3: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_36(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_37(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_38(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_39(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_40(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                ({
                    let _condition: () = unimplemented!();
                    let _arg: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
                        {
                            let fmt: Ptr<()> = fmt.as_pointer();
                            let expected: Ptr<()> = expected.as_pointer();
                            let out: Ptr<()> = out.as_pointer();
                        },
                        || {
                            ({
                                let _args: Value<Ptr<()>> = Rc::new(RefCell::new(unimplemented!()));
                                test_eprintf_2(
                                    Ptr::<()>::from_string_literal(b"%s"),
                                    _args.as_pointer(),
                                )
                            });
                        }
                    )));
                    test_require_41(
                        _condition,
                        Ptr::<()>::from_string_literal(b"out == expected"),
                        Ptr::<()>::from_string_literal(b"escaped_output.unicode.pass.cpp"),
                        50_(),
                        _arg.as_pointer(),
                    )
                });
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
            }
        }
    )));
);
thread_local!(
    pub static test_format_to_42: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let mut out: () = unimplemented!();
                let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
            }
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                unimplemented!();
                let out: Value<Box<[()]>> = Rc::new(RefCell::new(
                    (0..4096).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                ));
                let mut it: Ptr<()> = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let mut out: () = unimplemented!();
                let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
            }
            {
                let out: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                unimplemented!();
                let out: Value<Box<[()]>> = Rc::new(RefCell::new(
                    (0..4096).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                ));
                let mut it: Ptr<()> = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
    )));
);
thread_local!(
    pub static test_formatted_size_43: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let mut size: () = unimplemented!();
                unimplemented!();
            }
            {
                let mut size: () = unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let mut size: () = unimplemented!();
                unimplemented!();
            }
            {
                let mut size: () = unimplemented!();
                unimplemented!();
            }
        }
    )));
);
thread_local!(
    pub static test_format_to_n_44: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let mut n: () = unimplemented!();
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut n: () = unimplemented!();
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut n: () = 0_();
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut n: () = (unimplemented!()).wrapping_div(2_());
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |expected: (), fmt: (), args: Ptr<()>| {
            let expected: Value<()> = Rc::new(RefCell::new(expected));
            let fmt: Value<()> = Rc::new(RefCell::new(fmt));
            {
                let mut n: () = unimplemented!();
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut n: () = unimplemented!();
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut n: () = 0_();
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut n: () = (unimplemented!()).wrapping_div(2_());
                let mut out: () = unimplemented!();
                let result: Value<()> = Rc::new(RefCell::new(unimplemented!()));
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
    )));
);
pub fn test_char_45() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: Value<()> = Rc::new(RefCell::new(('\'' as ())));
        let _args_3: Value<()> = Rc::new(RefCell::new(('\"' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>, Ptr<()>)>(6)
            .call(_expected, _fmt, _args_2.as_pointer(), _args_3.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\t' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\n' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\r' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\\' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\'' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\"' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new((' ' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('a' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('b' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('c' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\0' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\x1f' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(7)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            let _args: Value<()> = Rc::new(RefCell::new((b'\x80' as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(7)
                .call(_expected, _fmt, _args.as_pointer())
        });
    }
    {}
    {}
}
pub fn test_char_46() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: Value<()> = Rc::new(RefCell::new(('\'' as ())));
        let _args_3: Value<()> = Rc::new(RefCell::new(('\"' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>, Ptr<()>)>(9)
            .call(_expected, _fmt, _args_2.as_pointer(), _args_3.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\t' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\n' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\r' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\\' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\'' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\"' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new((' ' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('a' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('b' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('c' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\0' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(('\x1f' as ())));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(10)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    {
        ({
            let _args: Value<()> = Rc::new(RefCell::new((160 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((12288 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((8232 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((8233 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((173 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((1536 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((65279 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((55296 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((57343 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((57344 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((63743 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((888 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((6004 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((65535 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((768 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((65056 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
    }
    {
        const _: () = assert!((::std::mem::size_of::<()>() == 4_()), "sizeof(CharT) == 4");;
        let mut x: () = 131071_();
        let mut y: () = 131071_();
        const _: () = assert!(true, "x == y");;
        ({
            let _args: Value<()> = Rc::new(RefCell::new((69821 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((917631 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((983040 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((1048573 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((1048576 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((1114109 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((65548 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((1048575 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((1114110 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((66045 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((917760 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((1114112 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
        ({
            let _args: Value<()> = Rc::new(RefCell::new((4294967295 as ())));
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(10)
                .call(unimplemented!(), unimplemented!(), _args.as_pointer())
        });
    }
}
pub fn test_string_47() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(0)
                .call(_expected, _fmt, Ptr::<()>::from_string_literal(b"\xc3"))
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(1)
                .call(_expected, _fmt, Ptr::<()>::from_string_literal(b"\xc3("))
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(1)
                .call(_expected, _fmt, Ptr::<()>::from_string_literal(b"\xc0\x80"))
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(1)
                .call(_expected, _fmt, Ptr::<()>::from_string_literal(b"\xc1\xbf"))
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(1)
                .call(_expected, _fmt, Ptr::<()>::from_string_literal(b"\xc2\x80"))
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xe0\x80\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xe0\x81\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xe0\x82\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xe0\x9f\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xe0\xa0\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xed\x9f\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xed\xa0\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xed\xaf\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xed\xbf\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xed\xbf\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(2)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xee\x80\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xf0\x80\x80\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xf0\x80\x81\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xf0\x80\x82\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xf0\x80\x9f\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xf0\x80\xa0\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xf0\x8f\xbf\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xf0\x90\x80\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xf4\x8f\xbf\xbf"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xf4\x90\x80\x80"),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(3)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(b"\xf4\xbf\xbf\xbf"),
                )
        });
    }
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(0)
                .call(_expected, _fmt, Ptr::<()>::from_string_literal(b"\x80"))
        });
    }
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    {}
}
pub fn test_string_48() {
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(5)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(&[195 as (), 0 as ()]),
                )
        });
        ({
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(12)
                .call(
                    _expected,
                    _fmt,
                    Ptr::<()>::from_string_literal(&[195 as (), 40 as (), 0 as ()]),
                )
        });
    }
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {}
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    {
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[160 as (), 12288 as (), 0 as ()]),
                )
        });
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(5)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[8232 as (), 0 as ()]),
                )
        });
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(5)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[8233 as (), 0 as ()]),
                )
        });
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(13)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[173 as (), 1536 as (), 65279 as (), 0 as ()]),
                )
        });
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(5)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[55296 as (), 0 as ()]),
                )
        });
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[57344 as (), 63743 as (), 0 as ()]),
                )
        });
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(13)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[888 as (), 6004 as (), 65535 as (), 0 as ()]),
                )
        });
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[768 as (), 65056 as (), 0 as ()]),
                )
        });
    }
    {
        const _: () = assert!((::std::mem::size_of::<()>() == 4_()), "sizeof(CharT) == 4");;
        let mut x: () = 131071_();
        let mut y: () = 131071_();
        const _: () = assert!(true, "x == y");;
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[69821 as (), 917631 as (), 0 as ()]),
                )
        });
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(14)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[
                        983040 as (),
                        1048573 as (),
                        1048576 as (),
                        1114109 as (),
                        0 as (),
                    ]),
                )
        });
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(13)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[
                        65548 as (),
                        1048575 as (),
                        1114110 as (),
                        0 as (),
                    ]),
                )
        });
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[66045 as (), 917760 as (), 0 as ()]),
                )
        });
        ({
            (*test_format_27.with(Value::clone).borrow())
                .spec::<fn((), (), Ptr<()>)>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    Ptr::<()>::from_string_literal(&[1114112 as (), 4294967295 as (), 0 as ()]),
                )
        });
    }
}
pub fn test_format_functions_49(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(8)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_50(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_51(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_52(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_53(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(11)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_54(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_55(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_format_functions_56(check: FnPtr<Generic>) {
    let check: Value<FnPtr<Generic>> = Rc::new(RefCell::new(check));
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
    ({
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        (*check.borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(_expected, _fmt, _args.as_pointer())
    });
}
pub fn test_57() {
    ({ test_char_45() });
    ({ test_string_47() });
    ({ test_format_functions_49((*test_format_27.with(Value::clone).borrow()).copy_from()) });
    ({ test_format_functions_50((*test_format_to_42.with(Value::clone).borrow()).copy_from()) });
    ({
        test_format_functions_51((*test_formatted_size_43.with(Value::clone).borrow()).copy_from())
    });
    ({ test_format_functions_52((*test_format_to_n_44.with(Value::clone).borrow()).copy_from()) });
}
pub fn test_58() {
    ({ test_char_46() });
    ({ test_string_48() });
    ({ test_format_functions_53((*test_format_27.with(Value::clone).borrow()).copy_from()) });
    ({ test_format_functions_54((*test_format_to_42.with(Value::clone).borrow()).copy_from()) });
    ({
        test_format_functions_55((*test_formatted_size_43.with(Value::clone).borrow()).copy_from())
    });
    ({ test_format_functions_56((*test_format_to_n_44.with(Value::clone).borrow()).copy_from()) });
}
pub fn test_ill_formed_utf8_59() {
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xdf"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xef"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xef\xbf"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(0)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xf7"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xf7\xbf"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(2)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xf7\xbf\xbf"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xdfa"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xefa"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(2)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xef\xbfa"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xf7a"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(2)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xf7\xbfa"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xf7\xbf\xbfa"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(4)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"a\xf1\x80\x80\xe1\x80\xc2b"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xf4\x8f\xbf\xbf"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xf4\x90\x80\x80"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xf5\xb1\xb2\xb3"),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(b"\xf7\xbf\xbf\xbf"),
            )
    });
}
pub fn test_ill_formed_utf32_60() {
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(5)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(&[1114111 as (), 0 as ()]),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(5)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(&[1114112 as (), 0 as ()]),
            )
    });
    ({
        (*test_format_27.with(Value::clone).borrow())
            .spec::<fn((), (), Ptr<()>)>(5)
            .call(
                unimplemented!(),
                unimplemented!(),
                Ptr::<()>::from_string_literal(&[4294967295 as (), 0 as ()]),
            )
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
    ({ test_57() });
    ({ test_58() });
    ({ test_ill_formed_utf8_59() });
    ({ test_ill_formed_utf32_60() });
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
    let _ = test_replacement_character_11.with(|_| ());
    let _ = test_format_27.with(|_| ());
    let _ = test_format_to_42.with(|_| ());
    let _ = test_formatted_size_43.with(|_| ());
    let _ = test_format_to_n_44.with(|_| ());
}
