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
pub unsafe fn operator_cmp_0(
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
            operator_cmp_0(
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
            operator_cmp_0(
                self as *const parse_call_validator,
                other as *const parse_call_validator,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for parse_call_validator {}
pub unsafe fn get_format_types_1() -> () {
    return unimplemented!();
}
pub type execution_modus = ();
pub const execution_modus_partial: execution_modus = 0;
pub const execution_modus_full: execution_modus = 1;
pub unsafe fn invalid_types_2(mut valid: ()) -> () {
    let mut result: () = unimplemented!();
    let types: () = unimplemented!();
    'loop_: for type_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut type_: () = (*type_);
        if ((unimplemented!()) != (unimplemented!())) {
            continue 'loop_;
        }
        switch!(match type_ {
            __v if __v == (*c"a".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"A".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"b".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"B".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"c".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"d".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"e".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"E".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"f".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"F".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"g".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"G".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"o".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"p".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"P".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"s".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"x".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"X".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"?".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == 0_() => {
                break;
            }
            _ => {
                unimplemented!();
            }
        });
    }
    return unimplemented!();
}
pub unsafe fn invalid_types_3(mut valid: ()) -> () {
    let mut result: () = unimplemented!();
    let types: () = unimplemented!();
    'loop_: for type_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut type_: () = (*type_);
        if ((unimplemented!()) != (unimplemented!())) {
            continue 'loop_;
        }
        switch!(match type_ {
            __v if __v == (*c"a".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"A".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"b".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"B".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"c".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"d".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"e".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"E".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"f".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"F".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"g".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"G".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"o".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"p".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"P".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"s".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"x".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"X".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == (*c"?".as_ptr().offset((0_()) as isize)) => {
                unimplemented!();
                break;
            }
            __v if __v == 0_() => {
                break;
            }
            _ => {
                unimplemented!();
            }
        });
    }
    return unimplemented!();
}
pub unsafe fn format_test_handle_4(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__status_ = {
            let mut __tmp_0: () = unimplemented!();
            std_basic_format_string_char__status_::new({ &mut __tmp_0 })
        };
        let mut _args: status = status_foo;
        check
            .spec::<fn((), std_basic_format_string_char__status_, *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__status_ = {
            let mut __tmp_1: () = unimplemented!();
            std_basic_format_string_char__status_::new({ &mut __tmp_1 })
        };
        let mut _args: status = status_foo;
        check
            .spec::<fn((), std_basic_format_string_char__status_, *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__status_ = {
            let mut __tmp_2: () = unimplemented!();
            std_basic_format_string_char__status_::new({ &mut __tmp_2 })
        };
        let mut _args: status = status_foo;
        check
            .spec::<fn((), std_basic_format_string_char__status_, *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__status_ = {
            let mut __tmp_3: () = unimplemented!();
            std_basic_format_string_char__status_::new({ &mut __tmp_3 })
        };
        let mut _args: status = status_foo;
        check
            .spec::<fn((), std_basic_format_string_char__status_, *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__status_ = {
            let mut __tmp_4: () = unimplemented!();
            std_basic_format_string_char__status_::new({ &mut __tmp_4 })
        };
        let mut _args: status = status_bar;
        check
            .spec::<fn((), std_basic_format_string_char__status_, *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__status_ = {
            let mut __tmp_5: () = unimplemented!();
            std_basic_format_string_char__status_::new({ &mut __tmp_5 })
        };
        let mut _args: status = status_bar;
        check
            .spec::<fn((), std_basic_format_string_char__status_, *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__status_ = {
            let mut __tmp_6: () = unimplemented!();
            std_basic_format_string_char__status_::new({ &mut __tmp_6 })
        };
        let mut _args: status = status_bar;
        check
            .spec::<fn((), std_basic_format_string_char__status_, *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__status_ = {
            let mut __tmp_7: () = unimplemented!();
            std_basic_format_string_char__status_::new({ &mut __tmp_7 })
        };
        let mut _args: status = status_bar;
        check
            .spec::<fn((), std_basic_format_string_char__status_, *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__status_ = {
            let mut __tmp_8: () = unimplemented!();
            std_basic_format_string_char__status_::new({ &mut __tmp_8 })
        };
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), std_basic_format_string_char__status_, *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__status_ = {
            let mut __tmp_9: () = unimplemented!();
            std_basic_format_string_char__status_::new({ &mut __tmp_9 })
        };
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), std_basic_format_string_char__status_, *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__status_ = {
            let mut __tmp_10: () = unimplemented!();
            std_basic_format_string_char__status_::new({ &mut __tmp_10 })
        };
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), std_basic_format_string_char__status_, *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__status_ = {
            let mut __tmp_11: () = unimplemented!();
            std_basic_format_string_char__status_::new({ &mut __tmp_11 })
        };
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), std_basic_format_string_char__status_, *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    let mut __tmp_12: () = (unsafe { invalid_types_2(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_12;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let fmt: *const () = &(*unimplemented!());
        (unsafe {
            let _arg0: () = unimplemented!();
            let _arg1: () = unimplemented!();
            let mut _arg2: status = status_foo;
            check_exception
                .spec::<fn((), (), *mut status)>(2)
                .call(_arg0, _arg1, &mut _arg2)
        });
    }
}
pub unsafe fn format_test_handle_5(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__status_ = {
            let mut __tmp_13: () = unimplemented!();
            std_basic_format_string_wchar_t__status_::new({ &mut __tmp_13 })
        };
        let mut _args: status = status_foo;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__status_, *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__status_ = {
            let mut __tmp_14: () = unimplemented!();
            std_basic_format_string_wchar_t__status_::new({ &mut __tmp_14 })
        };
        let mut _args: status = status_foo;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__status_, *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__status_ = {
            let mut __tmp_15: () = unimplemented!();
            std_basic_format_string_wchar_t__status_::new({ &mut __tmp_15 })
        };
        let mut _args: status = status_foo;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__status_, *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__status_ = {
            let mut __tmp_16: () = unimplemented!();
            std_basic_format_string_wchar_t__status_::new({ &mut __tmp_16 })
        };
        let mut _args: status = status_foo;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__status_, *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__status_ = {
            let mut __tmp_17: () = unimplemented!();
            std_basic_format_string_wchar_t__status_::new({ &mut __tmp_17 })
        };
        let mut _args: status = status_bar;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__status_, *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__status_ = {
            let mut __tmp_18: () = unimplemented!();
            std_basic_format_string_wchar_t__status_::new({ &mut __tmp_18 })
        };
        let mut _args: status = status_bar;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__status_, *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__status_ = {
            let mut __tmp_19: () = unimplemented!();
            std_basic_format_string_wchar_t__status_::new({ &mut __tmp_19 })
        };
        let mut _args: status = status_bar;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__status_, *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__status_ = {
            let mut __tmp_20: () = unimplemented!();
            std_basic_format_string_wchar_t__status_::new({ &mut __tmp_20 })
        };
        let mut _args: status = status_bar;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__status_, *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__status_ = {
            let mut __tmp_21: () = unimplemented!();
            std_basic_format_string_wchar_t__status_::new({ &mut __tmp_21 })
        };
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__status_, *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__status_ = {
            let mut __tmp_22: () = unimplemented!();
            std_basic_format_string_wchar_t__status_::new({ &mut __tmp_22 })
        };
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__status_, *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__status_ = {
            let mut __tmp_23: () = unimplemented!();
            std_basic_format_string_wchar_t__status_::new({ &mut __tmp_23 })
        };
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__status_, *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__status_ = {
            let mut __tmp_24: () = unimplemented!();
            std_basic_format_string_wchar_t__status_::new({ &mut __tmp_24 })
        };
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__status_, *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    let mut __tmp_25: () = (unsafe { invalid_types_3(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_25;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let fmt: *const () = &(*unimplemented!());
        (unsafe {
            let _arg1: () = unimplemented!();
            let mut _arg2: status = status_foo;
            check_exception.spec::<fn((), (), *mut status)>(5).call(
                unimplemented!(),
                _arg1,
                &mut _arg2,
            )
        });
    }
}
pub unsafe fn format_tests_6(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char_ = {
            let mut __tmp_26: () = unimplemented!();
            std_basic_format_string_char_::new({ &mut __tmp_26 })
        };
        check
            .spec::<fn((), std_basic_format_string_char_)>(0)
            .call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char_ = {
            let mut __tmp_27: () = unimplemented!();
            std_basic_format_string_char_::new({ &mut __tmp_27 })
        };
        check
            .spec::<fn((), std_basic_format_string_char_)>(0)
            .call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char_ = {
            let mut __tmp_28: () = unimplemented!();
            std_basic_format_string_char_::new({ &mut __tmp_28 })
        };
        check
            .spec::<fn((), std_basic_format_string_char_)>(0)
            .call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_29: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_29 })
        };
        let mut _args: () = (' ' as ());
        check
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char_ = {
            let mut __tmp_30: () = unimplemented!();
            std_basic_format_string_char_::new({ &mut __tmp_30 })
        };
        check
            .spec::<fn((), std_basic_format_string_char_)>(0)
            .call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char_ = {
            let mut __tmp_31: () = unimplemented!();
            std_basic_format_string_char__char_::new({ &mut __tmp_31 })
        };
        let mut _args: () = (' ' as ());
        check
            .spec::<fn((), std_basic_format_string_char__char_, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__bool__bool_ = {
            let mut __tmp_32: () = unimplemented!();
            std_basic_format_string_char__bool__bool_::new({ &mut __tmp_32 })
        };
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check
            .spec::<fn((), std_basic_format_string_char__bool__bool_, *mut (), *mut ())>(2)
            .call(_expected, _fmt, &mut _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__bool__bool_ = {
            let mut __tmp_33: () = unimplemented!();
            std_basic_format_string_char__bool__bool_::new({ &mut __tmp_33 })
        };
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check
            .spec::<fn((), std_basic_format_string_char__bool__bool_, *mut (), *mut ())>(2)
            .call(_expected, _fmt, &mut _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_  = { let mut __tmp_34 : ()  = unimplemented!() ; std_basic_format_string_char__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ :: new ( { & mut __tmp_34 } , )  } ;
        let mut _args_2: () = 1_();
        let mut _args_3: () = 2_();
        let mut _args_4: () = 3_();
        let mut _args_5: () = 4_();
        let mut _args_6: () = 5_();
        let mut _args_7: () = 6_();
        let mut _args_8: () = 7_();
        let mut _args_9: () = 8_();
        let mut _args_10: () = 9_();
        let mut _args_11: () = 0_();
        let mut _args_12: () = 1_();
        let mut _args_13: () = 2_();
        let mut _args_14: () = 3_();
        let mut _args_15: () = 4_();
        let mut _args_16: () = 5_();
        let mut _args_17: () = 6_();
        let mut _args_18: () = 7_();
        let mut _args_19: () = 8_();
        let mut _args_20: () = 9_();
        let mut _args_21: () = 0_();
        check  .spec::<fn(() ,std_basic_format_string_char__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,)>(3) .call ( _expected , _fmt , & mut _args_2 , & mut _args_3 , & mut _args_4 , & mut _args_5 , & mut _args_6 , & mut _args_7 , & mut _args_8 , & mut _args_9 , & mut _args_10 , & mut _args_11 , & mut _args_12 , & mut _args_13 , & mut _args_14 , & mut _args_15 , & mut _args_16 , & mut _args_17 , & mut _args_18 , & mut _args_19 , & mut _args_20 , & mut _args_21 , )
    });
    {}
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(0)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(1).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__char__char__char__char__char__char__char_ = {
            let mut __tmp_35: () = unimplemented!();
            std_basic_format_string_char__char__char__char__char__char__char__char_::new({
                &mut __tmp_35
            })
        };
        let mut _args_2: () = ('0' as ());
        let mut _args_3: () = ('9' as ());
        let mut _args_4: () = ('a' as ());
        let mut _args_5: () = ('z' as ());
        let mut _args_6: () = ('A' as ());
        let mut _args_7: () = ('Z' as ());
        let mut _args_8: () = ('!' as ());
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__char__char__char__char__char__char__char_,
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
            )>(4)
            .call(
                _expected,
                _fmt,
                &mut _args_2,
                &mut _args_3,
                &mut _args_4,
                &mut _args_5,
                &mut _args_6,
                &mut _args_7,
                &mut _args_8,
            )
    });
    {}
    {
        let mut buffer: [(); 8] = [
            ('0' as ()),
            ('9' as ()),
            ('a' as ()),
            ('z' as ()),
            ('A' as ()),
            ('Z' as ()),
            ('!' as ()),
            0_(),
        ];
        let mut data: *mut () = buffer.as_mut_ptr();
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__char_ptrref_ = {
                let mut __tmp_36: () = unimplemented!();
                std_basic_format_string_char__char_ptrref_::new({ &mut __tmp_36 })
            };
            check
                .spec::<fn((), std_basic_format_string_char__char_ptrref_, *mut *mut ())>(5)
                .call(_expected, _fmt, &mut data)
        });
    }
    {
        let mut buffer: [(); 8] = [
            ('0' as ()),
            ('9' as ()),
            ('a' as ()),
            ('z' as ()),
            ('A' as ()),
            ('Z' as ()),
            ('!' as ()),
            0_(),
        ];
        let mut data: *const () = (buffer.as_mut_ptr()).cast_const();
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__const_char_ptrref_ = {
                let mut __tmp_37: () = unimplemented!();
                std_basic_format_string_char__const_char_ptrref_::new({ &mut __tmp_37 })
            };
            check
                .spec::<fn((), std_basic_format_string_char__const_char_ptrref_, *mut *const ())>(6)
                .call(_expected, _fmt, &mut data)
        });
    }
    {
        let mut buffer: [(); 8] = [
            ('a' as ()),
            ('b' as ()),
            ('c' as ()),
            0_(),
            ('d' as ()),
            ('e' as ()),
            ('f' as ()),
            0_(),
        ];
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__char__ref_arr8arr_ = {
                let mut __tmp_38: () = unimplemented!();
                std_basic_format_string_char__char__ref_arr8arr_::new({ &mut __tmp_38 })
            };
            check
                .spec::<fn((), std_basic_format_string_char__char__ref_arr8arr_, *mut [(); 8])>(7)
                .call(_expected, _fmt, &mut buffer)
        });
        let mut buffer2: [(); 7] = [
            ('a' as ()),
            ('b' as ()),
            ('c' as ()),
            0_(),
            ('d' as ()),
            ('e' as ()),
            ('f' as ()),
        ];
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__char__ref_arr7arr_ = {
                let mut __tmp_39: () = unimplemented!();
                std_basic_format_string_char__char__ref_arr7arr_::new({ &mut __tmp_39 })
            };
            check
                .spec::<fn((), std_basic_format_string_char__char__ref_arr7arr_, *mut [(); 7])>(8)
                .call(_expected, _fmt, &mut buffer2)
        });
    }
    {
        let mut data: () = unimplemented!();
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_ref_ = {
                let mut __tmp_40: () = unimplemented!();
                std_basic_format_string_char__std_string_ref_::new({ &mut __tmp_40 })
            };
            check
                .spec::<fn((), std_basic_format_string_char__std_string_ref_, *mut ())>(9)
                .call(_expected, _fmt, &mut data)
        });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut data: () = (unsafe { buffer() });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_char__std_string_view_ref_ = {
                let mut __tmp_41: () = unimplemented!();
                std_basic_format_string_char__std_string_view_ref_::new({ &mut __tmp_41 })
            };
            check
                .spec::<fn((), std_basic_format_string_char__std_string_view_ref_, *mut ())>(10)
                .call(_expected, _fmt, &mut data)
        });
    }
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__bool__bool_ = {
            let mut __tmp_42: () = unimplemented!();
            std_basic_format_string_char__bool__bool_::new({ &mut __tmp_42 })
        };
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check
            .spec::<fn((), std_basic_format_string_char__bool__bool_, *mut (), *mut ())>(2)
            .call(_expected, _fmt, &mut _args_2, &mut _args_3)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__signed_char_ = {
            let mut __tmp_43: () = unimplemented!();
            std_basic_format_string_char__signed_char_::new({ &mut __tmp_43 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__signed_char_, *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__short_ = {
            let mut __tmp_44: () = unimplemented!();
            std_basic_format_string_char__short_::new({ &mut __tmp_44 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__short_, *mut ())>(12)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__int_ = {
            let mut __tmp_45: () = unimplemented!();
            std_basic_format_string_char__int_::new({ &mut __tmp_45 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__int_, *mut ())>(13)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__long_ = {
            let mut __tmp_46: () = unimplemented!();
            std_basic_format_string_char__long_::new({ &mut __tmp_46 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__long_, *mut ())>(14)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__long_long_ = {
            let mut __tmp_47: () = unimplemented!();
            std_basic_format_string_char__long_long_::new({ &mut __tmp_47 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__long_long_, *mut ())>(15)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char____int128_ = {
            let mut __tmp_48: () = unimplemented!();
            std_basic_format_string_char____int128_::new({ &mut __tmp_48 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char____int128_, *mut ())>(16)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__unsigned_char_ = {
            let mut __tmp_49: () = unimplemented!();
            std_basic_format_string_char__unsigned_char_::new({ &mut __tmp_49 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__unsigned_char_, *mut ())>(17)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__unsigned_short_ = {
            let mut __tmp_50: () = unimplemented!();
            std_basic_format_string_char__unsigned_short_::new({ &mut __tmp_50 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__unsigned_short_, *mut ())>(18)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__unsigned_int_ = {
            let mut __tmp_51: () = unimplemented!();
            std_basic_format_string_char__unsigned_int_::new({ &mut __tmp_51 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__unsigned_int_, *mut ())>(19)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__unsigned_long_ = {
            let mut __tmp_52: () = unimplemented!();
            std_basic_format_string_char__unsigned_long_::new({ &mut __tmp_52 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__unsigned_long_, *mut ())>(20)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__unsigned_long_long_ = {
            let mut __tmp_53: () = unimplemented!();
            std_basic_format_string_char__unsigned_long_long_::new({ &mut __tmp_53 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__unsigned_long_long_, *mut ())>(21)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__unsigned___int128_ = {
            let mut __tmp_54: () = unimplemented!();
            std_basic_format_string_char__unsigned___int128_::new({ &mut __tmp_54 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__unsigned___int128_, *mut ())>(22)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__float_ = {
            let mut __tmp_55: () = unimplemented!();
            std_basic_format_string_char__float_::new({ &mut __tmp_55 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__float_, *mut ())>(23)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__double_ = {
            let mut __tmp_56: () = unimplemented!();
            std_basic_format_string_char__double_::new({ &mut __tmp_56 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__double_, *mut ())>(24)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__long_double_ = {
            let mut __tmp_57: () = unimplemented!();
            std_basic_format_string_char__long_double_::new({ &mut __tmp_57 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_char__long_double_, *mut ())>(25)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_nullptr_t_ = {
            let mut __tmp_58: () = unimplemented!();
            std_basic_format_string_char__std_nullptr_t_::new({ &mut __tmp_58 })
        };
        let mut _args: () = Default::default();
        check
            .spec::<fn((), std_basic_format_string_char__std_nullptr_t_, *mut ())>(26)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__void_ptr_ = {
            let mut __tmp_59: () = unimplemented!();
            std_basic_format_string_char__void_ptr_::new({ &mut __tmp_59 })
        };
        let mut _args: () = (66_() as ());
        check
            .spec::<fn((), std_basic_format_string_char__void_ptr_, *mut ())>(27)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__const_void_ptr_ = {
            let mut __tmp_60: () = unimplemented!();
            std_basic_format_string_char__const_void_ptr_::new({ &mut __tmp_60 })
        };
        let mut _args: () = (66_() as ());
        check
            .spec::<fn((), std_basic_format_string_char__const_void_ptr_, *mut ())>(28)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe { format_test_handle_4(check.copy_from(), check_exception.copy_from()) });
    {}
}
pub unsafe fn format_tests_7(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t_ = {
            let mut __tmp_61: () = unimplemented!();
            std_basic_format_string_wchar_t_::new({ &mut __tmp_61 })
        };
        check
            .spec::<fn((), std_basic_format_string_wchar_t_)>(31)
            .call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t_ = {
            let mut __tmp_62: () = unimplemented!();
            std_basic_format_string_wchar_t_::new({ &mut __tmp_62 })
        };
        check
            .spec::<fn((), std_basic_format_string_wchar_t_)>(31)
            .call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t_ = {
            let mut __tmp_63: () = unimplemented!();
            std_basic_format_string_wchar_t_::new({ &mut __tmp_63 })
        };
        check
            .spec::<fn((), std_basic_format_string_wchar_t_)>(31)
            .call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_64: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new({ &mut __tmp_64 })
        };
        let mut _args: () = (' ' as ());
        check
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(32)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t_ = {
            let mut __tmp_65: () = unimplemented!();
            std_basic_format_string_wchar_t_::new({ &mut __tmp_65 })
        };
        check
            .spec::<fn((), std_basic_format_string_wchar_t_)>(31)
            .call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t_ = {
            let mut __tmp_66: () = unimplemented!();
            std_basic_format_string_wchar_t__wchar_t_::new({ &mut __tmp_66 })
        };
        let mut _args: () = (' ' as ());
        check
            .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_, *mut ())>(32)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__bool__bool_ = {
            let mut __tmp_67: () = unimplemented!();
            std_basic_format_string_wchar_t__bool__bool_::new({ &mut __tmp_67 })
        };
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__bool__bool_, *mut (), *mut ())>(33)
            .call(_expected, _fmt, &mut _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__bool__bool_ = {
            let mut __tmp_68: () = unimplemented!();
            std_basic_format_string_wchar_t__bool__bool_::new({ &mut __tmp_68 })
        };
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__bool__bool_, *mut (), *mut ())>(33)
            .call(_expected, _fmt, &mut _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_  = { let mut __tmp_69 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ :: new ( { & mut __tmp_69 } , )  } ;
        let mut _args_2: () = 1_();
        let mut _args_3: () = 2_();
        let mut _args_4: () = 3_();
        let mut _args_5: () = 4_();
        let mut _args_6: () = 5_();
        let mut _args_7: () = 6_();
        let mut _args_8: () = 7_();
        let mut _args_9: () = 8_();
        let mut _args_10: () = 9_();
        let mut _args_11: () = 0_();
        let mut _args_12: () = 1_();
        let mut _args_13: () = 2_();
        let mut _args_14: () = 3_();
        let mut _args_15: () = 4_();
        let mut _args_16: () = 5_();
        let mut _args_17: () = 6_();
        let mut _args_18: () = 7_();
        let mut _args_19: () = 8_();
        let mut _args_20: () = 9_();
        let mut _args_21: () = 0_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,)>(34) .call ( _expected , _fmt , & mut _args_2 , & mut _args_3 , & mut _args_4 , & mut _args_5 , & mut _args_6 , & mut _args_7 , & mut _args_8 , & mut _args_9 , & mut _args_10 , & mut _args_11 , & mut _args_12 , & mut _args_13 , & mut _args_14 , & mut _args_15 , & mut _args_16 , & mut _args_17 , & mut _args_18 , & mut _args_19 , & mut _args_20 , & mut _args_21 , )
    });
    {}
    (unsafe {
        check_exception
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        check_exception
            .spec::<fn((), ())>(3)
            .call(unimplemented!(), unimplemented!())
    });
    (unsafe {
        let mut _arg2: () = 42_();
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut _arg2,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t_  = { let mut __tmp_70 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t_ :: new ( { & mut __tmp_70 } , )  } ;
        let mut _args_2: () = ('0' as ());
        let mut _args_3: () = ('9' as ());
        let mut _args_4: () = ('a' as ());
        let mut _args_5: () = ('z' as ());
        let mut _args_6: () = ('A' as ());
        let mut _args_7: () = ('Z' as ());
        let mut _args_8: () = ('!' as ());
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t_ ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,*mut () ,)>(35) .call ( _expected , _fmt , & mut _args_2 , & mut _args_3 , & mut _args_4 , & mut _args_5 , & mut _args_6 , & mut _args_7 , & mut _args_8 , )
    });
    {}
    {
        let mut buffer: [(); 8] = [
            ('0' as ()),
            ('9' as ()),
            ('a' as ()),
            ('z' as ()),
            ('A' as ()),
            ('Z' as ()),
            ('!' as ()),
            0_(),
        ];
        let mut data: *mut () = buffer.as_mut_ptr();
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_wchar_t__wchar_t_ptrref_ = {
                let mut __tmp_71: () = unimplemented!();
                std_basic_format_string_wchar_t__wchar_t_ptrref_::new({ &mut __tmp_71 })
            };
            check
                .spec::<fn((), std_basic_format_string_wchar_t__wchar_t_ptrref_, *mut *mut ())>(36)
                .call(_expected, _fmt, &mut data)
        });
    }
    {
        let mut buffer: [(); 8] = [
            ('0' as ()),
            ('9' as ()),
            ('a' as ()),
            ('z' as ()),
            ('A' as ()),
            ('Z' as ()),
            ('!' as ()),
            0_(),
        ];
        let mut data: *const () = (buffer.as_mut_ptr()).cast_const();
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_wchar_t__const_wchar_t_ptrref_ = {
                let mut __tmp_72: () = unimplemented!();
                std_basic_format_string_wchar_t__const_wchar_t_ptrref_::new({ &mut __tmp_72 })
            };
            check  .spec::<fn(() ,std_basic_format_string_wchar_t__const_wchar_t_ptrref_ ,*mut *const () ,)>(37) .call ( _expected , _fmt , & mut data , )
        });
    }
    {
        let mut buffer: [(); 8] = [
            ('a' as ()),
            ('b' as ()),
            ('c' as ()),
            0_(),
            ('d' as ()),
            ('e' as ()),
            ('f' as ()),
            0_(),
        ];
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_wchar_t__wchar_t__ref_arr8arr_ = {
                let mut __tmp_73: () = unimplemented!();
                std_basic_format_string_wchar_t__wchar_t__ref_arr8arr_::new({ &mut __tmp_73 })
            };
            check  .spec::<fn(() ,std_basic_format_string_wchar_t__wchar_t__ref_arr8arr_ ,*mut [ () ; 8] ,)>(38) .call ( _expected , _fmt , & mut buffer , )
        });
        let mut buffer2: [(); 7] = [
            ('a' as ()),
            ('b' as ()),
            ('c' as ()),
            0_(),
            ('d' as ()),
            ('e' as ()),
            ('f' as ()),
        ];
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_wchar_t__wchar_t__ref_arr7arr_ = {
                let mut __tmp_74: () = unimplemented!();
                std_basic_format_string_wchar_t__wchar_t__ref_arr7arr_::new({ &mut __tmp_74 })
            };
            check  .spec::<fn(() ,std_basic_format_string_wchar_t__wchar_t__ref_arr7arr_ ,*mut [ () ; 7] ,)>(39) .call ( _expected , _fmt , & mut buffer2 , )
        });
    }
    {
        let mut data: () = unimplemented!();
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_wchar_t__std_wstring_ref_ = {
                let mut __tmp_75: () = unimplemented!();
                std_basic_format_string_wchar_t__std_wstring_ref_::new({ &mut __tmp_75 })
            };
            check
                .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_ref_, *mut ())>(40)
                .call(_expected, _fmt, &mut data)
        });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut data: () = (unsafe { buffer() });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: std_basic_format_string_wchar_t__std_wstring_view_ref_ = {
                let mut __tmp_76: () = unimplemented!();
                std_basic_format_string_wchar_t__std_wstring_view_ref_::new({ &mut __tmp_76 })
            };
            check
                .spec::<fn((), std_basic_format_string_wchar_t__std_wstring_view_ref_, *mut ())>(41)
                .call(_expected, _fmt, &mut data)
        });
    }
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__bool__bool_ = {
            let mut __tmp_77: () = unimplemented!();
            std_basic_format_string_wchar_t__bool__bool_::new({ &mut __tmp_77 })
        };
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__bool__bool_, *mut (), *mut ())>(33)
            .call(_expected, _fmt, &mut _args_2, &mut _args_3)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__signed_char_ = {
            let mut __tmp_78: () = unimplemented!();
            std_basic_format_string_wchar_t__signed_char_::new({ &mut __tmp_78 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__signed_char_, *mut ())>(42)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__short_ = {
            let mut __tmp_79: () = unimplemented!();
            std_basic_format_string_wchar_t__short_::new({ &mut __tmp_79 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__short_, *mut ())>(43)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__int_ = {
            let mut __tmp_80: () = unimplemented!();
            std_basic_format_string_wchar_t__int_::new({ &mut __tmp_80 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__int_, *mut ())>(44)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__long_ = {
            let mut __tmp_81: () = unimplemented!();
            std_basic_format_string_wchar_t__long_::new({ &mut __tmp_81 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__long_, *mut ())>(45)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__long_long_ = {
            let mut __tmp_82: () = unimplemented!();
            std_basic_format_string_wchar_t__long_long_::new({ &mut __tmp_82 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__long_long_, *mut ())>(46)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t____int128_ = {
            let mut __tmp_83: () = unimplemented!();
            std_basic_format_string_wchar_t____int128_::new({ &mut __tmp_83 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t____int128_, *mut ())>(47)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__unsigned_char_ = {
            let mut __tmp_84: () = unimplemented!();
            std_basic_format_string_wchar_t__unsigned_char_::new({ &mut __tmp_84 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__unsigned_char_, *mut ())>(48)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__unsigned_short_ = {
            let mut __tmp_85: () = unimplemented!();
            std_basic_format_string_wchar_t__unsigned_short_::new({ &mut __tmp_85 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__unsigned_short_, *mut ())>(49)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__unsigned_int_ = {
            let mut __tmp_86: () = unimplemented!();
            std_basic_format_string_wchar_t__unsigned_int_::new({ &mut __tmp_86 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__unsigned_int_, *mut ())>(50)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__unsigned_long_ = {
            let mut __tmp_87: () = unimplemented!();
            std_basic_format_string_wchar_t__unsigned_long_::new({ &mut __tmp_87 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__unsigned_long_, *mut ())>(51)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__unsigned_long_long_ = {
            let mut __tmp_88: () = unimplemented!();
            std_basic_format_string_wchar_t__unsigned_long_long_::new({ &mut __tmp_88 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__unsigned_long_long_, *mut ())>(52)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__unsigned___int128_ = {
            let mut __tmp_89: () = unimplemented!();
            std_basic_format_string_wchar_t__unsigned___int128_::new({ &mut __tmp_89 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__unsigned___int128_, *mut ())>(53)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__float_ = {
            let mut __tmp_90: () = unimplemented!();
            std_basic_format_string_wchar_t__float_::new({ &mut __tmp_90 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__float_, *mut ())>(54)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__double_ = {
            let mut __tmp_91: () = unimplemented!();
            std_basic_format_string_wchar_t__double_::new({ &mut __tmp_91 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__double_, *mut ())>(55)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__long_double_ = {
            let mut __tmp_92: () = unimplemented!();
            std_basic_format_string_wchar_t__long_double_::new({ &mut __tmp_92 })
        };
        let mut _args: () = 42_();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__long_double_, *mut ())>(56)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_nullptr_t_ = {
            let mut __tmp_93: () = unimplemented!();
            std_basic_format_string_wchar_t__std_nullptr_t_::new({ &mut __tmp_93 })
        };
        let mut _args: () = Default::default();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_nullptr_t_, *mut ())>(57)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__void_ptr_ = {
            let mut __tmp_94: () = unimplemented!();
            std_basic_format_string_wchar_t__void_ptr_::new({ &mut __tmp_94 })
        };
        let mut _args: () = (66_() as ());
        check
            .spec::<fn((), std_basic_format_string_wchar_t__void_ptr_, *mut ())>(58)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__const_void_ptr_ = {
            let mut __tmp_95: () = unimplemented!();
            std_basic_format_string_wchar_t__const_void_ptr_::new({ &mut __tmp_95 })
        };
        let mut _args: () = (66_() as ());
        check
            .spec::<fn((), std_basic_format_string_wchar_t__const_void_ptr_, *mut ())>(59)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe { format_test_handle_5(check.copy_from(), check_exception.copy_from()) });
    {}
}
pub unsafe fn format_tests_char_to_wchar_t_8(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__char__char__char__char__char_ = {
            let mut __tmp_96: () = unimplemented!();
            std_basic_format_string_wchar_t__char__char__char__char__char_::new({ &mut __tmp_96 })
        };
        let mut _args_2: () = ('0' as ());
        let mut _args_3: () = ('9' as ());
        let mut _args_4: () = ('a' as ());
        let mut _args_5: () = ('z' as ());
        let mut _args_6: () = ('A' as ());
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__char__char__char__char__char_,
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
            )>(30)
            .call(
                _expected,
                _fmt,
                &mut _args_2,
                &mut _args_3,
                &mut _args_4,
                &mut _args_5,
                &mut _args_6,
            )
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct support_double_move_tracker {
    moved_from_: (),
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        unimplemented!();
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        unimplemented!();
        (*other).moved_from_ = true;
        self.moved_from_ = false;
        return &mut (*(self as *mut support_double_move_tracker));
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        unsafe { support_double_move_tracker::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.it_.offset((n) as isize));
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_int_ptr_,
    ) -> *mut cpp20_input_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_output_iterator_int_ptr_,
    ) -> *mut cpp20_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_output_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
pub unsafe fn base_9(i: *const cpp20_output_iterator_char_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_char_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_char_ptr_ {
    pub unsafe fn new(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut cpp20_output_iterator_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_output_iterator_char_ptr_,
    ) -> *mut cpp20_output_iterator_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_output_iterator_char_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp20_output_iterator_char_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp20_output_iterator_char_ptr_));
    }
}
pub unsafe fn base_10(i: *const cpp20_output_iterator_wchar_t_ptr_) -> *mut () {
    return (*i).it_;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_wchar_t_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_wchar_t_ptr_ {
    pub unsafe fn new(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut cpp20_output_iterator_wchar_t_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_output_iterator_wchar_t_ptr_,
    ) -> *mut cpp20_output_iterator_wchar_t_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_output_iterator_wchar_t_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp20_output_iterator_wchar_t_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp20_output_iterator_wchar_t_ptr_));
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IteratorOpCounts {
    pub increments: (),
    pub decrements: (),
    pub zero_moves: (),
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
pub unsafe fn operator_eq_11(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> () {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut (),
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_11(
                self as *const forward_sized_iterator_int_ptr_,
                other as *const forward_sized_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            forward_sized_iterator_int_ptr_ {
                it_: std::ptr::null_mut(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub unsafe fn operator_add_12(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_13(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_12(_i, n)
    });
}
pub unsafe fn iter_swap_14(mut a: adl_Iterator, mut b: adl_Iterator) {
    (unsafe {
        let ___x: *mut *mut () = &mut a.ptr_;
        let ___y: *mut *mut () = &mut b.ptr_;
        swap_15(___x, ___y)
    });
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_16(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut *(&raw mut (*(unsafe { adl_Iterator::operator_deref(&iter) }))).cast::<()>();
}
pub unsafe fn operator_eq_17(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_18(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut (),
    iter_moves_: *mut (),
    iter_swaps_: *mut (),
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut (), mut iter_moves: *mut (), mut iter_swaps: *mut ()) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut (), iter_moves: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut (), iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> () {
        unimplemented!();
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> () {
        unimplemented!();
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<()>()) as ()
            as ());
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_dec();
        return prev;
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_18(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_17(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl Default for adl_Iterator {
    fn default() -> Self {
        unsafe {
            adl_Iterator {
                ptr_: std::ptr::null_mut(),
                iter_moves_: std::ptr::null_mut(),
                iter_swaps_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct rvalue_iterator_int_ptr_ {
    it_: *mut *mut (),
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        return (((*(self as *const rvalue_iterator_int_ptr_)).it_) == ((*_a0).it_));
    }
}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
        }
    }
}
impl std::cmp::PartialOrd for rvalue_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for rvalue_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
                == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const ()) -> Self {
        let mut this = Self {};
        this
    }
}
pub static mut test_19: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(|| unsafe {
    lambda_unsafe! ( Generic , { } , | mut expected : () , mut fmt : std_basic_format_string_char_ , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { format_to_20 ( cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  , unimplemented!() , fmt .clone() , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__char_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_21 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__bool__bool_ , args_2 : *mut () , args_3 : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; format_to_22 ( ___out_it , unimplemented!() , ___fmt , ___args_3 , ___args_4 , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , args_6 : *mut () , args_7 : *mut () , args_8 : *mut () , args_9 : *mut () , args_10 : *mut () , args_11 : *mut () , args_12 : *mut () , args_13 : *mut () , args_14 : *mut () , args_15 : *mut () , args_16 : *mut () , args_17 : *mut () , args_18 : *mut () , args_19 : *mut () , args_20 : *mut () , args_21 : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; let ___args_10: *mut ()  = args_9 ; let ___args_11: *mut ()  = args_10 ; let ___args_12: *mut ()  = args_11 ; let ___args_13: *mut ()  = args_12 ; let ___args_14: *mut ()  = args_13 ; let ___args_15: *mut ()  = args_14 ; let ___args_16: *mut ()  = args_15 ; let ___args_17: *mut ()  = args_16 ; let ___args_18: *mut ()  = args_17 ; let ___args_19: *mut ()  = args_18 ; let ___args_20: *mut ()  = args_19 ; let ___args_21: *mut ()  = args_20 ; let ___args_22: *mut ()  = args_21 ; format_to_23 ( ___out_it , unimplemented!() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ___args_10 , ___args_11 , ___args_12 , ___args_13 , ___args_14 , ___args_15 , ___args_16 , ___args_17 , ___args_18 , ___args_19 , ___args_20 , ___args_21 , ___args_22 , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__char__char__char__char__char__char__char_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , args_6 : *mut () , args_7 : *mut () , args_8 : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; format_to_24 ( ___out_it , unimplemented!() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__char_ptrref_ , args : *mut *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *mut ()  = args ; format_to_25 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__const_char_ptrref_ , args : *mut *const () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *const ()  = args ; format_to_26 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__char__ref_arr8arr_ , args : *mut [ () ; 8] , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 8]  = args ; format_to_27 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__char__ref_arr7arr_ , args : *mut [ () ; 7] , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 7]  = args ; format_to_28 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__std_string_ref_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_29 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__std_string_view_ref_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_30 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__signed_char_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_31 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__short_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_32 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__int_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_33 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__long_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_34 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__long_long_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_35 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char____int128_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_36 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__unsigned_char_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_37 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__unsigned_short_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_38 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__unsigned_int_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_39 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__unsigned_long_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_40 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__unsigned_long_long_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_41 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__unsigned___int128_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_42 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__float_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_43 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__double_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_44 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__long_double_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_45 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__std_nullptr_t_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_46 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__void_ptr_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_47 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__const_void_ptr_ , args : *mut () , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_48 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__status_ , args : *mut status , | { { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_char_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_char_ptr_  = cpp20_output_iterator_char_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut status  = args ; format_to_49 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__char__char__char__char__char_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , args_6 : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; format_to_50 ( ___out_it , unimplemented!() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t_ , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { format_to_51 ( cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  , unimplemented!() , fmt .clone() , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__wchar_t_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_52 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__bool__bool_ , args_2 : *mut () , args_3 : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; format_to_53 ( ___out_it , unimplemented!() , ___fmt , ___args_3 , ___args_4 , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , args_6 : *mut () , args_7 : *mut () , args_8 : *mut () , args_9 : *mut () , args_10 : *mut () , args_11 : *mut () , args_12 : *mut () , args_13 : *mut () , args_14 : *mut () , args_15 : *mut () , args_16 : *mut () , args_17 : *mut () , args_18 : *mut () , args_19 : *mut () , args_20 : *mut () , args_21 : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; let ___args_10: *mut ()  = args_9 ; let ___args_11: *mut ()  = args_10 ; let ___args_12: *mut ()  = args_11 ; let ___args_13: *mut ()  = args_12 ; let ___args_14: *mut ()  = args_13 ; let ___args_15: *mut ()  = args_14 ; let ___args_16: *mut ()  = args_15 ; let ___args_17: *mut ()  = args_16 ; let ___args_18: *mut ()  = args_17 ; let ___args_19: *mut ()  = args_18 ; let ___args_20: *mut ()  = args_19 ; let ___args_21: *mut ()  = args_20 ; let ___args_22: *mut ()  = args_21 ; format_to_54 ( ___out_it , unimplemented!() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ___args_10 , ___args_11 , ___args_12 , ___args_13 , ___args_14 , ___args_15 , ___args_16 , ___args_17 , ___args_18 , ___args_19 , ___args_20 , ___args_21 , ___args_22 , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , args_6 : *mut () , args_7 : *mut () , args_8 : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; format_to_55 ( ___out_it , unimplemented!() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__wchar_t_ptrref_ , args : *mut *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *mut ()  = args ; format_to_56 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__const_wchar_t_ptrref_ , args : *mut *const () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *const ()  = args ; format_to_57 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__wchar_t__ref_arr8arr_ , args : *mut [ () ; 8] , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 8]  = args ; format_to_58 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__wchar_t__ref_arr7arr_ , args : *mut [ () ; 7] , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 7]  = args ; format_to_59 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_wstring_ref_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_60 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_wstring_view_ref_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_61 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__signed_char_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_62 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__short_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_63 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__int_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_64 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__long_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_65 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__long_long_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_66 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t____int128_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_67 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__unsigned_char_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_68 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__unsigned_short_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_69 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__unsigned_int_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_70 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__unsigned_long_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_71 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__unsigned_long_long_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_72 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__unsigned___int128_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_73 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__float_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_74 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__double_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_75 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__long_double_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_76 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_nullptr_t_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_77 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__void_ptr_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_78 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__const_void_ptr_ , args : *mut () , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_79 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__status_ , args : *mut status , | { { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { unimplemented!()  } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut it : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { unimplemented!()  ;
 let mut out : [ () ; 4096] = [unimplemented!(); 4096] ;
  ;
 ;
 let mut it : cpp20_output_iterator_wchar_t_ptr_ = ( unsafe { let ___out_it: cpp20_output_iterator_wchar_t_ptr_  = cpp20_output_iterator_wchar_t_ptr_ :: new ( { out .as_mut_ptr() } , )  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut status  = args ; format_to_80 ( ___out_it , unimplemented!() , ___fmt , ___args , ) } )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } )
});
pub static mut test_exception_81: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut _a0: (), mut _a1: ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut status| {},
            |mut _a0: (), mut _a1: ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut status| {}
        )
    });

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
    (unsafe {
        format_tests_6(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_19)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_81)).copy_from(),
        )
    });
    (unsafe {
        format_tests_char_to_wchar_t_8(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_19)).copy_from(),
        )
    });
    (unsafe {
        format_tests_7(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_19)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_81)).copy_from(),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_19);
    std::cell::LazyCell::force(&*&raw const test_exception_81);
}
