extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub static mut test_format_27: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(
    || unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (),
             mut fmt: std_basic_format_string_char__const_char__ref_arr2arr_,
             args: *const [(); 2]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt : *mut std_basic_format_string_char__const_char__ref_arr2arr_ = & mut fmt ;
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
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_char__const_char__ref_arr3arr_,
             args: *const [(); 3]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt : *mut std_basic_format_string_char__const_char__ref_arr3arr_ = & mut fmt ;
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
                        test_require_28(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_char__const_char__ref_arr4arr_,
             args: *const [(); 4]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt : *mut std_basic_format_string_char__const_char__ref_arr4arr_ = & mut fmt ;
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
                        test_require_29(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_char__const_char__ref_arr5arr_,
             args: *const [(); 5]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt : *mut std_basic_format_string_char__const_char__ref_arr5arr_ = & mut fmt ;
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
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_char__const_char__ref_arr9arr_,
             args: *const [(); 9]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt : *mut std_basic_format_string_char__const_char__ref_arr9arr_ = & mut fmt ;
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
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_,
             args: *const [(); 2]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt : *mut std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ = & mut fmt ;
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
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_char__char__char_,
             args_2: *mut (),
             args_3: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut std_basic_format_string_char__char__char_ = &mut fmt;
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
                        test_require_33(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (), mut fmt: std_basic_format_string_char__char_, args: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut std_basic_format_string_char__char_ = &mut fmt;
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
                        test_require_34(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_char__std_string_view_,
             args: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut std_basic_format_string_char__std_string_view_ =
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
                        test_require_35(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_wchar_t__wchar_t__wchar_t_,
             args_2: *mut (),
             args_3: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut std_basic_format_string_wchar_t__wchar_t__wchar_t_ =
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
                        test_require_36(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_wchar_t__wchar_t_,
             args: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut std_basic_format_string_wchar_t__wchar_t_ = &mut fmt;
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
                        test_require_37(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_wchar_t__std_wstring_view_,
             args: *mut ()| {
                {
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
                        test_require_38(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_,
             args: *const [(); 3]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt : *mut std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ = & mut fmt ;
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
                        test_require_39(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_wchar_t__const_wchar_t__ref_arr4arr_,
             args: *const [(); 4]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt : *mut std_basic_format_string_wchar_t__const_wchar_t__ref_arr4arr_ = & mut fmt ;
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
                        test_require_40(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_wchar_t__const_wchar_t__ref_arr5arr_,
             args: *const [(); 5]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt : *mut std_basic_format_string_wchar_t__const_wchar_t__ref_arr5arr_ = & mut fmt ;
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
                        test_require_41(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50_(),
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                }
            }
        )
    },
);
pub static mut test_format_to_42: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(
    || unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (),
             mut fmt: std_basic_format_string_char__std_string_view_,
             args: *mut ()| {
                {
                    let mut out: () =
                        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                            { unimplemented!() },
                            { (' ' as ()) },
                            None,
                        );
                    let mut it: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut out: () =
                        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                            { unimplemented!() },
                            { (' ' as ()) },
                            None,
                        );
                    let mut it: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut out: () = unimplemented!();
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
                    let mut out: [(); 4096] = [unimplemented!(); 4096];
                    let mut it: *mut () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_wchar_t__std_wstring_view_,
             args: *mut ()| {
                {
                    let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
                    let mut it: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
                    let mut it: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut out: () = unimplemented!();
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
                    let mut out: [(); 4096] = [unimplemented!(); 4096];
                    let mut it: *mut () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
            }
        )
    },
);
pub static mut test_formatted_size_43: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (),
             mut fmt: std_basic_format_string_char__std_string_view_,
             args: *mut ()| {
                {
                    let mut size: () = unimplemented!();
                    unimplemented!();
                }
                {
                    let mut size: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_wchar_t__std_wstring_view_,
             args: *mut ()| {
                {
                    let mut size: () = unimplemented!();
                    unimplemented!();
                }
                {
                    let mut size: () = unimplemented!();
                    unimplemented!();
                }
            }
        )
    });
pub static mut test_format_to_n_44: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(
    || unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (),
             mut fmt: std_basic_format_string_char__std_string_view_,
             args: *mut ()| {
                {
                    let mut n: () = unimplemented!();
                    let mut out: () =
                        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                            { n },
                            { (' ' as ()) },
                            None,
                        );
                    let mut result: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut n: () = unimplemented!();
                    let mut out: () =
                        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                            { n },
                            { (' ' as ()) },
                            None,
                        );
                    let mut result: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut n: () = 0_();
                    let mut out: () = unimplemented!();
                    let mut result: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut n: () = (unimplemented!()).wrapping_div(2_());
                    let mut out: () =
                        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                            { n },
                            { (' ' as ()) },
                            None,
                        );
                    let mut result: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (),
             mut fmt: std_basic_format_string_wchar_t__std_wstring_view_,
             args: *mut ()| {
                {
                    let mut n: () = unimplemented!();
                    let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { n } , { ( ' ' as ()  ) } , None , )  ;
                    let mut result: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut n: () = unimplemented!();
                    let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { n } , { ( ' ' as ()  ) } , None , )  ;
                    let mut result: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut n: () = 0_();
                    let mut out: () = unimplemented!();
                    let mut result: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut n: () = (unimplemented!()).wrapping_div(2_());
                    let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { n } , { ( ' ' as ()  ) } , None , )  ;
                    let mut result: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
            }
        )
    },
);
pub unsafe fn test_char_45() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char__char_ = {
            let mut __tmp_0: () = unimplemented!();
            std_basic_format_string_char__char__char_::new({ &mut __tmp_0 })
        };
        let mut _args_2: () = ('\'' as ());
        let mut _args_3: () = ('\"' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char__char_, *mut (), *mut ())>(6)
            .call(_expected, _fmt, &mut _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_1: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_1 })
        };
        let mut _args: () = ('\t' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_2: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_2 })
        };
        let mut _args: () = ('\n' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_3: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_3 })
        };
        let mut _args: () = ('\r' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_4: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_4 })
        };
        let mut _args: () = ('\\' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_5: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_5 })
        };
        let mut _args: () = ('\'' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_6: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_6 })
        };
        let mut _args: () = ('\"' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_7: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_7 })
        };
        let mut _args: () = (' ' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_8: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_8 })
        };
        let mut _args: () = ('a' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_9: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_9 })
        };
        let mut _args: () = ('b' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_10: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_10 })
        };
        let mut _args: () = ('c' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_11: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_11 })
        };
        let mut _args: () = ('\0' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_12: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_12 })
        };
        let mut _args: () = ('\x1f' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__char_ = {
                let mut __tmp_13: () = unimplemented!();
                std_basic_format_string_char__char_::new({ &mut __tmp_13 })
            };
            let mut _args: () = (b'\x80' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(7)
                .call(_expected, _fmt, &mut _args)
        });
    }
    {}
    {}
}
pub unsafe fn test_char_46() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t__wchar_t_ = {
            let mut __tmp_14: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t__wchar_t_::new({ &mut __tmp_14 })
        };
        let mut _args_2: () = ('\'' as ());
        let mut _args_3: () = ('\"' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t__wchar_t_, *mut (), *mut ())>(9)
            .call(_expected, _fmt, &mut _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_15: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new_1({ &mut __tmp_15 })
        };
        let mut _args: () = ('\t' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_16: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new_1({ &mut __tmp_16 })
        };
        let mut _args: () = ('\n' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_17: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new_1({ &mut __tmp_17 })
        };
        let mut _args: () = ('\r' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_18: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new_1({ &mut __tmp_18 })
        };
        let mut _args: () = ('\\' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_19: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new_1({ &mut __tmp_19 })
        };
        let mut _args: () = ('\'' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_20: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new_1({ &mut __tmp_20 })
        };
        let mut _args: () = ('\"' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_21: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new_1({ &mut __tmp_21 })
        };
        let mut _args: () = (' ' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_22: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new_1({ &mut __tmp_22 })
        };
        let mut _args: () = ('a' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_23: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new_1({ &mut __tmp_23 })
        };
        let mut _args: () = ('b' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_24: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new_1({ &mut __tmp_24 })
        };
        let mut _args: () = ('c' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_25: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new_1({ &mut __tmp_25 })
        };
        let mut _args: () = ('\0' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_26: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new_1({ &mut __tmp_26 })
        };
        let mut _args: () = ('\x1f' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    {
        (unsafe {
            let mut _args: () = (160 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (12288 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (8232 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (8233 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (173 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (1536 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (65279 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (55296 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (57343 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (57344 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (63743 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (888 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (6004 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (65535 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (768 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (65056 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
    }
    {
        const _: () = assert!(
            ((::std::mem::size_of::<()>()) == (4_())),
            "sizeof(CharT) == 4"
        );;
        let x: () = 131071_();
        let y: u32 = 131071_u32;
        const _: () = assert!(true, "x == y");;
        (unsafe {
            let mut _args: () = (69821 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (917631 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (983040 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (1048573 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (1048576 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (1114109 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (65548 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (1048575 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (1114110 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (66045 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (917760 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (1114112 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
        (unsafe {
            let mut _args: () = (4294967295 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(10)
                .call(
                    unimplemented!(),
                    std_basic_format_string_wchar_t__wchar_t_::new_2({
                        &[123 as (), 58 as (), 63 as (), 125 as (), 0 as ()]
                    }),
                    &mut _args,
                )
        });
    }
}
pub unsafe fn test_string_47() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_27: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_27 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_28: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_28 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_29: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_29 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_30: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_30 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr2arr_ = {
                let mut __tmp_31: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr2arr_::new_1({ &mut __tmp_31 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr2arr_ ,*const [ () ; 2] ,)>(0) .call ( _expected , _fmt , & std::mem::transmute(*b"\xc3\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr3arr_ = {
                let mut __tmp_32: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr3arr_::new_1({ &mut __tmp_32 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr3arr_ ,*const [ () ; 3] ,)>(1) .call ( _expected , _fmt , & std::mem::transmute(*b"\xc3(\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr3arr_ = {
                let mut __tmp_33: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr3arr_::new_1({ &mut __tmp_33 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr3arr_ ,*const [ () ; 3] ,)>(1) .call ( _expected , _fmt , & std::mem::transmute(*b"\xc0\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr3arr_ = {
                let mut __tmp_34: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr3arr_::new_1({ &mut __tmp_34 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr3arr_ ,*const [ () ; 3] ,)>(1) .call ( _expected , _fmt , & std::mem::transmute(*b"\xc1\xbf\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr3arr_ = {
                let mut __tmp_35: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr3arr_::new_1({ &mut __tmp_35 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr3arr_ ,*const [ () ; 3] ,)>(1) .call ( _expected , _fmt , & std::mem::transmute(*b"\xc2\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
                let mut __tmp_36: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr4arr_::new_1({ &mut __tmp_36 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr4arr_ ,*const [ () ; 4] ,)>(2) .call ( _expected , _fmt , & std::mem::transmute(*b"\xe0\x80\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
                let mut __tmp_37: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr4arr_::new_1({ &mut __tmp_37 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr4arr_ ,*const [ () ; 4] ,)>(2) .call ( _expected , _fmt , & std::mem::transmute(*b"\xe0\x81\xbf\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
                let mut __tmp_38: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr4arr_::new_1({ &mut __tmp_38 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr4arr_ ,*const [ () ; 4] ,)>(2) .call ( _expected , _fmt , & std::mem::transmute(*b"\xe0\x82\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
                let mut __tmp_39: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr4arr_::new_1({ &mut __tmp_39 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr4arr_ ,*const [ () ; 4] ,)>(2) .call ( _expected , _fmt , & std::mem::transmute(*b"\xe0\x9f\xbf\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
                let mut __tmp_40: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr4arr_::new_1({ &mut __tmp_40 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr4arr_ ,*const [ () ; 4] ,)>(2) .call ( _expected , _fmt , & std::mem::transmute(*b"\xe0\xa0\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
                let mut __tmp_41: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr4arr_::new_1({ &mut __tmp_41 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr4arr_ ,*const [ () ; 4] ,)>(2) .call ( _expected , _fmt , & std::mem::transmute(*b"\xed\x9f\xbf\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
                let mut __tmp_42: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr4arr_::new_1({ &mut __tmp_42 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr4arr_ ,*const [ () ; 4] ,)>(2) .call ( _expected , _fmt , & std::mem::transmute(*b"\xed\xa0\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
                let mut __tmp_43: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr4arr_::new_1({ &mut __tmp_43 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr4arr_ ,*const [ () ; 4] ,)>(2) .call ( _expected , _fmt , & std::mem::transmute(*b"\xed\xaf\xbf\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
                let mut __tmp_44: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr4arr_::new_1({ &mut __tmp_44 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr4arr_ ,*const [ () ; 4] ,)>(2) .call ( _expected , _fmt , & std::mem::transmute(*b"\xed\xbf\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
                let mut __tmp_45: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr4arr_::new_1({ &mut __tmp_45 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr4arr_ ,*const [ () ; 4] ,)>(2) .call ( _expected , _fmt , & std::mem::transmute(*b"\xed\xbf\xbf\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr4arr_ = {
                let mut __tmp_46: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr4arr_::new_1({ &mut __tmp_46 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr4arr_ ,*const [ () ; 4] ,)>(2) .call ( _expected , _fmt , & std::mem::transmute(*b"\xee\x80\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr5arr_ = {
                let mut __tmp_47: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr5arr_::new_1({ &mut __tmp_47 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr5arr_ ,*const [ () ; 5] ,)>(3) .call ( _expected , _fmt , & std::mem::transmute(*b"\xf0\x80\x80\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr5arr_ = {
                let mut __tmp_48: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr5arr_::new_1({ &mut __tmp_48 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr5arr_ ,*const [ () ; 5] ,)>(3) .call ( _expected , _fmt , & std::mem::transmute(*b"\xf0\x80\x81\xbf\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr5arr_ = {
                let mut __tmp_49: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr5arr_::new_1({ &mut __tmp_49 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr5arr_ ,*const [ () ; 5] ,)>(3) .call ( _expected , _fmt , & std::mem::transmute(*b"\xf0\x80\x82\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr5arr_ = {
                let mut __tmp_50: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr5arr_::new_1({ &mut __tmp_50 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr5arr_ ,*const [ () ; 5] ,)>(3) .call ( _expected , _fmt , & std::mem::transmute(*b"\xf0\x80\x9f\xbf\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr5arr_ = {
                let mut __tmp_51: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr5arr_::new_1({ &mut __tmp_51 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr5arr_ ,*const [ () ; 5] ,)>(3) .call ( _expected , _fmt , & std::mem::transmute(*b"\xf0\x80\xa0\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr5arr_ = {
                let mut __tmp_52: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr5arr_::new_1({ &mut __tmp_52 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr5arr_ ,*const [ () ; 5] ,)>(3) .call ( _expected , _fmt , & std::mem::transmute(*b"\xf0\x8f\xbf\xbf\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr5arr_ = {
                let mut __tmp_53: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr5arr_::new_1({ &mut __tmp_53 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr5arr_ ,*const [ () ; 5] ,)>(3) .call ( _expected , _fmt , & std::mem::transmute(*b"\xf0\x90\x80\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr5arr_ = {
                let mut __tmp_54: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr5arr_::new_1({ &mut __tmp_54 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr5arr_ ,*const [ () ; 5] ,)>(3) .call ( _expected , _fmt , & std::mem::transmute(*b"\xf4\x8f\xbf\xbf\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr5arr_ = {
                let mut __tmp_55: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr5arr_::new_1({ &mut __tmp_55 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr5arr_ ,*const [ () ; 5] ,)>(3) .call ( _expected , _fmt , & std::mem::transmute(*b"\xf4\x90\x80\x80\0") , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr5arr_ = {
                let mut __tmp_56: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr5arr_::new_1({ &mut __tmp_56 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr5arr_ ,*const [ () ; 5] ,)>(3) .call ( _expected , _fmt , & std::mem::transmute(*b"\xf4\xbf\xbf\xbf\0") , )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_57: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_57 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_58: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_58 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_59: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_59 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_60: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_60 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char__ref_arr2arr_ = {
                let mut __tmp_61: () = unimplemented!();
                std_basic_format_string_char__const_char__ref_arr2arr_::new_1({ &mut __tmp_61 })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_char__const_char__ref_arr2arr_ ,*const [ () ; 2] ,)>(0) .call ( _expected , _fmt , & std::mem::transmute(*b"\x80\0") , )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_62: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_62 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_63: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_63 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_64: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_64 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    {}
}
pub unsafe fn test_string_48() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_65: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_65 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_66: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_66 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_67: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_67 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_68: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_68 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ = {
                let mut __tmp_69: () = unimplemented!();
                std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_::new_1({
                    &mut __tmp_69
                })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ ,*const [ () ; 2] ,)>(5) .call ( _expected , _fmt , & [195 as () , 0 as () , ] , )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ = {
                let mut __tmp_70: () = unimplemented!();
                std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_::new_1({
                    &mut __tmp_70
                })
            };
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ ,*const [ () ; 3] ,)>(12) .call ( _expected , _fmt , & [195 as () , 40 as () , 0 as () , ] , )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_71: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_71 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_72: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_72 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_73: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_73 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_74: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_74 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_75: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_75 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_76: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_76 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_77: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_77 })
        };
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ ,*const [ () ; 3] ,)>(12) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ :: new_2 ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [160 as () , 12288 as () , 0 as () , ] , )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ ,*const [ () ; 2] ,)>(5) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ :: new_2 ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [8232 as () , 0 as () , ] , )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ ,*const [ () ; 2] ,)>(5) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ :: new_2 ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [8233 as () , 0 as () , ] , )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr4arr_ ,*const [ () ; 4] ,)>(13) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr4arr_ :: new ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [173 as () , 1536 as () , 65279 as () , 0 as () , ] , )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ ,*const [ () ; 2] ,)>(5) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ :: new_2 ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [55296 as () , 0 as () , ] , )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ ,*const [ () ; 3] ,)>(12) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ :: new_2 ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [57344 as () , 63743 as () , 0 as () , ] , )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr4arr_ ,*const [ () ; 4] ,)>(13) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr4arr_ :: new ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [888 as () , 6004 as () , 65535 as () , 0 as () , ] , )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ ,*const [ () ; 3] ,)>(12) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ :: new_2 ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [768 as () , 65056 as () , 0 as () , ] , )
        });
    }
    {
        const _: () = assert!(
            ((::std::mem::size_of::<()>()) == (4_())),
            "sizeof(CharT) == 4"
        );;
        let x: () = 131071_();
        let y: u32 = 131071_u32;
        const _: () = assert!(true, "x == y");;
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ ,*const [ () ; 3] ,)>(12) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ :: new_2 ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [69821 as () , 917631 as () , 0 as () , ] , )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr5arr_ ,*const [ () ; 5] ,)>(14) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr5arr_ :: new ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [983040 as () , 1048573 as () , 1048576 as () , 1114109 as () , 0 as () , ] , )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr4arr_ ,*const [ () ; 4] ,)>(13) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr4arr_ :: new ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [65548 as () , 1048575 as () , 1114110 as () , 0 as () , ] , )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ ,*const [ () ; 3] ,)>(12) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ :: new_2 ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [66045 as () , 917760 as () , 0 as () , ] , )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ ,*const [ () ; 3] ,)>(12) .call ( unimplemented!() , std_basic_format_string_wchar_t__const_wchar_t__ref_arr3arr_ :: new_2 ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [1114112 as () , 4294967295 as () , 0 as () , ] , )
        });
    }
}
pub unsafe fn test_format_functions_49(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_78: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_78 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_79: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_79 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_80: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_80 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_81: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_81 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_82: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_82 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_83: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_83 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_84: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_84 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_85: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_85 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_86: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_86 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_87: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_87 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_88: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_88 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_89: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_89 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_90: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_90 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_91: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_91 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_92: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_92 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_93: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_93 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_94: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_94 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_95: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_95 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_96: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_96 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_97: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_97 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_98: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_98 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_99: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_99 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_100: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_100 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_101: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_101 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_102: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_102 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_103: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_103 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_104: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_104 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_105: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_105 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_106: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_106 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_107: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_107 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_108: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_108 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_109: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_109 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_50(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_110: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_110 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_111: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_111 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_112: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_112 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_113: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_113 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_114: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_114 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_115: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_115 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_116: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_116 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_117: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_117 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_118: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_118 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_119: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_119 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_120: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_120 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_121: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_121 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_122: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_122 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_123: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_123 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_124: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_124 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_125: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_125 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_126: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_126 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_127: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_127 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_128: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_128 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_129: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_129 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_130: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_130 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_131: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_131 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_132: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_132 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_133: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_133 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_134: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_134 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_135: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_135 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_136: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_136 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_137: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_137 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_138: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_138 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_139: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_139 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_140: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_140 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_141: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_141 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_51(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_142: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_142 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_143: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_143 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_144: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_144 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_145: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_145 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_146: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_146 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_147: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_147 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_148: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_148 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_149: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_149 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_150: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_150 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_151: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_151 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_152: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_152 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_153: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_153 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_154: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_154 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_155: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_155 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_156: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_156 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_157: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_157 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_158: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_158 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_159: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_159 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_160: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_160 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_161: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_161 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_162: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_162 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_163: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_163 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_164: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_164 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_165: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_165 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_166: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_166 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_167: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_167 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_168: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_168 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_169: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_169 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_170: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_170 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_171: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_171 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_172: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_172 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_173: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_173 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_52(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_174: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_174 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_175: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_175 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_176: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_176 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_177: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_177 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_178: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_178 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_179: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_179 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_180: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_180 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_181: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_181 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_182: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_182 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_183: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_183 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_184: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_184 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_185: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_185 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_186: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_186 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_187: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_187 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_188: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_188 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_189: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_189 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_190: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_190 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_191: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_191 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_192: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_192 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_193: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_193 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_194: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_194 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_195: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_195 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_196: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_196 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_197: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_197 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_198: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_198 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_199: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_199 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_200: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_200 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_201: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_201 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_202: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_202 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_203: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_203 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_204: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_204 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_string_view_ = {
            let mut __tmp_205: () = unimplemented!();
            std_basic_format_string_char__std_string_view_::new({ &mut __tmp_205 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_string_view_, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_53(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_206: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_206 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_207: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_207 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_208: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_208 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_209: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_209 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_210: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_210 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_211: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_211 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_212: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_212 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_213: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_213 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_214: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_214 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_215: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_215 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_216: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_216 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_217: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_217 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_218: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_218 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_219: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_219 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_220: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_220 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_221: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_221 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_222: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_222 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_223: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_223 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_224: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_224 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_225: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_225 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_226: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_226 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_227: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_227 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_228: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_228 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_229: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_229 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_230: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_230 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_231: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_231 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_232: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_232 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_233: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_233 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_234: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_234 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_235: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_235 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_236: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_236 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_237: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_237 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_54(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_238: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_238 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_239: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_239 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_240: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_240 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_241: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_241 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_242: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_242 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_243: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_243 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_244: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_244 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_245: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_245 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_246: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_246 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_247: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_247 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_248: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_248 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_249: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_249 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_250: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_250 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_251: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_251 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_252: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_252 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_253: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_253 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_254: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_254 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_255: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_255 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_256: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_256 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_257: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_257 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_258: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_258 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_259: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_259 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_260: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_260 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_261: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_261 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_262: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_262 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_263: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_263 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_264: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_264 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_265: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_265 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_266: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_266 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_267: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_267 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_268: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_268 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_269: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_269 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_55(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_270: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_270 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_271: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_271 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_272: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_272 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_273: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_273 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_274: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_274 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_275: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_275 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_276: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_276 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_277: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_277 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_278: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_278 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_279: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_279 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_280: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_280 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_281: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_281 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_282: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_282 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_283: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_283 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_284: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_284 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_285: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_285 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_286: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_286 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_287: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_287 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_288: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_288 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_289: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_289 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_290: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_290 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_291: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_291 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_292: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_292 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_293: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_293 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_294: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_294 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_295: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_295 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_296: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_296 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_297: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_297 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_298: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_298 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_299: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_299 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_300: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_300 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_301: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_301 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_56(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_302: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_302 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_303: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_303 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_304: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_304 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_305: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_305 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_306: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_306 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_307: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_307 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_308: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_308 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_309: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_309 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_310: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_310 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_311: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_311 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_312: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_312 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_313: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_313 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_314: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_314 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_315: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_315 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_316: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_316 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_317: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_317 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_318: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_318 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_319: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_319 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_320: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_320 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_321: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_321 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_322: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_322 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_323: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_323 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_324: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_324 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_325: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_325 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_326: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_326 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_327: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_327 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_328: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_328 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_329: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_329 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_330: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_330 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_331: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_331 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_332: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_332 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ = {
            let mut __tmp_333: () = unimplemented!();
            std_basic_format_string_wchar_t__std_wstring_view_::new({ &mut __tmp_333 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_57() {
    (unsafe { test_char_45() });
    (unsafe { test_string_47() });
    (unsafe {
        test_format_functions_49(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_50(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_to_42)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_51(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_formatted_size_43)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_52(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_to_n_44)).copy_from(),
        )
    });
}
pub unsafe fn test_58() {
    (unsafe { test_char_46() });
    (unsafe { test_string_48() });
    (unsafe {
        test_format_functions_53(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_54(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_to_42)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_55(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_formatted_size_43)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_56(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_to_n_44)).copy_from(),
        )
    });
}
pub unsafe fn test_ill_formed_utf8_59() {
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr2arr_, *const [(); 2])>(
                0,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\x{df}\"".as_ptr(), 8_()) }),
                std_basic_format_string_char__const_char__ref_arr2arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xdf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr2arr_, *const [(); 2])>(
                0,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\x{ef}\"".as_ptr(), 8_()) }),
                std_basic_format_string_char__const_char__ref_arr2arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xef\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr3arr_, *const [(); 3])>(
                1,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\x{ef}\\x{bf}\"".as_ptr(), 14_()) }),
                std_basic_format_string_char__const_char__ref_arr3arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xef\xbf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr2arr_, *const [(); 2])>(
                0,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\x{f7}\"".as_ptr(), 8_()) }),
                std_basic_format_string_char__const_char__ref_arr2arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xf7\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr3arr_, *const [(); 3])>(
                1,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\x{f7}\\x{bf}\"".as_ptr(), 14_()) }),
                std_basic_format_string_char__const_char__ref_arr3arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xf7\xbf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr4arr_, *const [(); 4])>(
                2,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\x{f7}\\x{bf}\\x{bf}\"".as_ptr(), 20_()) }),
                std_basic_format_string_char__const_char__ref_arr4arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xf7\xbf\xbf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr3arr_, *const [(); 3])>(
                1,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\x{df}a\"".as_ptr(), 9_()) }),
                std_basic_format_string_char__const_char__ref_arr3arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xdfa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr3arr_, *const [(); 3])>(
                1,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\x{ef}a\"".as_ptr(), 9_()) }),
                std_basic_format_string_char__const_char__ref_arr3arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xefa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr4arr_, *const [(); 4])>(
                2,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\x{ef}\\x{bf}a\"".as_ptr(), 15_()) }),
                std_basic_format_string_char__const_char__ref_arr4arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xef\xbfa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr3arr_, *const [(); 3])>(
                1,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\x{f7}a\"".as_ptr(), 9_()) }),
                std_basic_format_string_char__const_char__ref_arr3arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xf7a\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr4arr_, *const [(); 4])>(
                2,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\x{f7}\\x{bf}a\"".as_ptr(), 15_()) }),
                std_basic_format_string_char__const_char__ref_arr4arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xf7\xbfa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr5arr_, *const [(); 5])>(
                3,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\x{f7}\\x{bf}\\x{bf}a\"".as_ptr(), 21_()) }),
                std_basic_format_string_char__const_char__ref_arr5arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xf7\xbf\xbfa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr9arr_, *const [(); 9])>(
                4,
            )
            .call(
                (unsafe {
                    operator_literal_sv_60(
                        c"\"a\\x{f1}\\x{80}\\x{80}\\x{e1}\\x{80}\\x{c2}b\"".as_ptr(),
                        40_(),
                    )
                }),
                std_basic_format_string_char__const_char__ref_arr9arr_::new({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"a\xf1\x80\x80\xe1\x80\xc2b\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr5arr_, *const [(); 5])>(
                3,
            )
            .call(
                (unsafe { operator_literal_sv_60(c"\"\\u{10ffff}\"".as_ptr(), 12_()) }),
                std_basic_format_string_char__const_char__ref_arr5arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xf4\x8f\xbf\xbf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr5arr_, *const [(); 5])>(
                3,
            )
            .call(
                (unsafe {
                    operator_literal_sv_60(c"\"\\x{f4}\\x{90}\\x{80}\\x{80}\"".as_ptr(), 26_())
                }),
                std_basic_format_string_char__const_char__ref_arr5arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xf4\x90\x80\x80\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr5arr_, *const [(); 5])>(
                3,
            )
            .call(
                (unsafe {
                    operator_literal_sv_60(c"\"\\x{f5}\\x{b1}\\x{b2}\\x{b3}\"".as_ptr(), 26_())
                }),
                std_basic_format_string_char__const_char__ref_arr5arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xf5\xb1\xb2\xb3\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))
            .spec::<fn((), std_basic_format_string_char__const_char__ref_arr5arr_, *const [(); 5])>(
                3,
            )
            .call(
                (unsafe {
                    operator_literal_sv_60(c"\"\\x{f7}\\x{bf}\\x{bf}\\x{bf}\"".as_ptr(), 26_())
                }),
                std_basic_format_string_char__const_char__ref_arr5arr_::new_2({
                    &std::mem::transmute(*b"{:?}\0")
                }),
                &std::mem::transmute(*b"\xf7\xbf\xbf\xbf\0"),
            )
    });
}
pub unsafe fn test_ill_formed_utf32_61() {
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ ,*const [ () ; 2] ,)>(5) .call ( ( unsafe { operator_literal_sv_62 ( [34 as () , 92 as () , 117 as () , 123 as () , 49 as () , 48 as () , 102 as () , 102 as () , 102 as () , 102 as () , 125 as () , 34 as () , 0 as () , ] .as_ptr() , 12_() , ) } )  , std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ :: new_2 ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [1114111 as () , 0 as () , ] , )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ ,*const [ () ; 2] ,)>(5) .call ( ( unsafe { operator_literal_sv_62 ( [34 as () , 92 as () , 120 as () , 123 as () , 49 as () , 49 as () , 48 as () , 48 as () , 48 as () , 48 as () , 125 as () , 34 as () , 0 as () , ] .as_ptr() , 12_() , ) } )  , std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ :: new_2 ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [1114112 as () , 0 as () , ] , )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_27))  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ ,*const [ () ; 2] ,)>(5) .call ( ( unsafe { operator_literal_sv_62 ( [34 as () , 92 as () , 120 as () , 123 as () , 102 as () , 102 as () , 102 as () , 102 as () , 102 as () , 102 as () , 102 as () , 102 as () , 125 as () , 34 as () , 0 as () , ] .as_ptr() , 14_() , ) } )  , std_basic_format_string_wchar_t__const_wchar_t__ref_arr2arr_ :: new_2 ( { & [123 as () , 58 as () , 63 as () , 125 as () , 0 as () , ] } , )  , & [4294967295 as () , 0 as () , ] , )
    });
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
    (unsafe { test_57() });
    (unsafe { test_58() });
    (unsafe { test_ill_formed_utf8_59() });
    (unsafe { test_ill_formed_utf32_61() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_11);
    std::cell::LazyCell::force(&*&raw const test_format_27);
    std::cell::LazyCell::force(&*&raw const test_format_to_42);
    std::cell::LazyCell::force(&*&raw const test_formatted_size_43);
    std::cell::LazyCell::force(&*&raw const test_format_to_n_44);
}
