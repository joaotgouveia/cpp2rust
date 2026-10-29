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
    unimplemented!();
}
pub unsafe fn test_eprintf_1(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const *const (),
    args_3: *const (),
) {
    unimplemented!();
}
pub unsafe fn test_eprintf_2(mut fmt: *const (), args: *const *const ()) {
    unimplemented!();
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
pub unsafe fn test_log_5(
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
pub unsafe fn test_log_6(
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
pub unsafe fn test_log_7(
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
pub unsafe fn test_log_8(
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
pub unsafe fn test_log_9(
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
pub unsafe fn test_log_10(
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
pub unsafe fn test_log_11(
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
pub unsafe fn test_log_12(
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
pub unsafe fn test_log_13(
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
pub unsafe fn test_log_14(
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
pub unsafe fn test_log_15(
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
pub unsafe fn test_log_16(
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
pub unsafe fn test_log_17(
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
pub unsafe fn test_log_18(
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
pub unsafe fn test_require_19(
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
pub unsafe fn test_require_20(
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
        test_log_5(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_21(
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
        test_log_6(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_22(
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
        test_log_7(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_23(
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
pub unsafe fn test_require_24(
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
        test_log_9(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_25(
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
        test_log_10(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_26(
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
        test_log_11(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_27(
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
        test_log_12(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_28(
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
        test_log_13(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_29(
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
        test_log_14(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_30(
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
        test_log_15(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_31(
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
        test_log_16(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_32(
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
        test_log_17(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_require_33(
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
        test_log_18(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_is_high_surrogate_34(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (56319_()));
}
pub unsafe fn test_is_low_surrogate_35(mut value: ()) -> () {
    return ((value) >= (56320_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_surrogate_36(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_code_point_37(mut value: ()) -> () {
    return ((value) <= (1114111_()));
}
pub unsafe fn test_is_scalar_value_38(mut value: ()) -> () {
    return (unsafe { test_is_code_point_37(value) })
        && (!(unsafe { test_is_surrogate_36(value) }));
}
pub static mut test_replacement_character_39: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (65533 as ()) });
pub unsafe fn test_encode_40(out_it: *mut (), mut value: ()) {
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
pub unsafe fn test_encode_41(out_it: *mut (), mut value: ()) {
    if (((value) & (4294901760_())) == (0_())) {
        (unsafe {
            let _out_it: *mut () = out_it;
            let _value: () = value;
            test_encode_40(_out_it, _value)
        });
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_transcode_42(mut first: (), mut last: (), mut out_it: ()) -> () {
    'loop_: while !(unimplemented!()) {
        let mut value: () = (*unimplemented!());
        if (unsafe { test_is_code_point_37(value) }) {
            (unsafe { test_encode_41(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_40(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_39)),
                )
            });
        }
    }
    return unimplemented!();
}
pub unsafe fn test_transcode_43(mut first: *const (), mut last: *const (), mut out_it: ()) -> () {
    'loop_: while ((first) != (last)) {
        let mut value: () = (*first.postfix_inc());
        if (unsafe { test_is_code_point_37(value) }) {
            (unsafe { test_encode_41(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_40(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_39)),
                )
            });
        }
    }
    return unimplemented!();
}
pub unsafe fn operator_shl_44(os: *mut (), loc: *const ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_45(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_46(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_47(stream: *mut (), value: *const [(); 18]) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_48(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_49(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_50(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if !(unimplemented!()) {
        {}
        (unsafe { test_transcode_42(b.clone(), e.clone(), unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_51(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ((b) != (e)) {
        {}
        (unsafe { test_transcode_43(b, e, unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_52(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ((b) != (e)) {
        {}
        (unsafe { test_transcode_43(b, e, unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_message_53(
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
                test_concat_47(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_49(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_47(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_48(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_47(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_46(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
                                    test_concat_45(_stream, _value)
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
pub unsafe fn test_concat_message_54(
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
                test_concat_47(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_52(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_47(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_51(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_47(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_50(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
                                    test_concat_45(_stream, _value)
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
pub static mut test_format_55: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (), mut fmt: (), args: *const [(); 2]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
                        test_require_19(
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
            |mut expected: (), mut fmt: (), args: *const [(); 3]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
                        test_require_20(
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
            |mut expected: (), mut fmt: (), args: *const [(); 4]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
                        test_require_21(
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
            |mut expected: (), mut fmt: (), args: *const [(); 5]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
                        test_require_22(
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
            |mut expected: (), mut fmt: (), args: *const [(); 9]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
                        test_require_23(
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
            |mut expected: (), mut fmt: (), args: *const [(); 2]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
                        test_require_24(
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
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
                        test_require_25(
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
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
                        test_require_26(
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
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
                        test_require_27(
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
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
            |mut expected: (), mut fmt: (), args: *const [(); 3]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
            |mut expected: (), mut fmt: (), args: *const [(); 4]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
            |mut expected: (), mut fmt: (), args: *const [(); 5]| {
                {
                    let mut out: () = unimplemented!();
                    (unsafe {
                        let _condition: () = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
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
            }
        )
    });
pub static mut test_format_to_56: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    let mut it: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut out: () = unimplemented!();
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
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut out: () = unimplemented!();
                    let mut it: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut out: () = unimplemented!();
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
    });
pub static mut test_formatted_size_57: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut size: () = unimplemented!();
                    unimplemented!();
                }
                {
                    let mut size: () = unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
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
pub static mut test_format_to_n_58: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut n: () = unimplemented!();
                    let mut out: () = unimplemented!();
                    let mut result: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut n: () = unimplemented!();
                    let mut out: () = unimplemented!();
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
                    let mut out: () = unimplemented!();
                    let mut result: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut n: () = unimplemented!();
                    let mut out: () = unimplemented!();
                    let mut result: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                {
                    let mut n: () = unimplemented!();
                    let mut out: () = unimplemented!();
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
                    let mut out: () = unimplemented!();
                    let mut result: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
            }
        )
    });
pub unsafe fn test_char_59() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_2: () = ('\'' as ());
        let mut _args_3: () = ('\"' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut (), *mut ())>(6)
            .call(_expected, _fmt, &mut _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\t' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\n' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\r' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\\' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\'' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\"' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = (' ' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('a' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('b' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('c' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\0' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\x1f' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(7)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            let mut _args: () = (b'\x80' as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(7)
                .call(_expected, _fmt, &mut _args)
        });
    }
    {}
    {}
}
pub unsafe fn test_char_60() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_2: () = ('\'' as ());
        let mut _args_3: () = ('\"' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut (), *mut ())>(9)
            .call(_expected, _fmt, &mut _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\t' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\n' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\r' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\\' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\'' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\"' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = (' ' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('a' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('b' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('c' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\0' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = ('\x1f' as ());
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    {
        (unsafe {
            let mut _args: () = (160 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (12288 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (8232 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (8233 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (173 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (1536 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (65279 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (55296 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (57343 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (57344 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (63743 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (888 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (6004 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (65535 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (768 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (65056 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
    }
    {
        const _: () = assert!(
            ((::std::mem::size_of::<()>()) == (4_())),
            "sizeof(CharT) == 4"
        );;
        let x: () = 131071_();
        let y: () = 131071_();
        const _: () = assert!(true, "x == y");;
        (unsafe {
            let mut _args: () = (69821 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (917631 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (983040 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (1048573 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (1048576 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (1114109 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (65548 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (1048575 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (1114110 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (66045 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (917760 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (1114112 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: () = (4294967295 as ());
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
    }
}
pub unsafe fn test_string_61() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 2])>(0)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xc3\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 3])>(1)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xc3(\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 3])>(1)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xc0\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 3])>(1)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xc1\xbf\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 3])>(1)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xc2\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xe0\x80\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xe0\x81\xbf\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xe0\x82\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xe0\x9f\xbf\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xe0\xa0\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xed\x9f\xbf\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xed\xa0\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xed\xaf\xbf\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xed\xbf\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xed\xbf\xbf\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xee\x80\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x80\x80\x80\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x80\x81\xbf\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x80\x82\x80\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x80\x9f\xbf\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x80\xa0\x80\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x8f\xbf\xbf\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x90\x80\x80\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf4\x8f\xbf\xbf\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf4\x90\x80\x80\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf4\xbf\xbf\xbf\0"),
                )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 2])>(0)
                .call(_expected, _fmt, &std::mem::transmute(*b"\x80\0"))
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    {}
}
pub unsafe fn test_string_62() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 2])>(5)
                .call(_expected, _fmt, &[195 as (), 0 as ()])
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 3])>(12)
                .call(_expected, _fmt, &[195 as (), 40 as (), 0 as ()])
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 3])>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[160 as (), 12288 as (), 0 as ()],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 2])>(5)
                .call(unimplemented!(), unimplemented!(), &[8232 as (), 0 as ()])
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 2])>(5)
                .call(unimplemented!(), unimplemented!(), &[8233 as (), 0 as ()])
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(13)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[173 as (), 1536 as (), 65279 as (), 0 as ()],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 2])>(5)
                .call(unimplemented!(), unimplemented!(), &[55296 as (), 0 as ()])
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 3])>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[57344 as (), 63743 as (), 0 as ()],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(13)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[888 as (), 6004 as (), 65535 as (), 0 as ()],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 3])>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[768 as (), 65056 as (), 0 as ()],
                )
        });
    }
    {
        const _: () = assert!(
            ((::std::mem::size_of::<()>()) == (4_())),
            "sizeof(CharT) == 4"
        );;
        let x: () = 131071_();
        let y: () = 131071_();
        const _: () = assert!(true, "x == y");;
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 3])>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[69821 as (), 917631 as (), 0 as ()],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 5])>(14)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[
                        983040 as (),
                        1048573 as (),
                        1048576 as (),
                        1114109 as (),
                        0 as (),
                    ],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 4])>(13)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[65548 as (), 1048575 as (), 1114110 as (), 0 as ()],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 3])>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[66045 as (), 917760 as (), 0 as ()],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [(); 3])>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[1114112 as (), 4294967295 as (), 0 as ()],
                )
        });
    }
}
pub unsafe fn test_format_functions_63(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_64(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_65(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_66(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_67(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_68(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_69(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_70(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_71() {
    (unsafe { test_char_59() });
    (unsafe { test_string_61() });
    (unsafe {
        test_format_functions_63(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_64(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_to_56)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_65(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_formatted_size_57)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_66(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_to_n_58)).copy_from(),
        )
    });
}
pub unsafe fn test_72() {
    (unsafe { test_char_60() });
    (unsafe { test_string_62() });
    (unsafe {
        test_format_functions_67(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_68(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_to_56)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_69(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_formatted_size_57)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_70(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_to_n_58)).copy_from(),
        )
    });
}
pub unsafe fn test_ill_formed_utf8_73() {
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 2])>(0)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xdf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 2])>(0)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xef\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 3])>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xef\xbf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 2])>(0)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 3])>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7\xbf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 4])>(2)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7\xbf\xbf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 3])>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xdfa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 3])>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xefa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 4])>(2)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xef\xbfa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 3])>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7a\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 4])>(2)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7\xbfa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 5])>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7\xbf\xbfa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 9])>(4)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"a\xf1\x80\x80\xe1\x80\xc2b\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 5])>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf4\x8f\xbf\xbf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 5])>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf4\x90\x80\x80\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 5])>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf5\xb1\xb2\xb3\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 5])>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7\xbf\xbf\xbf\0"),
            )
    });
}
pub unsafe fn test_ill_formed_utf32_74() {
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 2])>(5)
            .call(
                unimplemented!(),
                unimplemented!(),
                &[1114111 as (), 0 as ()],
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 2])>(5)
            .call(
                unimplemented!(),
                unimplemented!(),
                &[1114112 as (), 0 as ()],
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [(); 2])>(5)
            .call(
                unimplemented!(),
                unimplemented!(),
                &[4294967295 as (), 0 as ()],
            )
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
    (unsafe { test_71() });
    (unsafe { test_72() });
    (unsafe { test_ill_formed_utf8_73() });
    (unsafe { test_ill_formed_utf32_74() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_39);
    std::cell::LazyCell::force(&*&raw const test_format_55);
    std::cell::LazyCell::force(&*&raw const test_format_to_56);
    std::cell::LazyCell::force(&*&raw const test_formatted_size_57);
    std::cell::LazyCell::force(&*&raw const test_format_to_n_58);
}
