extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_format_context_create_0(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_1((*unimplemented!()), args.clone(), None) });
}
pub unsafe fn test_format_context_create_2(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_3((*unimplemented!()), args.clone(), None) });
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
pub unsafe fn test_eprintf_4(
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
pub unsafe fn test_eprintf_5(
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
pub unsafe fn test_eprintf_6(mut fmt: *const (), args: *const *const ()) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args),
        )
    });
}
pub unsafe fn test_log_7(
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
        test_eprintf_4(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
pub unsafe fn test_log_8(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_5(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_require_9(
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
        test_log_8(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_is_high_surrogate_10(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (56319_()));
}
pub unsafe fn test_is_low_surrogate_11(mut value: ()) -> () {
    return ((value) >= (56320_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_surrogate_12(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_code_point_13(mut value: ()) -> () {
    return ((value) <= (1114111_()));
}
pub unsafe fn test_is_scalar_value_14(mut value: ()) -> () {
    return (unsafe { test_is_code_point_13(value) })
        && (!(unsafe { test_is_surrogate_12(value) }));
}
pub static mut test_replacement_character_15: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (65533 as ()) });
pub unsafe fn test_encode_16(out_it: *mut (), mut value: ()) {
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
pub unsafe fn test_encode_17(out_it: *mut (), mut value: ()) {
    if (((value) & (4294901760_())) == (0_())) {
        (unsafe {
            let _out_it: *mut () = out_it;
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
pub unsafe fn test_transcode_18(mut first: (), mut last: (), mut out_it: ()) -> () {
    'loop_: while !(unimplemented!()) {
        let mut value: () = (*unimplemented!());
        if (unsafe { test_is_code_point_13(value) }) {
            (unsafe { test_encode_17(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_16(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_15)),
                )
            });
        }
    }
    return out_it;
}
pub unsafe fn operator_shl_19(os: *mut (), loc: *const ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_20(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_21(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_22(stream: *mut (), value: *const [(); 18]) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_23(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if !(unimplemented!()) {
        {}
        (unsafe { test_transcode_18(unimplemented!(), unimplemented!(), unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_message_24(
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
                test_concat_22(_stream, _value)
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
                        test_concat_22(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_21(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_22(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_21(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
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
                test_concat_22(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_23(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_22(_stream, _value)
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
                                test_concat_22(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_23(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
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
pub unsafe fn test_26(mut expected: (), mut fmt: (), mut arg: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    (unsafe { (formatter as std___formatter_char_char_).parse(&mut parse_ctx) });
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () =
        (unsafe { test_format_context_create_0(out.clone(), unimplemented!()) });
    (unsafe { (formatter as std___formatter_char_char_).format(arg, &mut format_ctx) });
    (unsafe {
        let _condition: () = unimplemented!();
        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
            {
                let fmt: *mut () = &mut fmt;
                let expected: *mut () = &mut expected;
                let result: *mut () = &mut result;
            },
            || {
                (unsafe {
                    let mut _args: *const () = (unimplemented!() as *const ());
                    test_eprintf_6(c"%s".as_ptr(), &mut _args)
                });
            }
        );
        test_require_9(
            _condition,
            c"result == expected".as_ptr(),
            c"formatter.char.funsigned-char.pass.cpp".as_ptr(),
            57_(),
            &mut _arg,
        )
    });
}
pub unsafe fn test_27(mut expected: (), mut fmt: (), mut arg: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    (unsafe { (formatter as std___formatter_char_wchar_t_).parse(&mut parse_ctx) });
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () =
        (unsafe { test_format_context_create_2(out.clone(), unimplemented!()) });
    (unsafe { (formatter as std___formatter_char_wchar_t_).format(arg, &mut format_ctx) });
    (unsafe {
        let _condition: () = unimplemented!();
        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
            {
                let fmt: *mut () = &mut fmt;
                let expected: *mut () = &mut expected;
                let result: *mut () = &mut result;
            },
            || {
                (unsafe {
                    let mut _args: *const () = (unimplemented!() as *const ());
                    test_eprintf_6(c"%s".as_ptr(), &mut _args)
                });
            }
        );
        test_require_28(
            _condition,
            c"result == expected".as_ptr(),
            c"formatter.char.funsigned-char.pass.cpp".as_ptr(),
            57_(),
            &mut _arg,
        )
    });
}
pub unsafe fn test_29() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, ('\0' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, ('a' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, (b'\x80' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, (b'\xff' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, ('\0' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, ('a' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, (b'\x80' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, (b'\xff' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, ('\0' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, ('a' as ()))
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            test_26(_expected, _fmt, (b'\x80' as ()))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            test_26(_expected, _fmt, (b'\xff' as ()))
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 127_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 127_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 127_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 127_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 127_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, -1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_26(_expected, _fmt, 127_())
    });
}
pub unsafe fn test_30() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, ('\0' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, ('a' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, (b'\x80' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, (b'\xff' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, ('\0' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, ('a' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, (b'\x80' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, (b'\xff' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, ('\0' as ()))
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, ('a' as ()))
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            test_27(_expected, _fmt, (b'\x80' as ()))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            test_27(_expected, _fmt, (b'\xff' as ()))
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, -1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 127_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, -1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 127_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, -1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 127_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, -1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 127_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, -1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 127_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, -1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 1_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        test_27(_expected, _fmt, 127_())
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
    (unsafe { test_29() });
    (unsafe { test_30() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_15);
}
