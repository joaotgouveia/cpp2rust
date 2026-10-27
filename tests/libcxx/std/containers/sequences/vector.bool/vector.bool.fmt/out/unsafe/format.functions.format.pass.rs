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
pub unsafe fn get_colons_1() -> () {
    return unimplemented!();
}
pub unsafe fn get_colons_3() -> () {
    return unimplemented!();
}
pub unsafe fn get_colons_5() -> () {
    return unimplemented!();
}
pub unsafe fn get_colons_7() -> () {
    return unimplemented!();
}
pub unsafe fn get_format_types_9() -> () {
    return unimplemented!();
}
pub unsafe fn fmt_invalid_types_10(mut valid: ()) -> () {
    let mut result: () = unimplemented!();
    (unsafe {
        let mut ___r: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_11))(&mut ___r, unimplemented!())
    });
    return unimplemented!();
}
pub unsafe fn fmt_invalid_types_12(mut valid: ()) -> () {
    let mut result: () = unimplemented!();
    (unsafe {
        let mut ___r: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_11))(&mut ___r, unimplemented!())
    });
    return unimplemented!();
}
pub unsafe fn fmt_invalid_types_13(mut valid: ()) -> () {
    let mut result: () = unimplemented!();
    (unsafe {
        let mut ___r: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_11))(&mut ___r, unimplemented!())
    });
    return unimplemented!();
}
pub unsafe fn fmt_invalid_types_14(mut valid: ()) -> () {
    let mut result: () = unimplemented!();
    (unsafe {
        let mut ___r: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_11))(&mut ___r, unimplemented!())
    });
    return unimplemented!();
}
static mut result_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut result_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut result_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut result_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn fmt_invalid_types_15(mut valid: ()) -> () {
    return (unsafe { fmt_invalid_types_10(unimplemented!()) });
}
pub unsafe fn fmt_invalid_types_16(mut valid: ()) -> () {
    return (unsafe { fmt_invalid_types_13(unimplemented!()) });
}
pub unsafe fn fmt_invalid_nested_types_17(mut valid: ()) -> () {
    return (unsafe { fmt_invalid_types_12(unimplemented!()) });
}
pub unsafe fn fmt_invalid_nested_types_18(mut valid: ()) -> () {
    return (unsafe { fmt_invalid_types_14(unimplemented!()) });
}
pub unsafe fn format_test_vector_bool_19(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_0: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_0;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = unimplemented!();
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_1: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_1;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = unimplemented!();
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        let mut _args_4: () = 7_();
        check.spec::<fn((), (), *mut (), *mut (), *mut ())>(2).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
            &mut _args_4,
        )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        let mut _arg3: () = 32_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn format_test_vector_bool_20(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *const (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *const (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *const (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *const (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *const (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_2: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_2;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = unimplemented!();
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *const () = input;
            check_exception
                .spec::<fn((), (), *const ())>(2)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *const (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *const (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *const (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *const (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_3: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_3;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = unimplemented!();
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *const () = input;
            check_exception
                .spec::<fn((), (), *const ())>(2)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 32_();
        check
            .spec::<fn((), (), *const (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 32_();
        let mut _args_4: () = 7_();
        check
            .spec::<fn((), (), *const (), *mut (), *mut ())>(5)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        let mut _arg3: () = 32_();
        check_exception
            .spec::<fn((), (), *const (), *mut ())>(3)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn format_test_vector_bool_21(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_4: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_4;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = unimplemented!();
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(4)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_5: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_5;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = unimplemented!();
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(4)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        let mut _args_4: () = 7_();
        check.spec::<fn((), (), *mut (), *mut (), *mut ())>(8).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
            &mut _args_4,
        )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        let mut _arg3: () = 32_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(5)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn format_test_vector_bool_22(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *const (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 24_();
        check.spec::<fn((), (), *const (), *mut ())>(10).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 24_();
        check.spec::<fn((), (), *const (), *mut ())>(10).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 24_();
        check.spec::<fn((), (), *const (), *mut ())>(10).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 24_();
        check.spec::<fn((), (), *const (), *mut ())>(10).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_6: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_6;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = unimplemented!();
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *const () = input;
            check_exception
                .spec::<fn((), (), *const ())>(6)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 7_();
        check.spec::<fn((), (), *const (), *mut ())>(10).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 7_();
        check.spec::<fn((), (), *const (), *mut ())>(10).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 7_();
        check.spec::<fn((), (), *const (), *mut ())>(10).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 7_();
        check.spec::<fn((), (), *const (), *mut ())>(10).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_7: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_7;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = unimplemented!();
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *const () = input;
            check_exception
                .spec::<fn((), (), *const ())>(6)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *const () = input;
        check
            .spec::<fn((), (), *const ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 32_();
        check.spec::<fn((), (), *const (), *mut ())>(10).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *const () = input;
        let mut _args_3: () = 32_();
        let mut _args_4: () = 7_();
        check
            .spec::<fn((), (), *const (), *mut (), *mut ())>(11)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        check_exception
            .spec::<fn((), (), *const ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *const () = input;
        let mut _arg3: () = 32_();
        check_exception
            .spec::<fn((), (), *const (), *mut ())>(7)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn format_tests_23(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let mut _input: () = unimplemented!();
        format_test_vector_bool_19(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    let input: () = unimplemented!();
    (unsafe { format_test_vector_bool_20(check.copy_from(), check_exception.copy_from(), &input) });
}
pub unsafe fn format_tests_24(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let mut _input: () = unimplemented!();
        format_test_vector_bool_21(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    let input: () = unimplemented!();
    (unsafe { format_test_vector_bool_22(check.copy_from(), check_exception.copy_from(), &input) });
}
pub unsafe fn test_eprintf_25(
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
pub unsafe fn test_eprintf_26(
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
pub unsafe fn test_eprintf_27(mut fmt: *const (), args: *const *const ()) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args),
        )
    });
}
pub unsafe fn test_log_28(
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
        test_eprintf_25(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
pub unsafe fn test_log_29(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_26(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
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
        test_log_29(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_is_high_surrogate_31(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (56319_()));
}
pub unsafe fn test_is_low_surrogate_32(mut value: ()) -> () {
    return ((value) >= (56320_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_surrogate_33(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_code_point_34(mut value: ()) -> () {
    return ((value) <= (1114111_()));
}
pub unsafe fn test_is_scalar_value_35(mut value: ()) -> () {
    return (unsafe { test_is_code_point_34(value) })
        && (!(unsafe { test_is_surrogate_33(value) }));
}
pub static mut test_replacement_character_36: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (65533 as ()) });
pub unsafe fn operator_shl_37(os: *mut (), loc: *const ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_message_38(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_39(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_40(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_41(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_42(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_43(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_44(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_45(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_46(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_47(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_48(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_49(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub static mut test_50: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(|| unsafe {
    lambda_unsafe!(
        Generic,
        {},
        |mut expected: (), mut fmt: (), args: *mut ()| {
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
                            test_eprintf_27(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_30(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    41_(),
                    &mut _arg,
                )
            });
        },
        |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
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
                            test_eprintf_27(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_51(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    41_(),
                    &mut _arg,
                )
            });
        },
        |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
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
                            test_eprintf_27(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_52(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    41_(),
                    &mut _arg,
                )
            });
        },
        |mut expected: (), mut fmt: (), args: *const ()| {
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
                            test_eprintf_27(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_53(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    41_(),
                    &mut _arg,
                )
            });
        },
        |mut expected: (), mut fmt: (), args_2: *const (), args_3: *mut ()| {
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
                            test_eprintf_27(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_54(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    41_(),
                    &mut _arg,
                )
            });
        },
        |mut expected: (), mut fmt: (), args_2: *const (), args_3: *mut (), args_4: *mut ()| {
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
                            test_eprintf_27(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_55(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    41_(),
                    &mut _arg,
                )
            });
        },
        |mut expected: (), mut fmt: (), args: *mut ()| {
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
                            test_eprintf_27(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_56(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    41_(),
                    &mut _arg,
                )
            });
        },
        |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
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
                            test_eprintf_27(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_57(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    41_(),
                    &mut _arg,
                )
            });
        },
        |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
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
                            test_eprintf_27(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_58(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    41_(),
                    &mut _arg,
                )
            });
        },
        |mut expected: (), mut fmt: (), args: *const ()| {
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
                            test_eprintf_27(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_59(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    41_(),
                    &mut _arg,
                )
            });
        },
        |mut expected: (), mut fmt: (), args_2: *const (), args_3: *mut ()| {
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
                            test_eprintf_27(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_60(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    41_(),
                    &mut _arg,
                )
            });
        },
        |mut expected: (), mut fmt: (), args_2: *const (), args_3: *mut (), args_4: *mut ()| {
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
                            test_eprintf_27(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_61(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    41_(),
                    &mut _arg,
                )
            });
        }
    )
});
pub static mut test_exception_62: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *const ()| {},
            |mut _a0: (), mut _a1: (), _a2: *const (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *const ()| {},
            |mut _a0: (), mut _a1: (), _a2: *const (), _a3: *mut ()| {}
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
        format_tests_23(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_50)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_62)).copy_from(),
        )
    });
    (unsafe {
        format_tests_24(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_50)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_62)).copy_from(),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_36);
    std::cell::LazyCell::force(&*&raw const test_50);
    std::cell::LazyCell::force(&*&raw const test_exception_62);
}
