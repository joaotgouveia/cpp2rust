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
        if ((unimplemented!()) != (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_3))) {
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
pub unsafe fn invalid_types_4(mut valid: ()) -> () {
    let mut result: () = unimplemented!();
    let types: () = unimplemented!();
    'loop_: for type_ in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut type_: () = (*type_);
        if ((unimplemented!()) != (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_3))) {
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
pub unsafe fn format_test_handle_5(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
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
pub unsafe fn format_test_handle_6(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
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
    let mut __tmp_25: () = (unsafe { invalid_types_4(unimplemented!()) });
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
pub unsafe fn format_tests_7(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
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
    (unsafe { format_test_handle_5(check.copy_from(), check_exception.copy_from()) });
    {}
}
pub unsafe fn format_tests_8(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
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
    (unsafe { format_test_handle_6(check.copy_from(), check_exception.copy_from()) });
    {}
}
pub unsafe fn format_tests_char_to_wchar_t_9(mut check: FnPtr<Generic>) {
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
pub static mut test_10: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(|| unsafe {
    lambda_unsafe! ( Generic , { } , | mut expected : () , mut fmt : std_basic_format_string_char_ , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { format_to_n_11 ( unimplemented!()  , 0_() , fmt .clone() , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { format_to_n_12 ( unimplemented!()  , 5_() , fmt .clone() , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { format_to_n_13 ( unimplemented!()  , 1000_() , fmt .clone() , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { format_to_n_14 ( unimplemented!()  , 10_() , fmt .clone() , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { format_to_n_15 ( buffer .as_mut_ptr() , - 1_() , fmt .clone() , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__char_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_16 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_17 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_18 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_19 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_20 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__bool__bool_ , args_2 : *mut () , args_3 : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; format_to_n_21 ( ___out_it , 0_() , ___fmt , ___args_3 , ___args_4 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; format_to_n_22 ( ___out_it , 5_() , ___fmt , ___args_3 , ___args_4 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; format_to_n_23 ( ___out_it , 1000_() , ___fmt , ___args_3 , ___args_4 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; format_to_n_24 ( ___out_it , 10_() , ___fmt , ___args_3 , ___args_4 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; format_to_n_25 ( ___out_it , - 1_() , ___fmt , ___args_3 , ___args_4 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , args_6 : *mut () , args_7 : *mut () , args_8 : *mut () , args_9 : *mut () , args_10 : *mut () , args_11 : *mut () , args_12 : *mut () , args_13 : *mut () , args_14 : *mut () , args_15 : *mut () , args_16 : *mut () , args_17 : *mut () , args_18 : *mut () , args_19 : *mut () , args_20 : *mut () , args_21 : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; let ___args_10: *mut ()  = args_9 ; let ___args_11: *mut ()  = args_10 ; let ___args_12: *mut ()  = args_11 ; let ___args_13: *mut ()  = args_12 ; let ___args_14: *mut ()  = args_13 ; let ___args_15: *mut ()  = args_14 ; let ___args_16: *mut ()  = args_15 ; let ___args_17: *mut ()  = args_16 ; let ___args_18: *mut ()  = args_17 ; let ___args_19: *mut ()  = args_18 ; let ___args_20: *mut ()  = args_19 ; let ___args_21: *mut ()  = args_20 ; let ___args_22: *mut ()  = args_21 ; format_to_n_26 ( ___out_it , 0_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ___args_10 , ___args_11 , ___args_12 , ___args_13 , ___args_14 , ___args_15 , ___args_16 , ___args_17 , ___args_18 , ___args_19 , ___args_20 , ___args_21 , ___args_22 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; let ___args_10: *mut ()  = args_9 ; let ___args_11: *mut ()  = args_10 ; let ___args_12: *mut ()  = args_11 ; let ___args_13: *mut ()  = args_12 ; let ___args_14: *mut ()  = args_13 ; let ___args_15: *mut ()  = args_14 ; let ___args_16: *mut ()  = args_15 ; let ___args_17: *mut ()  = args_16 ; let ___args_18: *mut ()  = args_17 ; let ___args_19: *mut ()  = args_18 ; let ___args_20: *mut ()  = args_19 ; let ___args_21: *mut ()  = args_20 ; let ___args_22: *mut ()  = args_21 ; format_to_n_27 ( ___out_it , 5_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ___args_10 , ___args_11 , ___args_12 , ___args_13 , ___args_14 , ___args_15 , ___args_16 , ___args_17 , ___args_18 , ___args_19 , ___args_20 , ___args_21 , ___args_22 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; let ___args_10: *mut ()  = args_9 ; let ___args_11: *mut ()  = args_10 ; let ___args_12: *mut ()  = args_11 ; let ___args_13: *mut ()  = args_12 ; let ___args_14: *mut ()  = args_13 ; let ___args_15: *mut ()  = args_14 ; let ___args_16: *mut ()  = args_15 ; let ___args_17: *mut ()  = args_16 ; let ___args_18: *mut ()  = args_17 ; let ___args_19: *mut ()  = args_18 ; let ___args_20: *mut ()  = args_19 ; let ___args_21: *mut ()  = args_20 ; let ___args_22: *mut ()  = args_21 ; format_to_n_28 ( ___out_it , 1000_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ___args_10 , ___args_11 , ___args_12 , ___args_13 , ___args_14 , ___args_15 , ___args_16 , ___args_17 , ___args_18 , ___args_19 , ___args_20 , ___args_21 , ___args_22 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; let ___args_10: *mut ()  = args_9 ; let ___args_11: *mut ()  = args_10 ; let ___args_12: *mut ()  = args_11 ; let ___args_13: *mut ()  = args_12 ; let ___args_14: *mut ()  = args_13 ; let ___args_15: *mut ()  = args_14 ; let ___args_16: *mut ()  = args_15 ; let ___args_17: *mut ()  = args_16 ; let ___args_18: *mut ()  = args_17 ; let ___args_19: *mut ()  = args_18 ; let ___args_20: *mut ()  = args_19 ; let ___args_21: *mut ()  = args_20 ; let ___args_22: *mut ()  = args_21 ; format_to_n_29 ( ___out_it , 10_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ___args_10 , ___args_11 , ___args_12 , ___args_13 , ___args_14 , ___args_15 , ___args_16 , ___args_17 , ___args_18 , ___args_19 , ___args_20 , ___args_21 , ___args_22 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; let ___args_10: *mut ()  = args_9 ; let ___args_11: *mut ()  = args_10 ; let ___args_12: *mut ()  = args_11 ; let ___args_13: *mut ()  = args_12 ; let ___args_14: *mut ()  = args_13 ; let ___args_15: *mut ()  = args_14 ; let ___args_16: *mut ()  = args_15 ; let ___args_17: *mut ()  = args_16 ; let ___args_18: *mut ()  = args_17 ; let ___args_19: *mut ()  = args_18 ; let ___args_20: *mut ()  = args_19 ; let ___args_21: *mut ()  = args_20 ; let ___args_22: *mut ()  = args_21 ; format_to_n_30 ( ___out_it , - 1_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ___args_10 , ___args_11 , ___args_12 , ___args_13 , ___args_14 , ___args_15 , ___args_16 , ___args_17 , ___args_18 , ___args_19 , ___args_20 , ___args_21 , ___args_22 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__char__char__char__char__char__char__char_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , args_6 : *mut () , args_7 : *mut () , args_8 : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; format_to_n_31 ( ___out_it , 0_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; format_to_n_32 ( ___out_it , 5_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; format_to_n_33 ( ___out_it , 1000_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; format_to_n_34 ( ___out_it , 10_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; format_to_n_35 ( ___out_it , - 1_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__char_ptrref_ , args : *mut *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *mut ()  = args ; format_to_n_36 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *mut ()  = args ; format_to_n_37 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *mut ()  = args ; format_to_n_38 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *mut ()  = args ; format_to_n_39 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *mut ()  = args ; format_to_n_40 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__const_char_ptrref_ , args : *mut *const () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *const ()  = args ; format_to_n_41 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *const ()  = args ; format_to_n_42 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *const ()  = args ; format_to_n_43 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *const ()  = args ; format_to_n_44 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *const ()  = args ; format_to_n_45 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__char__ref_arr8arr_ , args : *mut [ () ; 8] , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 8]  = args ; format_to_n_46 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 8]  = args ; format_to_n_47 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 8]  = args ; format_to_n_48 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 8]  = args ; format_to_n_49 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 8]  = args ; format_to_n_50 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__char__ref_arr7arr_ , args : *mut [ () ; 7] , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 7]  = args ; format_to_n_51 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 7]  = args ; format_to_n_52 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 7]  = args ; format_to_n_53 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 7]  = args ; format_to_n_54 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 7]  = args ; format_to_n_55 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__std_string_ref_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_56 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_57 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_58 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_59 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_60 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__std_string_view_ref_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_61 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_62 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_63 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_64 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_65 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__signed_char_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_66 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_67 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_68 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_69 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_70 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__short_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_71 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_72 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_73 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_74 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_75 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__int_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_76 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_77 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_78 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_79 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_80 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__long_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_81 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_82 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_83 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_84 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___n: ()  = - 1_() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_85 ( ___out_it , ___n , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__long_long_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_86 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_87 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_88 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_89 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_90 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char____int128_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_91 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_92 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_93 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_94 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_95 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__unsigned_char_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_96 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_97 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_98 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_99 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_100 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__unsigned_short_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_101 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_102 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_103 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_104 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_105 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__unsigned_int_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_106 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_107 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_108 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_109 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_110 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__unsigned_long_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_111 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_112 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_113 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_114 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_115 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__unsigned_long_long_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_116 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_117 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_118 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_119 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_120 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__unsigned___int128_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_121 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_122 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_123 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_124 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_125 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__float_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_126 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_127 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_128 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_129 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_130 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__double_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_131 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_132 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_133 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_134 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_135 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__long_double_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_136 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_137 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_138 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_139 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_140 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__std_nullptr_t_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_141 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_142 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_143 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_144 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_145 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__void_ptr_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_146 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_147 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_148 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_149 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_150 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__const_void_ptr_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_151 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_152 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_153 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_154 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_155 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_char__status_ , args : *mut status , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut status  = args ; format_to_n_156 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut status  = args ; format_to_n_157 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut status  = args ; format_to_n_158 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_char__std_char_traits_char___std_allocator_char__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut status  = args ; format_to_n_159 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut status  = args ; format_to_n_160 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__char__char__char__char__char_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , args_6 : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; format_to_n_161 ( ___out_it , 0_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; format_to_n_162 ( ___out_it , 5_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; format_to_n_163 ( ___out_it , 1000_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; format_to_n_164 ( ___out_it , 10_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; format_to_n_165 ( ___out_it , - 1_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t_ , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { format_to_n_166 ( unimplemented!()  , 0_() , fmt .clone() , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { format_to_n_167 ( unimplemented!()  , 5_() , fmt .clone() , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { format_to_n_168 ( unimplemented!()  , 1000_() , fmt .clone() , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { format_to_n_169 ( unimplemented!()  , 10_() , fmt .clone() , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { format_to_n_170 ( buffer .as_mut_ptr() , - 1_() , fmt .clone() , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__wchar_t_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_171 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_172 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_173 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_174 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_175 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__bool__bool_ , args_2 : *mut () , args_3 : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; format_to_n_176 ( ___out_it , 0_() , ___fmt , ___args_3 , ___args_4 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; format_to_n_177 ( ___out_it , 5_() , ___fmt , ___args_3 , ___args_4 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; format_to_n_178 ( ___out_it , 1000_() , ___fmt , ___args_3 , ___args_4 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; format_to_n_179 ( ___out_it , 10_() , ___fmt , ___args_3 , ___args_4 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; format_to_n_180 ( ___out_it , - 1_() , ___fmt , ___args_3 , ___args_4 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , args_6 : *mut () , args_7 : *mut () , args_8 : *mut () , args_9 : *mut () , args_10 : *mut () , args_11 : *mut () , args_12 : *mut () , args_13 : *mut () , args_14 : *mut () , args_15 : *mut () , args_16 : *mut () , args_17 : *mut () , args_18 : *mut () , args_19 : *mut () , args_20 : *mut () , args_21 : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; let ___args_10: *mut ()  = args_9 ; let ___args_11: *mut ()  = args_10 ; let ___args_12: *mut ()  = args_11 ; let ___args_13: *mut ()  = args_12 ; let ___args_14: *mut ()  = args_13 ; let ___args_15: *mut ()  = args_14 ; let ___args_16: *mut ()  = args_15 ; let ___args_17: *mut ()  = args_16 ; let ___args_18: *mut ()  = args_17 ; let ___args_19: *mut ()  = args_18 ; let ___args_20: *mut ()  = args_19 ; let ___args_21: *mut ()  = args_20 ; let ___args_22: *mut ()  = args_21 ; format_to_n_181 ( ___out_it , 0_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ___args_10 , ___args_11 , ___args_12 , ___args_13 , ___args_14 , ___args_15 , ___args_16 , ___args_17 , ___args_18 , ___args_19 , ___args_20 , ___args_21 , ___args_22 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; let ___args_10: *mut ()  = args_9 ; let ___args_11: *mut ()  = args_10 ; let ___args_12: *mut ()  = args_11 ; let ___args_13: *mut ()  = args_12 ; let ___args_14: *mut ()  = args_13 ; let ___args_15: *mut ()  = args_14 ; let ___args_16: *mut ()  = args_15 ; let ___args_17: *mut ()  = args_16 ; let ___args_18: *mut ()  = args_17 ; let ___args_19: *mut ()  = args_18 ; let ___args_20: *mut ()  = args_19 ; let ___args_21: *mut ()  = args_20 ; let ___args_22: *mut ()  = args_21 ; format_to_n_182 ( ___out_it , 5_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ___args_10 , ___args_11 , ___args_12 , ___args_13 , ___args_14 , ___args_15 , ___args_16 , ___args_17 , ___args_18 , ___args_19 , ___args_20 , ___args_21 , ___args_22 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; let ___args_10: *mut ()  = args_9 ; let ___args_11: *mut ()  = args_10 ; let ___args_12: *mut ()  = args_11 ; let ___args_13: *mut ()  = args_12 ; let ___args_14: *mut ()  = args_13 ; let ___args_15: *mut ()  = args_14 ; let ___args_16: *mut ()  = args_15 ; let ___args_17: *mut ()  = args_16 ; let ___args_18: *mut ()  = args_17 ; let ___args_19: *mut ()  = args_18 ; let ___args_20: *mut ()  = args_19 ; let ___args_21: *mut ()  = args_20 ; let ___args_22: *mut ()  = args_21 ; format_to_n_183 ( ___out_it , 1000_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ___args_10 , ___args_11 , ___args_12 , ___args_13 , ___args_14 , ___args_15 , ___args_16 , ___args_17 , ___args_18 , ___args_19 , ___args_20 , ___args_21 , ___args_22 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; let ___args_10: *mut ()  = args_9 ; let ___args_11: *mut ()  = args_10 ; let ___args_12: *mut ()  = args_11 ; let ___args_13: *mut ()  = args_12 ; let ___args_14: *mut ()  = args_13 ; let ___args_15: *mut ()  = args_14 ; let ___args_16: *mut ()  = args_15 ; let ___args_17: *mut ()  = args_16 ; let ___args_18: *mut ()  = args_17 ; let ___args_19: *mut ()  = args_18 ; let ___args_20: *mut ()  = args_19 ; let ___args_21: *mut ()  = args_20 ; let ___args_22: *mut ()  = args_21 ; format_to_n_184 ( ___out_it , 10_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ___args_10 , ___args_11 , ___args_12 , ___args_13 , ___args_14 , ___args_15 , ___args_16 , ___args_17 , ___args_18 , ___args_19 , ___args_20 , ___args_21 , ___args_22 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; let ___args_10: *mut ()  = args_9 ; let ___args_11: *mut ()  = args_10 ; let ___args_12: *mut ()  = args_11 ; let ___args_13: *mut ()  = args_12 ; let ___args_14: *mut ()  = args_13 ; let ___args_15: *mut ()  = args_14 ; let ___args_16: *mut ()  = args_15 ; let ___args_17: *mut ()  = args_16 ; let ___args_18: *mut ()  = args_17 ; let ___args_19: *mut ()  = args_18 ; let ___args_20: *mut ()  = args_19 ; let ___args_21: *mut ()  = args_20 ; let ___args_22: *mut ()  = args_21 ; format_to_n_185 ( ___out_it , - 1_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ___args_10 , ___args_11 , ___args_12 , ___args_13 , ___args_14 , ___args_15 , ___args_16 , ___args_17 , ___args_18 , ___args_19 , ___args_20 , ___args_21 , ___args_22 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t__wchar_t_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , args_6 : *mut () , args_7 : *mut () , args_8 : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; format_to_n_186 ( ___out_it , 0_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; format_to_n_187 ( ___out_it , 5_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; format_to_n_188 ( ___out_it , 1000_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; format_to_n_189 ( ___out_it , 10_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args_3: *mut ()  = args_2 ; let ___args_4: *mut ()  = args_3 ; let ___args_5: *mut ()  = args_4 ; let ___args_6: *mut ()  = args_5 ; let ___args_7: *mut ()  = args_6 ; let ___args_8: *mut ()  = args_7 ; let ___args_9: *mut ()  = args_8 ; format_to_n_190 ( ___out_it , - 1_() , ___fmt , ___args_3 , ___args_4 , ___args_5 , ___args_6 , ___args_7 , ___args_8 , ___args_9 , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__wchar_t_ptrref_ , args : *mut *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *mut ()  = args ; format_to_n_191 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *mut ()  = args ; format_to_n_192 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *mut ()  = args ; format_to_n_193 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *mut ()  = args ; format_to_n_194 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *mut ()  = args ; format_to_n_195 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__const_wchar_t_ptrref_ , args : *mut *const () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *const ()  = args ; format_to_n_196 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *const ()  = args ; format_to_n_197 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *const ()  = args ; format_to_n_198 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *const ()  = args ; format_to_n_199 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut *const ()  = args ; format_to_n_200 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__wchar_t__ref_arr8arr_ , args : *mut [ () ; 8] , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 8]  = args ; format_to_n_201 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 8]  = args ; format_to_n_202 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 8]  = args ; format_to_n_203 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 8]  = args ; format_to_n_204 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 8]  = args ; format_to_n_205 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__wchar_t__ref_arr7arr_ , args : *mut [ () ; 7] , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 7]  = args ; format_to_n_206 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 7]  = args ; format_to_n_207 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 7]  = args ; format_to_n_208 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 7]  = args ; format_to_n_209 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut [ () ; 7]  = args ; format_to_n_210 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_wstring_ref_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_211 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_212 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_213 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_214 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_215 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_wstring_view_ref_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_216 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_217 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_218 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_219 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_220 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__signed_char_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_221 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_222 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_223 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_224 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_225 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__short_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_226 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_227 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_228 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_229 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_230 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__int_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_231 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_232 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_233 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_234 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_235 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__long_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_236 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_237 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_238 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_239 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___n: ()  = - 1_() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_240 ( ___out_it , ___n , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__long_long_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_241 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_242 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_243 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_244 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_245 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t____int128_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_246 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_247 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_248 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_249 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_250 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__unsigned_char_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_251 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_252 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_253 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_254 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_255 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__unsigned_short_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_256 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_257 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_258 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_259 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_260 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__unsigned_int_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_261 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_262 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_263 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_264 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_265 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__unsigned_long_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_266 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_267 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_268 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_269 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_270 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__unsigned_long_long_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_271 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_272 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_273 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_274 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_275 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__unsigned___int128_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_276 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_277 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_278 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_279 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_280 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__float_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_281 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_282 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_283 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_284 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_285 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__double_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_286 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_287 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_288 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_289 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_290 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__long_double_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_291 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_292 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_293 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_294 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_295 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_nullptr_t_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_296 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_297 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_298 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_299 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_300 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__void_ptr_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_301 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_302 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_303 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_304 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_305 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__const_void_ptr_ , args : *mut () , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_306 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_307 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_308 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_309 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut ()  = args ; format_to_n_310 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__status_ , args : *mut status , | { { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut status  = args ; format_to_n_311 ( ___out_it , 0_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut status  = args ; format_to_n_312 ( ___out_it , 5_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = unimplemented!() ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut status  = args ; format_to_n_313 ( ___out_it , 1000_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { let mut out : () = std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__ :: new_1 ( { 10_() } , { ( ' ' as ()  ) } , None , )  ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: ()  = unimplemented!()  ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut status  = args ; format_to_n_314 ( ___out_it , 10_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 let mut size : () = ( ( * unimplemented!()  )   as () )  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } { const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>"); ;
 ;
 let mut buffer : [ () ; 1] = [ 0_() , ] ;
  ;
 ;
 let mut result : () = ( unsafe { let ___out_it: *mut ()  = buffer .as_mut_ptr() ; let ___fmt: ()  = fmt .clone() ; let ___args: *mut status  = args ; format_to_n_315 ( ___out_it , - 1_() , ___fmt , ___args , ) } )  ;
  ;
 ;
 ;
 ;
 let mut formatted_size : () = unimplemented!()  ;
  ;
 ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 } } )
});
pub static mut test_exception_316: std::cell::LazyCell<FnPtr<Generic>> =
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
        format_tests_7(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_10)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_316)).copy_from(),
        )
    });
    (unsafe {
        format_tests_char_to_wchar_t_9(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_10)).copy_from(),
        )
    });
    (unsafe {
        format_tests_8(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_10)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_316)).copy_from(),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_10);
    std::cell::LazyCell::force(&*&raw const test_exception_316);
}
