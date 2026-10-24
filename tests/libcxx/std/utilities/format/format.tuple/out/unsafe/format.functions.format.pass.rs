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
pub type color = ();
pub const color_black: color = 0;
pub const color_red: color = 1;
pub const color_gold: color = 2;
pub unsafe fn test_tuple_or_pair_int_int_2(
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
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
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
    let mut __tmp_0: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_0;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_int_3(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(17)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(17)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(17)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(17)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(5)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_1: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_1;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(5)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_int_4(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(14)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_2: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_2;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(14)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_int_5(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(53)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(53)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(53)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(53)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_3: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_3;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(19)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_string_6(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(9)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(9)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(9)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(9)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(1)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(1)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(1)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(1)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(1)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(1)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(1)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(1)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(1)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_4: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_4;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(1)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_string_7(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(11)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(11)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(11)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(11)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(10)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_5: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_5;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(2)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_string_8(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(3)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_6: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_6;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(3)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_string_9(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_7: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_7;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(6)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_string_10(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(20)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(20)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(20)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(20)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(20)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(20)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(20)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(21)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(21)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(21)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(21)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(7)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(20)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(20)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_8: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_8;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(7)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_string_11(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(22)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(22)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(22)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(22)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(22)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(22)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(22)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(23)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(23)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(23)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(23)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(22)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(22)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_9: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_9;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(8)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_string_12(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(44)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(44)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(44)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(44)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(44)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(44)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(44)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(45)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(45)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(45)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(45)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(44)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(44)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_10: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_10;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(15)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_string_13(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(46)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(46)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(46)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(46)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(46)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(46)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(46)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(47)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(47)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(47)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(47)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(16)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(46)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(46)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_11: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_11;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(16)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_string_14(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(48)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(48)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(48)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(48)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(48)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(48)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(48)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(49)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(49)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(49)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(49)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(48)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(48)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_12: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_12;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(17)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_string_15(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(54)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(54)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(54)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(54)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(54)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(54)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(54)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(20)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(54)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(54)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_13: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_13;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(20)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_string_16(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(56)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(56)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(56)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(56)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(56)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(56)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(56)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(57)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(57)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(57)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(57)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(56)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(56)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_14: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_14;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(21)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_tuple_or_pair_int_string_17(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(59)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(59)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(59)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(59)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(22)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(22)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(22)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(22)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(22)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(22)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(22)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(22)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(22)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(58)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_15: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_15;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(22)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_escaping_18(mut check: FnPtr<Generic>, input: *mut ()) {
    const _: () = assert!(
        true,
        "std::same_as<std::remove_cvref_t<decltype(std::get<0>(input))>, CharT>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<std::remove_cvref_t<decltype(std::get<1>(input))>, std::basic_string<CharT>>"
    );;
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(34)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_19(input) })) = ('\t' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(34)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_19(input) })) = ('\n' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(34)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_19(input) })) = ('\0' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(34)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_19(input) })) = ('*' as ());
    unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(34)
            .call(_expected, _fmt, _args)
    });
}
pub unsafe fn test_escaping_20(mut check: FnPtr<Generic>, input: *mut ()) {
    const _: () = assert!(
        true,
        "std::same_as<std::remove_cvref_t<decltype(std::get<0>(input))>, CharT>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<std::remove_cvref_t<decltype(std::get<1>(input))>, std::basic_string<CharT>>"
    );;
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(35)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_21(input) })) = ('\t' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(35)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_21(input) })) = ('\n' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(35)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_21(input) })) = ('\0' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(35)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_21(input) })) = ('*' as ());
    unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(35)
            .call(_expected, _fmt, _args)
    });
}
pub unsafe fn test_escaping_22(mut check: FnPtr<Generic>, input: *mut ()) {
    const _: () = assert!(
        true,
        "std::same_as<std::remove_cvref_t<decltype(std::get<0>(input))>, CharT>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<std::remove_cvref_t<decltype(std::get<1>(input))>, std::basic_string<CharT>>"
    );;
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(70)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_23(input) })) = ('\t' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(70)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_23(input) })) = ('\n' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(70)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_23(input) })) = ('\0' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(70)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_23(input) })) = ('*' as ());
    unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(70)
            .call(_expected, _fmt, _args)
    });
}
pub unsafe fn test_escaping_24(mut check: FnPtr<Generic>, input: *mut ()) {
    const _: () = assert!(
        true,
        "std::same_as<std::remove_cvref_t<decltype(std::get<0>(input))>, CharT>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<std::remove_cvref_t<decltype(std::get<1>(input))>, std::basic_string<CharT>>"
    );;
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(71)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_25(input) })) = ('\t' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(71)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_25(input) })) = ('\n' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(71)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_25(input) })) = ('\0' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(71)
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_25(input) })) = ('*' as ());
    unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(71)
            .call(_expected, _fmt, _args)
    });
}
pub unsafe fn test_pair_int_int_26(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_int_2(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_pair_int_int_27(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_int_4(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_pair_int_string_28(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_string_6(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_string_7(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_string_8(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_pair_int_string_29(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_string_12(
            check.copy_from(),
            check_exception.copy_from(),
            &mut _input,
        )
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_string_13(
            check.copy_from(),
            check_exception.copy_from(),
            &mut _input,
        )
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_string_14(
            check.copy_from(),
            check_exception.copy_from(),
            &mut _input,
        )
    });
}
pub unsafe fn test_tuple_int_30(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 9_();
        check.spec::<fn((), (), *mut (), *mut ())>(15).call(
            _expected,
            _fmt,
            &mut input,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 9_();
        check.spec::<fn((), (), *mut (), *mut ())>(15).call(
            _expected,
            _fmt,
            &mut input,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 9_();
        check.spec::<fn((), (), *mut (), *mut ())>(15).call(
            _expected,
            _fmt,
            &mut input,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 9_();
        check.spec::<fn((), (), *mut (), *mut ())>(15).call(
            _expected,
            _fmt,
            &mut input,
            &mut _args_3,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(4).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    let mut __tmp_16: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_16;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(4).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
}
pub unsafe fn test_tuple_int_31(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 9_();
        check.spec::<fn((), (), *mut (), *mut ())>(51).call(
            _expected,
            _fmt,
            &mut input,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 9_();
        check.spec::<fn((), (), *mut (), *mut ())>(51).call(
            _expected,
            _fmt,
            &mut input,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 9_();
        check.spec::<fn((), (), *mut (), *mut ())>(51).call(
            _expected,
            _fmt,
            &mut input,
            &mut _args_3,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 9_();
        check.spec::<fn((), (), *mut (), *mut ())>(51).call(
            _expected,
            _fmt,
            &mut input,
            &mut _args_3,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    let mut __tmp_17: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_17;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(18).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
}
pub unsafe fn test_tuple_int_string_color_32(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    let input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *const (), *mut ())>(25)
            .call(_expected, _fmt, &input, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *const (), *mut ())>(25)
            .call(_expected, _fmt, &input, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *const (), *mut ())>(25)
            .call(_expected, _fmt, &input, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *const (), *mut ())>(25)
            .call(_expected, _fmt, &input, &mut _args_3)
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(9).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(9).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(9).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(9).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(9).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(9).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(9).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(9).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(9).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(9).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(24)
            .call(_expected, _fmt, &input)
    });
    let mut __tmp_18: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_18;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            check_exception.spec::<fn((), (), *const ())>(9).call(
                unimplemented!(),
                unimplemented!(),
                &input,
            )
        });
    }
}
pub unsafe fn test_tuple_int_string_color_33(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    let input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *const (), *mut ())>(61)
            .call(_expected, _fmt, &input, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *const (), *mut ())>(61)
            .call(_expected, _fmt, &input, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *const (), *mut ())>(61)
            .call(_expected, _fmt, &input, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *const (), *mut ())>(61)
            .call(_expected, _fmt, &input, &mut _args_3)
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(23).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(23).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(23).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(23).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(23).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(23).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(23).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(23).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(23).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *const ())>(23).call(
            unimplemented!(),
            unimplemented!(),
            &input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        check
            .spec::<fn((), (), *const ())>(60)
            .call(_expected, _fmt, &input)
    });
    let mut __tmp_19: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_19;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            check_exception.spec::<fn((), (), *const ())>(23).call(
                unimplemented!(),
                unimplemented!(),
                &input,
            )
        });
    }
}
pub unsafe fn test_tuple_int_int_34(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_int_3(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_tuple_int_int_35(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_int_5(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_tuple_int_string_36(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_string_9(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_string_10(
            check.copy_from(),
            check_exception.copy_from(),
            &mut _input,
        )
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_string_11(
            check.copy_from(),
            check_exception.copy_from(),
            &mut _input,
        )
    });
}
pub unsafe fn test_tuple_int_string_37(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_string_15(
            check.copy_from(),
            check_exception.copy_from(),
            &mut _input,
        )
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_string_16(
            check.copy_from(),
            check_exception.copy_from(),
            &mut _input,
        )
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_tuple_or_pair_int_string_17(
            check.copy_from(),
            check_exception.copy_from(),
            &mut _input,
        )
    });
}
pub unsafe fn test_nested_38(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(27)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(27)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(27)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(27)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(26)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_20: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_20;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_nested_39(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(29)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(29)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(29)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(29)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(11)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(11)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(11)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(11)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(11)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(11)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(11)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(11)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(11)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(28)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_21: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_21;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(11)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_nested_40(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(12)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(12)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(12)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(12)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(12)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(12)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(12)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(12)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(12)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_22: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_22;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(12)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_nested_41(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(32)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(32)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(32)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(32)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(32)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(32)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(32)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(33)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(33)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(33)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(33)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(13)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(13)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(13)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(13)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(13)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(13)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(13)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(13)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(13)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(32)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(32)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_23: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_23;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(13)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_nested_42(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(62)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(62)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(62)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(62)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(62)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(62)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(62)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(63)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(63)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(63)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(63)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(24)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(24)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(24)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(24)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(24)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(24)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(24)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(24)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(24)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(62)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(62)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_24: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_24;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(24)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_nested_43(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(64)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(64)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(64)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(64)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(64)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(64)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(64)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(65)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(65)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(65)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(65)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(64)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(64)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_25: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_25;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(25)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_nested_44(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(26)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(26)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(26)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(26)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(26)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(26)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(26)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(26)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(26)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_26: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_26;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(26)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn test_nested_45(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(68)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(68)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(68)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(68)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(68)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(68)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(68)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(69)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(69)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(69)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(69)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(68)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(68)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_27: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_27;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(27)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
}
pub unsafe fn run_tests_46(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe { test_pair_int_int_26(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_pair_int_string_28(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_tuple_int_30(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_tuple_int_int_34(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_tuple_int_string_36(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_tuple_int_string_color_32(check.copy_from(), check_exception.copy_from()) });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_nested_38(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_nested_39(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_nested_40(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_nested_41(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_escaping_18(check.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_escaping_20(check.copy_from(), &mut _input)
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
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    let mut answer: () = 42_();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(2)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(4)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(5)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn run_tests_47(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe { test_pair_int_int_27(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_pair_int_string_29(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_tuple_int_31(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_tuple_int_int_35(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_tuple_int_string_37(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_tuple_int_string_color_33(check.copy_from(), check_exception.copy_from()) });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_nested_42(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_nested_43(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_nested_44(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_nested_45(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_escaping_22(check.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_escaping_24(check.copy_from(), &mut _input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(37)
            .call(_expected, _fmt, &mut _args)
    });
    let mut answer: () = 42_();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(38)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(40)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(41)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_eprintf_48(
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
pub unsafe fn test_eprintf_49(
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
pub unsafe fn test_eprintf_50(mut fmt: *const (), args: *const *const ()) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args),
        )
    });
}
pub unsafe fn test_log_51(
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
        test_eprintf_48(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
pub unsafe fn test_log_52(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_49(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_require_53(
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
        test_log_54(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_is_high_surrogate_55(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (56319_()));
}
pub unsafe fn test_is_low_surrogate_56(mut value: ()) -> () {
    return ((value) >= (56320_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_surrogate_57(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_code_point_58(mut value: ()) -> () {
    return ((value) <= (1114111_()));
}
pub unsafe fn test_is_scalar_value_59(mut value: ()) -> () {
    return (unsafe { test_is_code_point_58(value) })
        && (!(unsafe { test_is_surrogate_57(value) }));
}
pub static mut test_replacement_character_60: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (65533 as ()) });
pub unsafe fn test_encode_61(out_it: *mut (), mut value: ()) {
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
pub unsafe fn test_encode_62(out_it: *mut (), mut value: ()) {
    if (((value) & (4294901760_())) == (0_())) {
        (unsafe {
            let _out_it: *mut () = out_it;
            let _value: () = value;
            test_encode_61(_out_it, _value)
        });
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_transcode_63(mut first: (), mut last: (), mut out_it: ()) -> () {
    'loop_: while !(unimplemented!()) {
        let mut value: () = (*unimplemented!());
        if (unsafe { test_is_code_point_58(value) }) {
            (unsafe { test_encode_62(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_61(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_60)),
                )
            });
        }
    }
    return out_it;
}
pub unsafe fn test_transcode_64(mut first: *const (), mut last: *const (), mut out_it: ()) -> () {
    'loop_: while ((first) != (last)) {
        let mut value: () = (*first.postfix_inc());
        if (unsafe { test_is_code_point_58(value) }) {
            (unsafe { test_encode_62(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_61(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_60)),
                )
            });
        }
    }
    return out_it;
}
pub unsafe fn operator_shl_65(os: *mut (), loc: *const ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_66(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_67(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_68(stream: *mut (), value: *const [(); 18]) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_69(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_70(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_71(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if !(unimplemented!()) {
        {}
        (unsafe { test_transcode_63(unimplemented!(), unimplemented!(), unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_72(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ((b) != (e)) {
        {}
        (unsafe { test_transcode_64(b, e, unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_73(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ((b) != (e)) {
        {}
        (unsafe { test_transcode_64(b, e, unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_message_74(
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
                test_concat_68(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_70(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_68(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_69(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_68(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_67(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
                                    test_concat_66(_stream, _value)
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
pub unsafe fn test_concat_message_75(
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
                test_concat_68(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_73(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_68(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_72(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_68(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_71(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
                                    test_concat_66(_stream, _value)
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
pub static mut test_76: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(|| unsafe {
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_53(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_77(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_78(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_79(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_80(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_81(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_82(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_83(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_84(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_85(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_86(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_87(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_88(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_89(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_90(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_91(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_92(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_93(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_94(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_95(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_96(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_97(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_98(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_99(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_100(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_101(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_102(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_103(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_104(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_105(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_106(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_107(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_108(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_109(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_110(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_111(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_112(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_113(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_114(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_115(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_116(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_117(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_118(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_119(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_120(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_121(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_122(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_123(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_124(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_125(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_126(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_127(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_128(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_129(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_130(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_131(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_132(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_133(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_134(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_135(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_136(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_137(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_138(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_139(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_140(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_141(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_142(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_143(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_144(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_145(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_146(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
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
                            test_eprintf_50(c"%s".as_ptr(), &mut _args)
                        });
                    }
                );
                test_require_147(
                    _condition,
                    c"out == expected".as_ptr(),
                    c"format.functions.format.pass.cpp".as_ptr(),
                    42_(),
                    &mut _arg,
                )
            });
        }
    )
});
pub static mut test_exception_148: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *const ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *const ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {}
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
        run_tests_46(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_76)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_148)).copy_from(),
        )
    });
    (unsafe {
        run_tests_47(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_76)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_148)).copy_from(),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_60);
    std::cell::LazyCell::force(&*&raw const test_76);
    std::cell::LazyCell::force(&*&raw const test_exception_148);
}
