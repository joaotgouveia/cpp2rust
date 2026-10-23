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
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref_ = {
            let mut __tmp_0: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref_::new({ &mut __tmp_0 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__int__ref_, *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref_ = {
            let mut __tmp_1: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref_::new({ &mut __tmp_1 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__int__ref_, *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref_ = {
            let mut __tmp_2: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref_::new({ &mut __tmp_2 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__int__ref_, *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref_ = {
            let mut __tmp_3: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref_::new({ &mut __tmp_3 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__int__ref_, *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref_ = {
            let mut __tmp_4: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref_::new({ &mut __tmp_4 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__int__ref_, *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref_ = {
            let mut __tmp_5: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref_::new({ &mut __tmp_5 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__int__ref_, *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref_ = {
            let mut __tmp_6: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref_::new({ &mut __tmp_6 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__int__ref_, *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref__int_ = {
            let mut __tmp_7: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref__int_::new({ &mut __tmp_7 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__int__ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref__int_ = {
            let mut __tmp_8: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref__int_::new({ &mut __tmp_8 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__int__ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref__int_ = {
            let mut __tmp_9: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref__int_::new({ &mut __tmp_9 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__int__ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref__int_ = {
            let mut __tmp_10: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref__int_::new({ &mut __tmp_10 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__int__ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref_ = {
            let mut __tmp_11: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref_::new({ &mut __tmp_11 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__int__ref_, *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__int__ref_ = {
            let mut __tmp_12: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__int__ref_::new({ &mut __tmp_12 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__int__ref_, *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_13: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_13;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref_ = {
            let mut __tmp_14: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref_::new({ &mut __tmp_14 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__int__ref_, *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref_ = {
            let mut __tmp_15: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref_::new({ &mut __tmp_15 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__int__ref_, *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref_ = {
            let mut __tmp_16: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref_::new({ &mut __tmp_16 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__int__ref_, *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref_ = {
            let mut __tmp_17: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref_::new({ &mut __tmp_17 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__int__ref_, *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref_ = {
            let mut __tmp_18: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref_::new({ &mut __tmp_18 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__int__ref_, *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref_ = {
            let mut __tmp_19: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref_::new({ &mut __tmp_19 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__int__ref_, *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref_ = {
            let mut __tmp_20: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref_::new({ &mut __tmp_20 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__int__ref_, *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref__int_ = {
            let mut __tmp_21: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref__int_::new({ &mut __tmp_21 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__int__ref__int_ ,*mut () ,*mut () ,)>(17) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref__int_ = {
            let mut __tmp_22: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref__int_::new({ &mut __tmp_22 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__int__ref__int_ ,*mut () ,*mut () ,)>(17) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref__int_ = {
            let mut __tmp_23: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref__int_::new({ &mut __tmp_23 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__int__ref__int_ ,*mut () ,*mut () ,)>(17) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref__int_ = {
            let mut __tmp_24: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref__int_::new({ &mut __tmp_24 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__int__ref__int_ ,*mut () ,*mut () ,)>(17) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref_ = {
            let mut __tmp_25: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref_::new({ &mut __tmp_25 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__int__ref_, *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__int__ref_ = {
            let mut __tmp_26: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__int__ref_::new({ &mut __tmp_26 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__int__ref_, *mut ())>(16)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_27: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_27;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref_ = {
            let mut __tmp_28: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref_::new({ &mut __tmp_28 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_pair_int__int__ref_, *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref_ = {
            let mut __tmp_29: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref_::new({ &mut __tmp_29 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_pair_int__int__ref_, *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref_ = {
            let mut __tmp_30: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref_::new({ &mut __tmp_30 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_pair_int__int__ref_, *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref_ = {
            let mut __tmp_31: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref_::new({ &mut __tmp_31 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_pair_int__int__ref_, *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref_ = {
            let mut __tmp_32: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref_::new({ &mut __tmp_32 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_pair_int__int__ref_, *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref_ = {
            let mut __tmp_33: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref_::new({ &mut __tmp_33 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_pair_int__int__ref_, *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref_ = {
            let mut __tmp_34: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref_::new({ &mut __tmp_34 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_pair_int__int__ref_, *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref__int_ = {
            let mut __tmp_35: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref__int_::new({ &mut __tmp_35 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__int__ref__int_ ,*mut () ,*mut () ,)>(43) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref__int_ = {
            let mut __tmp_36: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref__int_::new({ &mut __tmp_36 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__int__ref__int_ ,*mut () ,*mut () ,)>(43) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref__int_ = {
            let mut __tmp_37: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref__int_::new({ &mut __tmp_37 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__int__ref__int_ ,*mut () ,*mut () ,)>(43) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref__int_ = {
            let mut __tmp_38: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref__int_::new({ &mut __tmp_38 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__int__ref__int_ ,*mut () ,*mut () ,)>(43) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref_ = {
            let mut __tmp_39: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref_::new({ &mut __tmp_39 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_pair_int__int__ref_, *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__int__ref_ = {
            let mut __tmp_40: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__int__ref_::new({ &mut __tmp_40 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_pair_int__int__ref_, *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_41: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_41;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref_ = {
            let mut __tmp_42: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref_::new({ &mut __tmp_42 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__int__ref_, *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref_ = {
            let mut __tmp_43: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref_::new({ &mut __tmp_43 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__int__ref_, *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref_ = {
            let mut __tmp_44: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref_::new({ &mut __tmp_44 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__int__ref_, *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref_ = {
            let mut __tmp_45: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref_::new({ &mut __tmp_45 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__int__ref_, *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref_ = {
            let mut __tmp_46: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref_::new({ &mut __tmp_46 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__int__ref_, *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref_ = {
            let mut __tmp_47: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref_::new({ &mut __tmp_47 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__int__ref_, *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref_ = {
            let mut __tmp_48: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref_::new({ &mut __tmp_48 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__int__ref_, *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_ = {
            let mut __tmp_49: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_::new({ &mut __tmp_49 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_ ,*mut () ,*mut () ,)>(53) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_ = {
            let mut __tmp_50: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_::new({ &mut __tmp_50 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_ ,*mut () ,*mut () ,)>(53) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_ = {
            let mut __tmp_51: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_::new({ &mut __tmp_51 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_ ,*mut () ,*mut () ,)>(53) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_ = {
            let mut __tmp_52: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_::new({ &mut __tmp_52 })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_ ,*mut () ,*mut () ,)>(53) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref_ = {
            let mut __tmp_53: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref_::new({ &mut __tmp_53 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__int__ref_, *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__int__ref_ = {
            let mut __tmp_54: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__int__ref_::new({ &mut __tmp_54 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__int__ref_, *mut ())>(52)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_55: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_55;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref_ = {
            let mut __tmp_56: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref_::new({
                &mut __tmp_56
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_string_view__ref_ ,*mut () ,)>(8) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref_ = {
            let mut __tmp_57: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref_::new({
                &mut __tmp_57
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_string_view__ref_ ,*mut () ,)>(8) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref_ = {
            let mut __tmp_58: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref_::new({
                &mut __tmp_58
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_string_view__ref_ ,*mut () ,)>(8) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref_ = {
            let mut __tmp_59: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref_::new({
                &mut __tmp_59
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_string_view__ref_ ,*mut () ,)>(8) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref_ = {
            let mut __tmp_60: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref_::new({
                &mut __tmp_60
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_string_view__ref_ ,*mut () ,)>(8) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref_ = {
            let mut __tmp_61: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref_::new({
                &mut __tmp_61
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_string_view__ref_ ,*mut () ,)>(8) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref_ = {
            let mut __tmp_62: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref_::new({
                &mut __tmp_62
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_string_view__ref_ ,*mut () ,)>(8) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref__int_ = {
            let mut __tmp_63: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref__int_::new({
                &mut __tmp_63
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_string_view__ref__int_,
                *mut (),
                *mut (),
            )>(9)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref__int_ = {
            let mut __tmp_64: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref__int_::new({
                &mut __tmp_64
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_string_view__ref__int_,
                *mut (),
                *mut (),
            )>(9)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref__int_ = {
            let mut __tmp_65: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref__int_::new({
                &mut __tmp_65
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_string_view__ref__int_,
                *mut (),
                *mut (),
            )>(9)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref__int_ = {
            let mut __tmp_66: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref__int_::new({
                &mut __tmp_66
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_string_view__ref__int_,
                *mut (),
                *mut (),
            )>(9)
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
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref_ = {
            let mut __tmp_67: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref_::new({
                &mut __tmp_67
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_string_view__ref_ ,*mut () ,)>(8) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string_view__ref_ = {
            let mut __tmp_68: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string_view__ref_::new({
                &mut __tmp_68
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_string_view__ref_ ,*mut () ,)>(8) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_69: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_69;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref_ = {
            let mut __tmp_70: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref_::new({ &mut __tmp_70 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__std_string__ref_, *mut ())>(
                10,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref_ = {
            let mut __tmp_71: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref_::new({ &mut __tmp_71 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__std_string__ref_, *mut ())>(
                10,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref_ = {
            let mut __tmp_72: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref_::new({ &mut __tmp_72 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__std_string__ref_, *mut ())>(
                10,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref_ = {
            let mut __tmp_73: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref_::new({ &mut __tmp_73 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__std_string__ref_, *mut ())>(
                10,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref_ = {
            let mut __tmp_74: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref_::new({ &mut __tmp_74 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__std_string__ref_, *mut ())>(
                10,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref_ = {
            let mut __tmp_75: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref_::new({ &mut __tmp_75 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__std_string__ref_, *mut ())>(
                10,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref_ = {
            let mut __tmp_76: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref_::new({ &mut __tmp_76 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__std_string__ref_, *mut ())>(
                10,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref__int_ = {
            let mut __tmp_77: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref__int_::new({
                &mut __tmp_77
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_string__ref__int_,
                *mut (),
                *mut (),
            )>(11)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref__int_ = {
            let mut __tmp_78: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref__int_::new({
                &mut __tmp_78
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_string__ref__int_,
                *mut (),
                *mut (),
            )>(11)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref__int_ = {
            let mut __tmp_79: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref__int_::new({
                &mut __tmp_79
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_string__ref__int_,
                *mut (),
                *mut (),
            )>(11)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref__int_ = {
            let mut __tmp_80: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref__int_::new({
                &mut __tmp_80
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_string__ref__int_,
                *mut (),
                *mut (),
            )>(11)
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
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref_ = {
            let mut __tmp_81: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref_::new({ &mut __tmp_81 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__std_string__ref_, *mut ())>(
                10,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_string__ref_ = {
            let mut __tmp_82: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__std_string__ref_::new({ &mut __tmp_82 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_int__std_string__ref_, *mut ())>(
                10,
            )
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_83: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_83;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ = {
            let mut __tmp_84: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref_::new({ &mut __tmp_84 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ = {
            let mut __tmp_85: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref_::new({ &mut __tmp_85 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ = {
            let mut __tmp_86: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref_::new({ &mut __tmp_86 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ = {
            let mut __tmp_87: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref_::new({ &mut __tmp_87 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ = {
            let mut __tmp_88: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref_::new({ &mut __tmp_88 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ = {
            let mut __tmp_89: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref_::new({ &mut __tmp_89 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ = {
            let mut __tmp_90: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref_::new({ &mut __tmp_90 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_ = {
            let mut __tmp_91: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_::new({
                &mut __tmp_91
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_ = {
            let mut __tmp_92: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_::new({
                &mut __tmp_92
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_ = {
            let mut __tmp_93: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_::new({
                &mut __tmp_93
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_ = {
            let mut __tmp_94: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_::new({
                &mut __tmp_94
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(13)
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
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ = {
            let mut __tmp_95: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref_::new({ &mut __tmp_95 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ = {
            let mut __tmp_96: () = unimplemented!();
            std_basic_format_string_char__std_pair_int__const_char_ptr__ref_::new({ &mut __tmp_96 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_97: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_97;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref_ = {
            let mut __tmp_98: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref_::new({
                &mut __tmp_98
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_string_view__ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref_ = {
            let mut __tmp_99: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref_::new({
                &mut __tmp_99
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_string_view__ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref_ = {
            let mut __tmp_100: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref_::new({
                &mut __tmp_100
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_string_view__ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref_ = {
            let mut __tmp_101: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref_::new({
                &mut __tmp_101
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_string_view__ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref_ = {
            let mut __tmp_102: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref_::new({
                &mut __tmp_102
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_string_view__ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref_ = {
            let mut __tmp_103: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref_::new({
                &mut __tmp_103
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_string_view__ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref_ = {
            let mut __tmp_104: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref_::new({
                &mut __tmp_104
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_string_view__ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_ = {
            let mut __tmp_105: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_::new({
                &mut __tmp_105
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_,
                *mut (),
                *mut (),
            )>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_ = {
            let mut __tmp_106: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_::new({
                &mut __tmp_106
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_,
                *mut (),
                *mut (),
            )>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_ = {
            let mut __tmp_107: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_::new({
                &mut __tmp_107
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_,
                *mut (),
                *mut (),
            )>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_ = {
            let mut __tmp_108: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_::new({
                &mut __tmp_108
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_,
                *mut (),
                *mut (),
            )>(19)
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
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref_ = {
            let mut __tmp_109: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref_::new({
                &mut __tmp_109
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_string_view__ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string_view__ref_ = {
            let mut __tmp_110: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string_view__ref_::new({
                &mut __tmp_110
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_string_view__ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_111: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_111;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref_ = {
            let mut __tmp_112: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref_::new({ &mut __tmp_112 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__std_string__ref_, *mut ())>(
                20,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref_ = {
            let mut __tmp_113: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref_::new({ &mut __tmp_113 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__std_string__ref_, *mut ())>(
                20,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref_ = {
            let mut __tmp_114: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref_::new({ &mut __tmp_114 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__std_string__ref_, *mut ())>(
                20,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref_ = {
            let mut __tmp_115: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref_::new({ &mut __tmp_115 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__std_string__ref_, *mut ())>(
                20,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref_ = {
            let mut __tmp_116: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref_::new({ &mut __tmp_116 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__std_string__ref_, *mut ())>(
                20,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref_ = {
            let mut __tmp_117: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref_::new({ &mut __tmp_117 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__std_string__ref_, *mut ())>(
                20,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref_ = {
            let mut __tmp_118: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref_::new({ &mut __tmp_118 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__std_string__ref_, *mut ())>(
                20,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref__int_ = {
            let mut __tmp_119: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref__int_::new({
                &mut __tmp_119
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_string__ref__int_,
                *mut (),
                *mut (),
            )>(21)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref__int_ = {
            let mut __tmp_120: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref__int_::new({
                &mut __tmp_120
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_string__ref__int_,
                *mut (),
                *mut (),
            )>(21)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref__int_ = {
            let mut __tmp_121: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref__int_::new({
                &mut __tmp_121
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_string__ref__int_,
                *mut (),
                *mut (),
            )>(21)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref__int_ = {
            let mut __tmp_122: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref__int_::new({
                &mut __tmp_122
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_string__ref__int_,
                *mut (),
                *mut (),
            )>(21)
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
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref_ = {
            let mut __tmp_123: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref_::new({ &mut __tmp_123 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__std_string__ref_, *mut ())>(
                20,
            )
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_string__ref_ = {
            let mut __tmp_124: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__std_string__ref_::new({ &mut __tmp_124 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__std_string__ref_, *mut ())>(
                20,
            )
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_125: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_125;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ = {
            let mut __tmp_126: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_::new({
                &mut __tmp_126
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ ,*mut () ,)>(22) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ = {
            let mut __tmp_127: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_::new({
                &mut __tmp_127
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ ,*mut () ,)>(22) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ = {
            let mut __tmp_128: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_::new({
                &mut __tmp_128
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ ,*mut () ,)>(22) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ = {
            let mut __tmp_129: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_::new({
                &mut __tmp_129
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ ,*mut () ,)>(22) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ = {
            let mut __tmp_130: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_::new({
                &mut __tmp_130
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ ,*mut () ,)>(22) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ = {
            let mut __tmp_131: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_::new({
                &mut __tmp_131
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ ,*mut () ,)>(22) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ = {
            let mut __tmp_132: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_::new({
                &mut __tmp_132
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ ,*mut () ,)>(22) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_ = {
            let mut __tmp_133: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_::new({
                &mut __tmp_133
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(23)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_ = {
            let mut __tmp_134: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_::new({
                &mut __tmp_134
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(23)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_ = {
            let mut __tmp_135: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_::new({
                &mut __tmp_135
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(23)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_ = {
            let mut __tmp_136: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_::new({
                &mut __tmp_136
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(23)
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
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ = {
            let mut __tmp_137: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_::new({
                &mut __tmp_137
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ ,*mut () ,)>(22) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ = {
            let mut __tmp_138: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_::new({
                &mut __tmp_138
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ ,*mut () ,)>(22) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_139: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_139;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ = {
            let mut __tmp_140: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_::new({
                &mut __tmp_140
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ = {
            let mut __tmp_141: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_::new({
                &mut __tmp_141
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ = {
            let mut __tmp_142: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_::new({
                &mut __tmp_142
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ = {
            let mut __tmp_143: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_::new({
                &mut __tmp_143
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ = {
            let mut __tmp_144: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_::new({
                &mut __tmp_144
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ = {
            let mut __tmp_145: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_::new({
                &mut __tmp_145
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ = {
            let mut __tmp_146: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_::new({
                &mut __tmp_146
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_ = {
            let mut __tmp_147: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_::new({
                &mut __tmp_147
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_,
                *mut (),
                *mut (),
            )>(45)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_ = {
            let mut __tmp_148: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_::new({
                &mut __tmp_148
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_,
                *mut (),
                *mut (),
            )>(45)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_ = {
            let mut __tmp_149: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_::new({
                &mut __tmp_149
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_,
                *mut (),
                *mut (),
            )>(45)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_ = {
            let mut __tmp_150: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_::new({
                &mut __tmp_150
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_,
                *mut (),
                *mut (),
            )>(45)
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ = {
            let mut __tmp_151: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_::new({
                &mut __tmp_151
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ = {
            let mut __tmp_152: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_::new({
                &mut __tmp_152
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_153: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_153;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ = {
            let mut __tmp_154: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_::new({
                &mut __tmp_154
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ ,*mut () ,)>(46) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ = {
            let mut __tmp_155: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_::new({
                &mut __tmp_155
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ ,*mut () ,)>(46) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ = {
            let mut __tmp_156: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_::new({
                &mut __tmp_156
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ ,*mut () ,)>(46) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ = {
            let mut __tmp_157: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_::new({
                &mut __tmp_157
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ ,*mut () ,)>(46) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ = {
            let mut __tmp_158: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_::new({
                &mut __tmp_158
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ ,*mut () ,)>(46) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ = {
            let mut __tmp_159: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_::new({
                &mut __tmp_159
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ ,*mut () ,)>(46) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ = {
            let mut __tmp_160: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_::new({
                &mut __tmp_160
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ ,*mut () ,)>(46) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_ = {
            let mut __tmp_161: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_::new({
                &mut __tmp_161
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_,
                *mut (),
                *mut (),
            )>(47)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_ = {
            let mut __tmp_162: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_::new({
                &mut __tmp_162
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_,
                *mut (),
                *mut (),
            )>(47)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_ = {
            let mut __tmp_163: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_::new({
                &mut __tmp_163
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_,
                *mut (),
                *mut (),
            )>(47)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_ = {
            let mut __tmp_164: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_::new({
                &mut __tmp_164
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_,
                *mut (),
                *mut (),
            )>(47)
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ = {
            let mut __tmp_165: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_::new({
                &mut __tmp_165
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ ,*mut () ,)>(46) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ = {
            let mut __tmp_166: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_::new({
                &mut __tmp_166
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ ,*mut () ,)>(46) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_167: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_167;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_168: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_168
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_169: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_169
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_170: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_170
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_171: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_171
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_172: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_172
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_173: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_173
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_174: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_174
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_ = {
            let mut __tmp_175: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_::new({
                &mut __tmp_175
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(49)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_ = {
            let mut __tmp_176: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_::new({
                &mut __tmp_176
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(49)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_ = {
            let mut __tmp_177: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_::new({
                &mut __tmp_177
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(49)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_ = {
            let mut __tmp_178: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_::new({
                &mut __tmp_178
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(49)
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_179: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_179
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(48) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_180: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_180
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(48) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_181: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_181;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ = {
            let mut __tmp_182: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_::new({
                &mut __tmp_182
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ ,*mut () ,)>(54) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ = {
            let mut __tmp_183: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_::new({
                &mut __tmp_183
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ ,*mut () ,)>(54) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ = {
            let mut __tmp_184: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_::new({
                &mut __tmp_184
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ ,*mut () ,)>(54) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ = {
            let mut __tmp_185: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_::new({
                &mut __tmp_185
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ ,*mut () ,)>(54) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ = {
            let mut __tmp_186: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_::new({
                &mut __tmp_186
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ ,*mut () ,)>(54) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ = {
            let mut __tmp_187: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_::new({
                &mut __tmp_187
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ ,*mut () ,)>(54) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ = {
            let mut __tmp_188: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_::new({
                &mut __tmp_188
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ ,*mut () ,)>(54) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_ = {
            let mut __tmp_189: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_::new({
                &mut __tmp_189
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_,
                *mut (),
                *mut (),
            )>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_ = {
            let mut __tmp_190: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_::new({
                &mut __tmp_190
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_,
                *mut (),
                *mut (),
            )>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_ = {
            let mut __tmp_191: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_::new({
                &mut __tmp_191
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_,
                *mut (),
                *mut (),
            )>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_ = {
            let mut __tmp_192: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_::new({
                &mut __tmp_192
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_,
                *mut (),
                *mut (),
            )>(55)
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ = {
            let mut __tmp_193: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_::new({
                &mut __tmp_193
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ ,*mut () ,)>(54) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ = {
            let mut __tmp_194: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_::new({
                &mut __tmp_194
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ ,*mut () ,)>(54) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_195: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_195;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ = {
            let mut __tmp_196: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_::new({
                &mut __tmp_196
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ = {
            let mut __tmp_197: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_::new({
                &mut __tmp_197
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ = {
            let mut __tmp_198: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_::new({
                &mut __tmp_198
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ = {
            let mut __tmp_199: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_::new({
                &mut __tmp_199
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ = {
            let mut __tmp_200: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_::new({
                &mut __tmp_200
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ = {
            let mut __tmp_201: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_::new({
                &mut __tmp_201
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ = {
            let mut __tmp_202: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_::new({
                &mut __tmp_202
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_ = {
            let mut __tmp_203: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_::new({
                &mut __tmp_203
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_,
                *mut (),
                *mut (),
            )>(57)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_ = {
            let mut __tmp_204: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_::new({
                &mut __tmp_204
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_,
                *mut (),
                *mut (),
            )>(57)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_ = {
            let mut __tmp_205: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_::new({
                &mut __tmp_205
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_,
                *mut (),
                *mut (),
            )>(57)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_ = {
            let mut __tmp_206: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_::new({
                &mut __tmp_206
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_,
                *mut (),
                *mut (),
            )>(57)
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ = {
            let mut __tmp_207: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_::new({
                &mut __tmp_207
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ = {
            let mut __tmp_208: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_::new({
                &mut __tmp_208
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_209: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_209;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_210: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_210
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(58) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_211: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_211
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(58) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_212: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_212
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(58) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_213: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_213
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(58) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_214: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_214
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(58) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_215: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_215
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(58) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_216: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_216
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(58) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_ = {
            let mut __tmp_217: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_::new({
                &mut __tmp_217
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(59)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_ = {
            let mut __tmp_218: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_::new({
                &mut __tmp_218
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(59)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_ = {
            let mut __tmp_219: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_::new({
                &mut __tmp_219
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(59)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_ = {
            let mut __tmp_220: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_::new({
                &mut __tmp_220
            })
        };
        let _args_2: *mut () = input;
        let mut _args_3: () = 18_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_,
                *mut (),
                *mut (),
            )>(59)
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_221: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_221
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(58) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ = {
            let mut __tmp_222: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_::new({
                &mut __tmp_222
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ ,*mut () ,)>(58) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_223: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_223;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_char__std_pair_char__std_string__ref_ = {
            let mut __tmp_224: () = unimplemented!();
            std_basic_format_string_char__std_pair_char__std_string__ref_::new({ &mut __tmp_224 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_char__std_string__ref_, *mut ())>(
                34,
            )
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_19(input) })) = ('\t' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_char__std_string__ref_ = {
            let mut __tmp_225: () = unimplemented!();
            std_basic_format_string_char__std_pair_char__std_string__ref_::new({ &mut __tmp_225 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_char__std_string__ref_, *mut ())>(
                34,
            )
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_19(input) })) = ('\n' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_char__std_string__ref_ = {
            let mut __tmp_226: () = unimplemented!();
            std_basic_format_string_char__std_pair_char__std_string__ref_::new({ &mut __tmp_226 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_char__std_string__ref_, *mut ())>(
                34,
            )
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_19(input) })) = ('\0' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_char__std_string__ref_ = {
            let mut __tmp_227: () = unimplemented!();
            std_basic_format_string_char__std_pair_char__std_string__ref_::new({ &mut __tmp_227 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_char__std_string__ref_, *mut ())>(
                34,
            )
            .call(_expected, _fmt, _args)
    });
    (*(unsafe { get_19(input) })) = ('*' as ());
    unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_char__std_string__ref_ = {
            let mut __tmp_228: () = unimplemented!();
            std_basic_format_string_char__std_pair_char__std_string__ref_::new({ &mut __tmp_228 })
        };
        let _args: *mut () = input;
        check
            .spec::<fn((), std_basic_format_string_char__std_pair_char__std_string__ref_, *mut ())>(
                34,
            )
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
        let _fmt: std_basic_format_string_char__std_tuple_char__std_string__ref_ = {
            let mut __tmp_229: () = unimplemented!();
            std_basic_format_string_char__std_tuple_char__std_string__ref_::new({ &mut __tmp_229 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_char__std_string__ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (*(unsafe { get_21(input) })) = ('\t' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_char__std_string__ref_ = {
            let mut __tmp_230: () = unimplemented!();
            std_basic_format_string_char__std_tuple_char__std_string__ref_::new({ &mut __tmp_230 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_char__std_string__ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (*(unsafe { get_21(input) })) = ('\n' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_char__std_string__ref_ = {
            let mut __tmp_231: () = unimplemented!();
            std_basic_format_string_char__std_tuple_char__std_string__ref_::new({ &mut __tmp_231 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_char__std_string__ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (*(unsafe { get_21(input) })) = ('\0' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_char__std_string__ref_ = {
            let mut __tmp_232: () = unimplemented!();
            std_basic_format_string_char__std_tuple_char__std_string__ref_::new({ &mut __tmp_232 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_char__std_string__ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (*(unsafe { get_21(input) })) = ('*' as ());
    unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_char__std_string__ref_ = {
            let mut __tmp_233: () = unimplemented!();
            std_basic_format_string_char__std_tuple_char__std_string__ref_::new({ &mut __tmp_233 })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_char__std_string__ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_ = {
            let mut __tmp_234: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_::new({
                &mut __tmp_234
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_ ,*mut () ,)>(70) .call ( _expected , _fmt , _args , )
    });
    (*(unsafe { get_23(input) })) = ('\t' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_ = {
            let mut __tmp_235: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_::new({
                &mut __tmp_235
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_ ,*mut () ,)>(70) .call ( _expected , _fmt , _args , )
    });
    (*(unsafe { get_23(input) })) = ('\n' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_ = {
            let mut __tmp_236: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_::new({
                &mut __tmp_236
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_ ,*mut () ,)>(70) .call ( _expected , _fmt , _args , )
    });
    (*(unsafe { get_23(input) })) = ('\0' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_ = {
            let mut __tmp_237: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_::new({
                &mut __tmp_237
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_ ,*mut () ,)>(70) .call ( _expected , _fmt , _args , )
    });
    (*(unsafe { get_23(input) })) = ('*' as ());
    unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_ = {
            let mut __tmp_238: () = unimplemented!();
            std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_::new({
                &mut __tmp_238
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_ ,*mut () ,)>(70) .call ( _expected , _fmt , _args , )
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_ = {
            let mut __tmp_239: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_::new({
                &mut __tmp_239
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_ ,*mut () ,)>(71) .call ( _expected , _fmt , _args , )
    });
    (*(unsafe { get_25(input) })) = ('\t' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_ = {
            let mut __tmp_240: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_::new({
                &mut __tmp_240
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_ ,*mut () ,)>(71) .call ( _expected , _fmt , _args , )
    });
    (*(unsafe { get_25(input) })) = ('\n' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_ = {
            let mut __tmp_241: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_::new({
                &mut __tmp_241
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_ ,*mut () ,)>(71) .call ( _expected , _fmt , _args , )
    });
    (*(unsafe { get_25(input) })) = ('\0' as ());
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_ = {
            let mut __tmp_242: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_::new({
                &mut __tmp_242
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_ ,*mut () ,)>(71) .call ( _expected , _fmt , _args , )
    });
    (*(unsafe { get_25(input) })) = ('*' as ());
    unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_ = {
            let mut __tmp_243: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_::new({
                &mut __tmp_243
            })
        };
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_ ,*mut () ,)>(71) .call ( _expected , _fmt , _args , )
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
        let _fmt: std_basic_format_string_char__std_tuple_int__ref_ = {
            let mut __tmp_244: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__ref_::new({ &mut __tmp_244 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__ref_, *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__ref_ = {
            let mut __tmp_245: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__ref_::new({ &mut __tmp_245 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__ref_, *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__ref_ = {
            let mut __tmp_246: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__ref_::new({ &mut __tmp_246 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__ref_, *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__ref_ = {
            let mut __tmp_247: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__ref_::new({ &mut __tmp_247 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__ref_, *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__ref_ = {
            let mut __tmp_248: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__ref_::new({ &mut __tmp_248 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__ref_, *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__ref_ = {
            let mut __tmp_249: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__ref_::new({ &mut __tmp_249 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__ref_, *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__ref_ = {
            let mut __tmp_250: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__ref_::new({ &mut __tmp_250 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__ref_, *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__ref__int_ = {
            let mut __tmp_251: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__ref__int_::new({ &mut __tmp_251 })
        };
        let mut _args_3: () = 9_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__ref__int_ ,*mut () ,*mut () ,)>(15) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__ref__int_ = {
            let mut __tmp_252: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__ref__int_::new({ &mut __tmp_252 })
        };
        let mut _args_3: () = 9_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__ref__int_ ,*mut () ,*mut () ,)>(15) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__ref__int_ = {
            let mut __tmp_253: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__ref__int_::new({ &mut __tmp_253 })
        };
        let mut _args_3: () = 9_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__ref__int_ ,*mut () ,*mut () ,)>(15) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__ref__int_ = {
            let mut __tmp_254: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__ref__int_::new({ &mut __tmp_254 })
        };
        let mut _args_3: () = 9_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__ref__int_ ,*mut () ,*mut () ,)>(15) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_char__std_tuple_int__ref_ = {
            let mut __tmp_255: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__ref_::new({ &mut __tmp_255 })
        };
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__ref_, *mut ())>(14)
            .call(_expected, _fmt, &mut input)
    });
    let mut __tmp_256: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_256;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(4).call(
                unimplemented!(),
                (unsafe { unimplemented!()() }),
                &mut input,
            )
        });
    }
}
pub unsafe fn test_tuple_int_31(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ref_ = {
            let mut __tmp_257: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__ref_::new({ &mut __tmp_257 })
        };
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__ref_, *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ref_ = {
            let mut __tmp_258: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__ref_::new({ &mut __tmp_258 })
        };
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__ref_, *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ref_ = {
            let mut __tmp_259: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__ref_::new({ &mut __tmp_259 })
        };
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__ref_, *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ref_ = {
            let mut __tmp_260: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__ref_::new({ &mut __tmp_260 })
        };
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__ref_, *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ref_ = {
            let mut __tmp_261: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__ref_::new({ &mut __tmp_261 })
        };
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__ref_, *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ref_ = {
            let mut __tmp_262: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__ref_::new({ &mut __tmp_262 })
        };
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__ref_, *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ref_ = {
            let mut __tmp_263: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__ref_::new({ &mut __tmp_263 })
        };
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__ref_, *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ref__int_ = {
            let mut __tmp_264: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__ref__int_::new({ &mut __tmp_264 })
        };
        let mut _args_3: () = 9_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ref__int_ = {
            let mut __tmp_265: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__ref__int_::new({ &mut __tmp_265 })
        };
        let mut _args_3: () = 9_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ref__int_ = {
            let mut __tmp_266: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__ref__int_::new({ &mut __tmp_266 })
        };
        let mut _args_3: () = 9_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ref__int_ = {
            let mut __tmp_267: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__ref__int_::new({ &mut __tmp_267 })
        };
        let mut _args_3: () = 9_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ref_ = {
            let mut __tmp_268: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__ref_::new({ &mut __tmp_268 })
        };
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__ref_, *mut ())>(50)
            .call(_expected, _fmt, &mut input)
    });
    let mut __tmp_269: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_269;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(18).call(
                unimplemented!(),
                (unsafe { unimplemented!()() }),
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
        let _fmt: std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_ = {
            let mut __tmp_270: () = unimplemented!();
            std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_::new({
                &mut __tmp_270
            })
        };
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_,
                *const (),
            )>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_ = {
            let mut __tmp_271: () = unimplemented!();
            std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_::new({
                &mut __tmp_271
            })
        };
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_,
                *const (),
            )>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_ = {
            let mut __tmp_272: () = unimplemented!();
            std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_::new({
                &mut __tmp_272
            })
        };
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_,
                *const (),
            )>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_ = {
            let mut __tmp_273: () = unimplemented!();
            std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_::new({
                &mut __tmp_273
            })
        };
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_,
                *const (),
            )>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_ = {
            let mut __tmp_274: () = unimplemented!();
            std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_::new({
                &mut __tmp_274
            })
        };
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_,
                *const (),
            )>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_ = {
            let mut __tmp_275: () = unimplemented!();
            std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_::new({
                &mut __tmp_275
            })
        };
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_,
                *const (),
            )>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_ = {
            let mut __tmp_276: () = unimplemented!();
            std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_::new({
                &mut __tmp_276
            })
        };
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_,
                *const (),
            )>(24)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_  = { let mut __tmp_277 : ()  = unimplemented!() ; std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_ :: new ( { & mut __tmp_277 } , )  } ;
        let mut _args_3: () = 25_();
        check  .spec::<fn(() ,std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_ ,*const () ,*mut () ,)>(25) .call ( _expected , _fmt , & input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_  = { let mut __tmp_278 : ()  = unimplemented!() ; std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_ :: new ( { & mut __tmp_278 } , )  } ;
        let mut _args_3: () = 25_();
        check  .spec::<fn(() ,std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_ ,*const () ,*mut () ,)>(25) .call ( _expected , _fmt , & input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_  = { let mut __tmp_279 : ()  = unimplemented!() ; std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_ :: new ( { & mut __tmp_279 } , )  } ;
        let mut _args_3: () = 25_();
        check  .spec::<fn(() ,std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_ ,*const () ,*mut () ,)>(25) .call ( _expected , _fmt , & input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_  = { let mut __tmp_280 : ()  = unimplemented!() ; std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_ :: new ( { & mut __tmp_280 } , )  } ;
        let mut _args_3: () = 25_();
        check  .spec::<fn(() ,std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_ ,*const () ,*mut () ,)>(25) .call ( _expected , _fmt , & input , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_ = {
            let mut __tmp_281: () = unimplemented!();
            std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_::new({
                &mut __tmp_281
            })
        };
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_,
                *const (),
            )>(24)
            .call(_expected, _fmt, &input)
    });
    let mut __tmp_282: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_282;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            check_exception.spec::<fn((), (), *const ())>(9).call(
                unimplemented!(),
                (unsafe { unimplemented!()() }),
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
        let _fmt: std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_  = { let mut __tmp_283 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_ :: new ( { & mut __tmp_283 } , )  } ;
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_,
                *const (),
            )>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_  = { let mut __tmp_284 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_ :: new ( { & mut __tmp_284 } , )  } ;
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_,
                *const (),
            )>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_  = { let mut __tmp_285 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_ :: new ( { & mut __tmp_285 } , )  } ;
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_,
                *const (),
            )>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_  = { let mut __tmp_286 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_ :: new ( { & mut __tmp_286 } , )  } ;
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_,
                *const (),
            )>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_  = { let mut __tmp_287 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_ :: new ( { & mut __tmp_287 } , )  } ;
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_,
                *const (),
            )>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_  = { let mut __tmp_288 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_ :: new ( { & mut __tmp_288 } , )  } ;
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_,
                *const (),
            )>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_  = { let mut __tmp_289 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_ :: new ( { & mut __tmp_289 } , )  } ;
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_,
                *const (),
            )>(60)
            .call(_expected, _fmt, &input)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_  = { let mut __tmp_290 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_ :: new ( { & mut __tmp_290 } , )  } ;
        let mut _args_3: () = 25_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_ ,*const () ,*mut () ,)>(61) .call ( _expected , _fmt , & input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_  = { let mut __tmp_291 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_ :: new ( { & mut __tmp_291 } , )  } ;
        let mut _args_3: () = 25_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_ ,*const () ,*mut () ,)>(61) .call ( _expected , _fmt , & input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_  = { let mut __tmp_292 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_ :: new ( { & mut __tmp_292 } , )  } ;
        let mut _args_3: () = 25_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_ ,*const () ,*mut () ,)>(61) .call ( _expected , _fmt , & input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_  = { let mut __tmp_293 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_ :: new ( { & mut __tmp_293 } , )  } ;
        let mut _args_3: () = 25_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_ ,*const () ,*mut () ,)>(61) .call ( _expected , _fmt , & input , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_  = { let mut __tmp_294 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_ :: new ( { & mut __tmp_294 } , )  } ;
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_,
                *const (),
            )>(60)
            .call(_expected, _fmt, &input)
    });
    let mut __tmp_295: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_295;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            check_exception.spec::<fn((), (), *const ())>(23).call(
                unimplemented!(),
                (unsafe { unimplemented!()() }),
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
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_296 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_296 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_297 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_297 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_298 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_298 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_299 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_299 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_300 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_300 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_301 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_301 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_302 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_302 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_  = { let mut __tmp_303 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_ :: new ( { & mut __tmp_303 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(27) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_  = { let mut __tmp_304 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_ :: new ( { & mut __tmp_304 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(27) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_  = { let mut __tmp_305 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_ :: new ( { & mut __tmp_305 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(27) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_  = { let mut __tmp_306 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_ :: new ( { & mut __tmp_306 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(27) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_307 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_307 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(26)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_308 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_308 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(26)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_309: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_309;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_310 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_310 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_311 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_311 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_312 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_312 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_313 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_313 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_314 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_314 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_315 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_315 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_316 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_316 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_  = { let mut __tmp_317 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_ :: new ( { & mut __tmp_317 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(29) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_  = { let mut __tmp_318 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_ :: new ( { & mut __tmp_318 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(29) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_  = { let mut __tmp_319 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_ :: new ( { & mut __tmp_319 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(29) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_  = { let mut __tmp_320 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_ :: new ( { & mut __tmp_320 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(29) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_321 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_321 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_322 : ()  = unimplemented!() ; std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_322 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_,
                *mut (),
            )>(28)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_323: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_323;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_324 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_324 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_325 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_325 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_326 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_326 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_327 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_327 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_328 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_328 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_329 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_329 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_330 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_330 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_  = { let mut __tmp_331 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_ :: new ( { & mut __tmp_331 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(31) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_  = { let mut __tmp_332 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_ :: new ( { & mut __tmp_332 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(31) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_  = { let mut __tmp_333 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_ :: new ( { & mut __tmp_333 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(31) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_  = { let mut __tmp_334 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_ :: new ( { & mut __tmp_334 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(31) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_335 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_335 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_  = { let mut __tmp_336 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_ :: new ( { & mut __tmp_336 } , )  } ;
        let _args: *mut () = input;
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_,
                *mut (),
            )>(30)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_337: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_337;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_338 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_338 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_339 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_339 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_340 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_340 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_341 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_341 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_342 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_342 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_343 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_343 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_344 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_344 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_  = { let mut __tmp_345 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_ :: new ( { & mut __tmp_345 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_  = { let mut __tmp_346 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_ :: new ( { & mut __tmp_346 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_  = { let mut __tmp_347 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_ :: new ( { & mut __tmp_347 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_  = { let mut __tmp_348 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_ :: new ( { & mut __tmp_348 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_349 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_349 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_  = { let mut __tmp_350 : ()  = unimplemented!() ; std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ :: new ( { & mut __tmp_350 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_351: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_351;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_352 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_352 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(62) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_353 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_353 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(62) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_354 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_354 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(62) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_355 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_355 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(62) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_356 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_356 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(62) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_357 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_357 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(62) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_358 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_358 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(62) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_  = { let mut __tmp_359 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_359 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(63) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_  = { let mut __tmp_360 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_360 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(63) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_  = { let mut __tmp_361 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_361 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(63) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_  = { let mut __tmp_362 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_362 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(63) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_363 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_363 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(62) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_364 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_364 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(62) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_365: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_365;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_366 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_366 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(64) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_367 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_367 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(64) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_368 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_368 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(64) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_369 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_369 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(64) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_370 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_370 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(64) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_371 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_371 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(64) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_372 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_372 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(64) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_  = { let mut __tmp_373 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_373 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(65) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_  = { let mut __tmp_374 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_374 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(65) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_  = { let mut __tmp_375 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_375 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(65) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_  = { let mut __tmp_376 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_376 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(65) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_377 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_377 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(64) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_378 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_378 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(64) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_379: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_379;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_380 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_380 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(66) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_381 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_381 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(66) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_382 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_382 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(66) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_383 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_383 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(66) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_384 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_384 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(66) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_385 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_385 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(66) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_386 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_386 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(66) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_  = { let mut __tmp_387 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_387 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(67) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_  = { let mut __tmp_388 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_388 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(67) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_  = { let mut __tmp_389 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_389 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(67) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_  = { let mut __tmp_390 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_390 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(67) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_391 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_391 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(66) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_  = { let mut __tmp_392 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ :: new ( { & mut __tmp_392 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ ,*mut () ,)>(66) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_393: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_393;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_394 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_394 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(68) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_395 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_395 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(68) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_396 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_396 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(68) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_397 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_397 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(68) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_398 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_398 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(68) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_399 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_399 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(68) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_400 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_400 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(68) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_  = { let mut __tmp_401 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_401 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(69) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_  = { let mut __tmp_402 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_402 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(69) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_  = { let mut __tmp_403 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_403 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(69) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_  = { let mut __tmp_404 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_ :: new ( { & mut __tmp_404 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_ ,*mut () ,*mut () ,)>(69) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_405 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_405 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(68) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_  = { let mut __tmp_406 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ :: new ( { & mut __tmp_406 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ ,*mut () ,)>(68) .call ( _expected , _fmt , _args , )
    });
    let mut __tmp_407: () = unimplemented!();
    let __range1: *mut () = &mut __tmp_407;
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*c);
        (unsafe {
            let _arg1: () = (unsafe { unimplemented!()() });
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
        let _fmt: std_basic_format_string_char__std_tuple_int__ = {
            let mut __tmp_408: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int__::new({ &mut __tmp_408 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int__, *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_const_int__ = {
            let mut __tmp_409: () = unimplemented!();
            std_basic_format_string_char__std_tuple_const_int__::new({ &mut __tmp_409 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_const_int__, *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    let mut answer: () = 42_();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int_ref__ = {
            let mut __tmp_410: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int_ref__::new({ &mut __tmp_410 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int_ref__, *mut ())>(2)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_const_int_ref__ = {
            let mut __tmp_411: () = unimplemented!();
            std_basic_format_string_char__std_tuple_const_int_ref__::new({ &mut __tmp_411 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_const_int_ref__, *mut ())>(3)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_int_refref__ = {
            let mut __tmp_412: () = unimplemented!();
            std_basic_format_string_char__std_tuple_int_refref__::new({ &mut __tmp_412 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_int_refref__, *mut ())>(4)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_tuple_const_int_refref__ = {
            let mut __tmp_413: () = unimplemented!();
            std_basic_format_string_char__std_tuple_const_int_refref__::new({ &mut __tmp_413 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_char__std_tuple_const_int_refref__, *mut ())>(5)
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
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int__ = {
            let mut __tmp_414: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int__::new({ &mut __tmp_414 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int__, *mut ())>(36)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_const_int__ = {
            let mut __tmp_415: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_const_int__::new({ &mut __tmp_415 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_const_int__, *mut ())>(37)
            .call(_expected, _fmt, &mut _args)
    });
    let mut answer: () = 42_();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int_ref__ = {
            let mut __tmp_416: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int_ref__::new({ &mut __tmp_416 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int_ref__, *mut ())>(38)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_const_int_ref__ = {
            let mut __tmp_417: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_const_int_ref__::new({ &mut __tmp_417 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_const_int_ref__, *mut ())>(39)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_int_refref__ = {
            let mut __tmp_418: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_int_refref__::new({ &mut __tmp_418 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_int_refref__, *mut ())>(40)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_tuple_const_int_refref__ = {
            let mut __tmp_419: () = unimplemented!();
            std_basic_format_string_wchar_t__std_tuple_const_int_refref__::new({ &mut __tmp_419 })
        };
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), std_basic_format_string_wchar_t__std_tuple_const_int_refref__, *mut ())>(
                41,
            )
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
        (unsafe { test_transcode_63(b.clone(), e.clone(), unimplemented!()) });
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
    lambda_unsafe! ( Generic , { } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_53 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_const_int__ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_const_int__ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_77 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int_ref__ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int_ref__ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_78 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_const_int_ref__ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_const_int_ref__ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_79 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int_refref__ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int_refref__ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_80 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_const_int_refref__ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_const_int_refref__ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_81 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_int__int__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_int__int__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_82 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_int__int__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_int__int__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_83 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_int__std_string_view__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_int__std_string_view__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_84 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_int__std_string_view__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_int__std_string_view__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_85 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_int__std_string__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_int__std_string__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_86 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_int__std_string__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_int__std_string__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_87 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_int__const_char_ptr__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_88 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_int__const_char_ptr__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_89 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_90 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_91 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__int__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__int__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_92 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__int__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__int__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_93 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__std_string_view__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__std_string_view__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_94 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__std_string_view__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_95 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__std_string__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__std_string__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_96 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__std_string__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__std_string__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_97 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__const_char_ptr__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_98 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__const_char_ptr__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_99 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_ , args : *const () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_100 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_ , args_2 : *const () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__const_std_tuple_int__std_string_view__color__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_101 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_102 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_int__std_pair_std_string_view__color___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_103 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_104 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_int__std_tuple_std_string_view__color___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_105 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_106 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__std_pair_std_string_view__color___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_107 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_108 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_int__std_tuple_std_string_view__color___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_109 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_pair_char__std_string__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_pair_char__std_string__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_110 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_tuple_char__std_string__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_tuple_char__std_string__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_111 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_112 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_const_int__ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_const_int__ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_113 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int_ref__ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int_ref__ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_114 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_const_int_ref__ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_const_int_ref__ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_115 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int_refref__ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int_refref__ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_116 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_const_int_refref__ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_const_int_refref__ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_117 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_int__int__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_int__int__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_118 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_int__int__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_int__int__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_119 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_120 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_int__std_wstring_view__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_121 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_122 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_int__std_wstring__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_123 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_124 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_int__const_wchar_t_ptr__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_125 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_126 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_127 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__int__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__int__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_128 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__int__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_129 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_130 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__std_wstring_view__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_131 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_132 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__std_wstring__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_133 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_134 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__const_wchar_t_ptr__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_135 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_ , args : *const () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_136 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_ , args_2 : *const () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__const_std_tuple_int__std_wstring_view__color__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_137 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_138 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_int__std_pair_std_wstring_view__color___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_139 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_140 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_int__std_tuple_std_wstring_view__color___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_141 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_142 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__std_pair_std_wstring_view__color___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_143 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_144 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_int__std_tuple_std_wstring_view__color___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_145 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_pair_wchar_t__std_wstring__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_146 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_tuple_wchar_t__std_wstring__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_50 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_147 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } )
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
