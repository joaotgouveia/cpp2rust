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
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foo;
        check
            .spec::<fn((), (), *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foo;
        check
            .spec::<fn((), (), *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foo;
        check
            .spec::<fn((), (), *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foo;
        check
            .spec::<fn((), (), *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_bar;
        check
            .spec::<fn((), (), *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_bar;
        check
            .spec::<fn((), (), *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_bar;
        check
            .spec::<fn((), (), *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_bar;
        check
            .spec::<fn((), (), *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), (), *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), (), *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), (), *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), (), *mut status)>(29)
            .call(_expected, _fmt, &mut _args)
    });
    let mut __tmp_0: () = (unsafe { invalid_types_2(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_0;
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
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foo;
        check
            .spec::<fn((), (), *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foo;
        check
            .spec::<fn((), (), *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foo;
        check
            .spec::<fn((), (), *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foo;
        check
            .spec::<fn((), (), *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_bar;
        check
            .spec::<fn((), (), *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_bar;
        check
            .spec::<fn((), (), *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_bar;
        check
            .spec::<fn((), (), *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_bar;
        check
            .spec::<fn((), (), *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), (), *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), (), *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), (), *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: status = status_foobar;
        check
            .spec::<fn((), (), *mut status)>(60)
            .call(_expected, _fmt, &mut _args)
    });
    let mut __tmp_1: () = (unsafe { invalid_types_3(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_1;
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
        let _fmt: () = unimplemented!();
        check.spec::<fn((), ())>(0).call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), ())>(0).call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), ())>(0).call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = (' ' as ());
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), ())>(0).call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = (' ' as ());
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check.spec::<fn((), (), *mut (), *mut ())>(2).call(
            _expected,
            _fmt,
            &mut _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check.spec::<fn((), (), *mut (), *mut ())>(2).call(
            _expected,
            _fmt,
            &mut _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
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
        check
            .spec::<fn(
                (),
                (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
            )>(3)
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
                &mut _args_9,
                &mut _args_10,
                &mut _args_11,
                &mut _args_12,
                &mut _args_13,
                &mut _args_14,
                &mut _args_15,
                &mut _args_16,
                &mut _args_17,
                &mut _args_18,
                &mut _args_19,
                &mut _args_20,
                &mut _args_21,
            )
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
        let _fmt: () = unimplemented!();
        let mut _args_2: () = ('0' as ());
        let mut _args_3: () = ('9' as ());
        let mut _args_4: () = ('a' as ());
        let mut _args_5: () = ('z' as ());
        let mut _args_6: () = ('A' as ());
        let mut _args_7: () = ('Z' as ());
        let mut _args_8: () = ('!' as ());
        check
            .spec::<fn((), (), *mut (), *mut (), *mut (), *mut (), *mut (), *mut (), *mut ())>(4)
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
            let _fmt: () = unimplemented!();
            check
                .spec::<fn((), (), *mut *mut ())>(5)
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
            let _fmt: () = unimplemented!();
            check
                .spec::<fn((), (), *mut *const ())>(6)
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
            let _fmt: () = unimplemented!();
            check
                .spec::<fn((), (), *mut [(); 8])>(7)
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
            let _fmt: () = unimplemented!();
            check
                .spec::<fn((), (), *mut [(); 7])>(8)
                .call(_expected, _fmt, &mut buffer2)
        });
    }
    {
        let mut data: () = unimplemented!();
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            check
                .spec::<fn((), (), *mut ())>(9)
                .call(_expected, _fmt, &mut data)
        });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut data: () = unimplemented!();
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            check
                .spec::<fn((), (), *mut ())>(10)
                .call(_expected, _fmt, &mut data)
        });
    }
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check.spec::<fn((), (), *mut (), *mut ())>(2).call(
            _expected,
            _fmt,
            &mut _args_2,
            &mut _args_3,
        )
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(13)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(14)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(16)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(17)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(19)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(20)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(22)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(23)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(25)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = Default::default();
        check
            .spec::<fn((), (), *mut ())>(26)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = (66_() as ());
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = (66_() as ());
        check
            .spec::<fn((), (), *mut ())>(28)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe { format_test_handle_4(check.copy_from(), check_exception.copy_from()) });
    {}
}
pub unsafe fn format_tests_7(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), ())>(31).call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), ())>(31).call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), ())>(31).call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = (' ' as ());
        check
            .spec::<fn((), (), *mut ())>(32)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check.spec::<fn((), ())>(31).call(_expected, _fmt)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = (' ' as ());
        check
            .spec::<fn((), (), *mut ())>(32)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check.spec::<fn((), (), *mut (), *mut ())>(33).call(
            _expected,
            _fmt,
            &mut _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check.spec::<fn((), (), *mut (), *mut ())>(33).call(
            _expected,
            _fmt,
            &mut _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
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
        check
            .spec::<fn(
                (),
                (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
                *mut (),
            )>(34)
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
                &mut _args_9,
                &mut _args_10,
                &mut _args_11,
                &mut _args_12,
                &mut _args_13,
                &mut _args_14,
                &mut _args_15,
                &mut _args_16,
                &mut _args_17,
                &mut _args_18,
                &mut _args_19,
                &mut _args_20,
                &mut _args_21,
            )
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
        let _fmt: () = unimplemented!();
        let mut _args_2: () = ('0' as ());
        let mut _args_3: () = ('9' as ());
        let mut _args_4: () = ('a' as ());
        let mut _args_5: () = ('z' as ());
        let mut _args_6: () = ('A' as ());
        let mut _args_7: () = ('Z' as ());
        let mut _args_8: () = ('!' as ());
        check
            .spec::<fn((), (), *mut (), *mut (), *mut (), *mut (), *mut (), *mut (), *mut ())>(35)
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
            let _fmt: () = unimplemented!();
            check
                .spec::<fn((), (), *mut *mut ())>(36)
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
            let _fmt: () = unimplemented!();
            check
                .spec::<fn((), (), *mut *const ())>(37)
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
            let _fmt: () = unimplemented!();
            check
                .spec::<fn((), (), *mut [(); 8])>(38)
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
            let _fmt: () = unimplemented!();
            check
                .spec::<fn((), (), *mut [(); 7])>(39)
                .call(_expected, _fmt, &mut buffer2)
        });
    }
    {
        let mut data: () = unimplemented!();
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            check
                .spec::<fn((), (), *mut ())>(40)
                .call(_expected, _fmt, &mut data)
        });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut data: () = unimplemented!();
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            check
                .spec::<fn((), (), *mut ())>(41)
                .call(_expected, _fmt, &mut data)
        });
    }
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_2: () = false;
        let mut _args_3: () = true;
        check.spec::<fn((), (), *mut (), *mut ())>(33).call(
            _expected,
            _fmt,
            &mut _args_2,
            &mut _args_3,
        )
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(43)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(44)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(46)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(47)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(48)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(50)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(51)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(52)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(54)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(55)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = 42_();
        check
            .spec::<fn((), (), *mut ())>(56)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = Default::default();
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = (66_() as ());
        check
            .spec::<fn((), (), *mut ())>(58)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = (66_() as ());
        check
            .spec::<fn((), (), *mut ())>(59)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe { format_test_handle_5(check.copy_from(), check_exception.copy_from()) });
    {}
}
pub unsafe fn format_tests_char_to_wchar_t_8(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_2: () = ('0' as ());
        let mut _args_3: () = ('9' as ());
        let mut _args_4: () = ('a' as ());
        let mut _args_5: () = ('z' as ());
        let mut _args_6: () = ('A' as ());
        check
            .spec::<fn((), (), *mut (), *mut (), *mut (), *mut (), *mut ())>(30)
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
pub static mut test_9: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(|| unsafe {
    lambda_unsafe!(
        Generic,
        {},
        |mut expected: (), mut fmt: ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (),
         mut fmt: (),
         args_2: *mut (),
         args_3: *mut (),
         args_4: *mut (),
         args_5: *mut (),
         args_6: *mut (),
         args_7: *mut (),
         args_8: *mut (),
         args_9: *mut (),
         args_10: *mut (),
         args_11: *mut (),
         args_12: *mut (),
         args_13: *mut (),
         args_14: *mut (),
         args_15: *mut (),
         args_16: *mut (),
         args_17: *mut (),
         args_18: *mut (),
         args_19: *mut (),
         args_20: *mut (),
         args_21: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (),
         mut fmt: (),
         args_2: *mut (),
         args_3: *mut (),
         args_4: *mut (),
         args_5: *mut (),
         args_6: *mut (),
         args_7: *mut (),
         args_8: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut *const ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut [(); 8]| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut [(); 7]| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut status| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (),
         mut fmt: (),
         args_2: *mut (),
         args_3: *mut (),
         args_4: *mut (),
         args_5: *mut (),
         args_6: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (),
         mut fmt: (),
         args_2: *mut (),
         args_3: *mut (),
         args_4: *mut (),
         args_5: *mut (),
         args_6: *mut (),
         args_7: *mut (),
         args_8: *mut (),
         args_9: *mut (),
         args_10: *mut (),
         args_11: *mut (),
         args_12: *mut (),
         args_13: *mut (),
         args_14: *mut (),
         args_15: *mut (),
         args_16: *mut (),
         args_17: *mut (),
         args_18: *mut (),
         args_19: *mut (),
         args_20: *mut (),
         args_21: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (),
         mut fmt: (),
         args_2: *mut (),
         args_3: *mut (),
         args_4: *mut (),
         args_5: *mut (),
         args_6: *mut (),
         args_7: *mut (),
         args_8: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut *const ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut [(); 8]| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut [(); 7]| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        },
        |mut expected: (), mut fmt: (), args: *mut status| {
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
            }
            {
                let mut out: () = unimplemented!();
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                let mut size: () = (*unimplemented!());
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                const _: () = assert!(true, "std::is_signed_v<std::iter_difference_t<CharT*>>");;
                let mut buffer: [(); 1] = [0_()];
                let mut result: () = unimplemented!();
                let mut formatted_size: () = unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
    )
});
pub static mut test_exception_10: std::cell::LazyCell<FnPtr<Generic>> =
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
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_9)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_10)).copy_from(),
        )
    });
    (unsafe {
        format_tests_char_to_wchar_t_8(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_9)).copy_from(),
        )
    });
    (unsafe {
        format_tests_7(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_9)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_10)).copy_from(),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_9);
    std::cell::LazyCell::force(&*&raw const test_exception_10);
}
