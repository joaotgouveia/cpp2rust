extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_eprintf_0(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const *const (),
    args_3: *const *const (),
    args_4: *const (),
    args_5: *const *const (),
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
            (*args_3),
            (*args_4),
            (*args_5),
        )
    });
}
pub unsafe fn test_eprintf_1(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const *const (),
    args_3: *const (),
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
            (*args_3),
        )
    });
}
pub unsafe fn test_eprintf_2(mut fmt: *const (), args: *const *const ()) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args),
        )
    });
}
pub unsafe fn test_log_3(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    mut message: *const (),
) {
    let mut msg: *const () = if !(condition).is_null() {
        c"Assertion failure: ".as_ptr()
    } else {
        c"Unconditional failure:".as_ptr()
    };
    (unsafe {
        test_eprintf_0(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
pub unsafe fn test_log_4(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_require_5(
    mut condition: (),
    mut condition_str: *const (),
    mut file: *const (),
    mut line: (),
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const () = condition_str;
        let _file: *const () = file;
        let _line: () = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_4(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_is_high_surrogate_6(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (56319_()));
}
pub unsafe fn test_is_low_surrogate_7(mut value: ()) -> () {
    return ((value) >= (56320_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_surrogate_8(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_code_point_9(mut value: ()) -> () {
    return ((value) <= (1114111_()));
}
pub unsafe fn test_is_scalar_value_10(mut value: ()) -> () {
    return (unsafe { test_is_code_point_9(value) }) && (!(unsafe { test_is_surrogate_8(value) }));
}
pub static mut test_replacement_character_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (65533 as ()) });
pub unsafe fn test_encode_12(out_it: *mut (), mut value: ()) {
    if ((value) < (128_())) {
        unimplemented!();
    } else if ((value) < (2048_())) {
        unimplemented!();
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_encode_13(out_it: *mut (), mut value: ()) {
    if (((value) & (4294901760_())) == (0_())) {
        (unsafe {
            let _out_it: *mut () = out_it;
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
pub unsafe fn test_transcode_14(mut first: (), mut last: (), mut out_it: ()) -> () {
    'loop_: while !(unimplemented!()) {
        let mut value: () = (*unimplemented!());
        if (unsafe { test_is_code_point_9(value) }) {
            (unsafe { test_encode_13(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_12(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_11)),
                )
            });
        }
    }
    return out_it;
}
pub unsafe fn test_transcode_15(mut first: *const (), mut last: *const (), mut out_it: ()) -> () {
    'loop_: while ((first) != (last)) {
        let mut value: () = (*first.postfix_inc());
        if (unsafe { test_is_code_point_9(value) }) {
            (unsafe { test_encode_13(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_12(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_11)),
                )
            });
        }
    }
    return out_it;
}
pub unsafe fn operator_shl_16(os: *mut (), loc: *const ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_17(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_18(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_19(stream: *mut (), value: *const [(); 18]) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_20(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_21(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_22(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if !(unimplemented!()) {
        {}
        (unsafe { test_transcode_14(b.clone(), e.clone(), unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_23(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ((b) != (e)) {
        {}
        (unsafe { test_transcode_15(b, e, unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_24(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ((b) != (e)) {
        {}
        (unsafe { test_transcode_15(b, e, unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_message_25(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        let mut sstr: () = unimplemented!();
        {
            (unsafe {
                let _stream: *mut () = &mut sstr;
                let _value: *const [(); 18] = args_0;
                test_concat_19(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_21(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_19(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_20(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_19(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_18(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
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
pub unsafe fn test_concat_message_26(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        let mut sstr: () = unimplemented!();
        {
            (unsafe {
                let _stream: *mut () = &mut sstr;
                let _value: *const [(); 18] = args_0;
                test_concat_19(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_24(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_19(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_23(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_19(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_22(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MultiStringType {
    pub w_: *const (),
    pub wn_: (),
    pub u8_: *const (),
    pub u8n_: (),
    pub u16_: *const (),
    pub u16n_: (),
    pub u32_: *const (),
    pub u32n_: (),
    pub s_: *const (),
    pub sn_: (),
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const (),
        mut wn: (),
        mut u8: *const (),
        mut u8n: (),
        mut u16: *const (),
        mut u16n: (),
        mut u32: *const (),
        mut u32n: (),
        mut s: *const (),
        mut sn: (),
    ) -> Self {
        let mut this = Self {
            w_: w,
            wn_: wn,
            u8_: u8,
            u8n_: u8n,
            u16_: u16,
            u16n_: u16n,
            u32_: u32,
            u32n_: u32n,
            s_: s,
            sn_: sn,
        };
        this
    }
    pub unsafe fn as_ptr_2(&self, mut _a0: *const ()) -> *const () {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const ()) -> *const () {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const ()) -> *const () {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const ()) -> *const () {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const ()) -> *const () {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const ()) -> () {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const ()) -> () {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const ()) -> () {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const ()) -> () {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const ()) -> () {
        return self.u32n_;
    }
    pub unsafe fn to_pconst_12(&self) -> *const () {
        return self.s_;
    }
    pub unsafe fn to_pconst_13(&self) -> *const () {
        return self.w_;
    }
    pub unsafe fn to_pconst_14(&self) -> *const () {
        return self.u8_;
    }
    pub unsafe fn to_pconst_15(&self) -> *const () {
        return self.u16_;
    }
    pub unsafe fn to_pconst_16(&self) -> *const () {
        return self.u32_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct context_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct context_wchar_t_ {}
pub type status = u16;
pub const status_foo: status = 43690;
pub const status_bar: status = 21845;
pub const status_foobar: status = 43605;
pub unsafe fn operator_cmp_27(
    lhs: *const parse_call_validator,
    rhs: *const parse_call_validator,
) -> () {
    return std::cmp::Ord::cmp(&(lhs), &(rhs));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct parse_call_validator_parse_function_not_called {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct parse_call_validator {}
impl std::cmp::Ord for parse_call_validator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_27(
                self as *const parse_call_validator,
                other as *const parse_call_validator,
            )
        }
    }
}
impl std::cmp::PartialOrd for parse_call_validator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for parse_call_validator {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_cmp_27(
                self as *const parse_call_validator,
                other as *const parse_call_validator,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for parse_call_validator {}
pub unsafe fn get_format_types_28() -> () {
    return unimplemented!();
}
pub static mut check_29: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (), mut fmt: std_basic_format_string_char__int_, args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut std_basic_format_string_char__int_ = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_5(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"fill.unicode.pass.cpp".as_ptr(),
                        40_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_char__std_string_view_,
             args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut std_basic_format_string_char__std_string_view_ = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_30(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"fill.unicode.pass.cpp".as_ptr(),
                        40_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: std_basic_format_string_wchar_t__int_, args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut std_basic_format_string_wchar_t__int_ = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_31(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"fill.unicode.pass.cpp".as_ptr(),
                        40_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_wchar_t__std_wstring_view_,
             args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut std_basic_format_string_wchar_t__std_wstring_view_ =
                                &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_2(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_32(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"fill.unicode.pass.cpp".as_ptr(),
                        40_(),
                        &mut _arg,
                    )
                });
            }
        )
    });
pub static mut check_exception_33: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {}
        )
    });
pub unsafe fn test_34() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__int_ = {
            let mut __tmp_0: () = unimplemented!();
            std_basic_format_string_char__int_::new({ &mut __tmp_0 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_char__int_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__int_ = {
            let mut __tmp_1: () = unimplemented!();
            std_basic_format_string_char__int_::new({ &mut __tmp_1 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_char__int_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__int_ = {
            let mut __tmp_2: () = unimplemented!();
            std_basic_format_string_char__int_::new({ &mut __tmp_2 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_char__int_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__int_ = {
            let mut __tmp_3: () = unimplemented!();
            std_basic_format_string_char__int_::new({ &mut __tmp_3 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_char__int_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__int_ = {
            let mut __tmp_4: () = unimplemented!();
            std_basic_format_string_char__int_::new({ &mut __tmp_4 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_char__int_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__int_ = {
            let mut __tmp_5: () = unimplemented!();
            std_basic_format_string_char__int_::new({ &mut __tmp_5 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_char__int_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__int_ = {
            let mut __tmp_6: () = unimplemented!();
            std_basic_format_string_char__int_::new({ &mut __tmp_6 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_char__int_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__int_ = {
            let mut __tmp_7: () = unimplemented!();
            std_basic_format_string_char__int_::new({ &mut __tmp_7 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_char__int_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_8: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_8 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_9: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_9 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_10: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_10 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
    }
}
pub unsafe fn test_35() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__int_ = {
            let mut __tmp_11: () = unimplemented!();
            std_basic_format_string_wchar_t__int_::new({ &mut __tmp_11 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_wchar_t__int_, *mut ())>(2)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__int_ = {
            let mut __tmp_12: () = unimplemented!();
            std_basic_format_string_wchar_t__int_::new({ &mut __tmp_12 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_wchar_t__int_, *mut ())>(2)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__int_ = {
            let mut __tmp_13: () = unimplemented!();
            std_basic_format_string_wchar_t__int_::new({ &mut __tmp_13 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_wchar_t__int_, *mut ())>(2)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__int_ = {
            let mut __tmp_14: () = unimplemented!();
            std_basic_format_string_wchar_t__int_::new({ &mut __tmp_14 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_wchar_t__int_, *mut ())>(2)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__int_ = {
            let mut __tmp_15: () = unimplemented!();
            std_basic_format_string_wchar_t__int_::new({ &mut __tmp_15 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_wchar_t__int_, *mut ())>(2)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__int_ = {
            let mut __tmp_16: () = unimplemented!();
            std_basic_format_string_wchar_t__int_::new({ &mut __tmp_16 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_wchar_t__int_, *mut ())>(2)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__int_ = {
            let mut __tmp_17: () = unimplemented!();
            std_basic_format_string_wchar_t__int_::new({ &mut __tmp_17 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_wchar_t__int_, *mut ())>(2)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__int_ = {
            let mut __tmp_18: () = unimplemented!();
            std_basic_format_string_wchar_t__int_::new({ &mut __tmp_18 })
        };
        let mut _args: () = 42_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_wchar_t__int_, *mut ())>(2)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_19: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_19 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(3)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_20: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_20 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(3)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_21: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_21 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut check_29))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(3)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(1)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(1)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(1)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(1)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(1)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(1)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = 42_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut check_exception_33))
                .spec::<fn((), (), *mut ())>(1)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
    }
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_34() });
    (unsafe { test_35() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_11);
    std::cell::LazyCell::force(&*&raw const check_29);
    std::cell::LazyCell::force(&*&raw const check_exception_33);
}
